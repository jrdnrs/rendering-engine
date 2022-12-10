use std::{
    fs,
    io::{self, BufReader, ErrorKind, Read},
};

use image::{self, DynamicImage::*};

#[cfg(feature = "opengl")]
pub use super::opengl::texture::{ImageFormat};
use super::{texture::Image, DataType};

fn as_u32_le(array: &[u8; 4]) -> u32 {
    ((array[0] as u32) << 0)
        | ((array[1] as u32) << 8)
        | ((array[2] as u32) << 16)
        | ((array[3] as u32) << 24)
}

fn read_u32_le<R: Read>(r: &mut BufReader<R>) -> Result<u32, io::Error> {
    let mut buf = [0; 4];
    r.read_exact(&mut buf)?;
    Ok(as_u32_le(&buf))
}

pub enum DxgiFormat {
    Bc6hTypeless = 94,
    Bc6hUnsignedFloat16,
    Bc6hSignedFloat16,

    Bc7Typeless,
    Bc7UnsignedNormalised,
    Bc7UnsignedNormalisedSrgb,
}

#[derive(Debug)]
pub struct Header {
    pub flags: u32,
    pub height: u32,
    pub width: u32,
    pub pitch_or_linear_size: u32,
    pub depth: u32,
    pub mipmap_count: u32,
    pub pixel_format: PixelFormat,
    pub caps: u32,
    pub caps2: u32,
}

impl Header {
    pub fn from_reader<R: Read>(r: &mut BufReader<R>) -> Result<Self, io::Error> {
        let size = read_u32_le(r)?;
        let flags = read_u32_le(r)?;
        let height = read_u32_le(r)?;
        let width = read_u32_le(r)?;
        let pitch_or_linear_size = read_u32_le(r)?;
        let depth = read_u32_le(r)?;
        let mipmap_count = read_u32_le(r)?;
        // Skip `dwReserved1`
        {
            let mut skipped = [0; 4 * 11];
            r.read_exact(&mut skipped)?;
        }
        let pixel_format = PixelFormat::from_reader(r)?;
        let caps = read_u32_le(r)?;
        let caps2 = read_u32_le(r)?;
        // Skip `dwCaps3`, `dwCaps4`, `dwReserved2` (unused)
        {
            let mut skipped = [0; 4 + 4 + 4];
            r.read_exact(&mut skipped)?;
        }

        Ok(Self {
            flags,
            height,
            width,
            pitch_or_linear_size,
            depth,
            mipmap_count,
            pixel_format,
            caps,
            caps2,
        })
    }
}

#[derive(Debug)]
pub struct DX10Header {
    pub dxgi_format: u32,
    pub resource_dimension: u32,
    pub misc_flag: u32,
    pub array_size: u32,
    pub misc_flags_2: u32,
}

impl DX10Header {
    pub fn from_reader<R: Read>(r: &mut BufReader<R>) -> Result<Self, io::Error> {
        let dxgi_format = read_u32_le(r)?;
        let resource_dimension = read_u32_le(r)?;
        let misc_flag = read_u32_le(r)?;
        let array_size = read_u32_le(r)?;
        let misc_flags_2 = read_u32_le(r)?;

        let dx10_header = Self {
            dxgi_format,
            resource_dimension,
            misc_flag,
            array_size,
            misc_flags_2,
        };
        Ok(dx10_header)
    }
}

#[derive(Debug)]
pub struct PixelFormat {
    pub flags: u32,
    pub fourcc: [u8; 4],
    pub rgb_bit_count: u32,
    pub r_bit_mask: u32,
    pub g_bit_mask: u32,
    pub b_bit_mask: u32,
    pub a_bit_mask: u32,
}

impl PixelFormat {
    pub fn from_reader<R: Read>(r: &mut BufReader<R>) -> Result<Self, io::Error> {
        let size = read_u32_le(r)?;

        Ok(Self {
            flags: read_u32_le(r)?,
            fourcc: {
                let mut v = [0; 4];
                r.read_exact(&mut v)?;
                v
            },
            rgb_bit_count: read_u32_le(r)?,
            r_bit_mask: read_u32_le(r)?,
            g_bit_mask: read_u32_le(r)?,
            b_bit_mask: read_u32_le(r)?,
            a_bit_mask: read_u32_le(r)?,
        })
    }
}

#[derive(Debug)]
pub struct BptcImage {
    pub header: Header,
    pub dx10_header: DX10Header,
    pub bytes: Vec<u8>,
}

impl BptcImage {
    pub fn from_path(path: &'static str) -> Result<Self, io::Error> {
        let file = fs::File::open(path).unwrap();
        let mut reader = io::BufReader::new(file);

        let mut magic = [0; 4];
        reader.read_exact(&mut magic)?;

        if magic != "DDS ".as_bytes() {
            return Err(io::Error::from(ErrorKind::InvalidData));
        }

        let header = Header::from_reader(&mut reader)?;
        let dx10_header = DX10Header::from_reader(&mut reader)?;

        if !(dx10_header.dxgi_format >= 94 && dx10_header.dxgi_format <= 99) {
            return Err(io::Error::from(ErrorKind::InvalidData));
        }

        let mut bytes = Vec::new();
        let bytes_read = reader.read_to_end(&mut bytes);

        Ok(Self {
            header,
            dx10_header,
            bytes,
        })
    }
}


impl Image {
    pub fn from_path(path: &'static str) -> Result<Self, io::Error> {
        // check if it's bc7 format which Image crate doesn't support
        if let Ok(bptc_image) = BptcImage::from_path(path) {
            let format = match bptc_image.dx10_header.dxgi_format {
                94 /* DXGI_FORMAT_BC6H_TYPELESS  */=> {ImageFormat::Bc6hUnsignedFloat16},
                95 /* DXGI_FORMAT_BC6H_UF16  */=> {ImageFormat::Bc6hUnsignedFloat16}, 
                96 /* DXGI_FORMAT_BC6H_SF16  */=> {ImageFormat::Bc6hSignedFloat16},
                97 /* DXGI_FORMAT_BC7_TYPELESS */=> {ImageFormat::Bc7UnsignedNormalised}, 
                98 /* DXGI_FORMAT_BC7_UNORM  */=> {ImageFormat::Bc7UnsignedNormalised},
                99 /* DXGI_FORMAT_BC7_UNORM_SRGB  */=> {ImageFormat::Bc7UnsignedNormalisedSrgb},
                _ => return Err(io::Error::from(ErrorKind::InvalidData))
            };

            return Ok(Self {
                path,
                format,
                data_type: DataType::NA, // unused
                compressed: true,
                mipmap_count: bptc_image.header.mipmap_count,
                width: bptc_image.header.width,
                height: bptc_image.header.height,
                bytes: bptc_image.bytes,
            });
        }

        if let Ok(image) = image::open(path) {
            let (format, data_type) = match image {
                ImageRgb8(_) => (ImageFormat::RGB, DataType::Uint8),
                ImageRgba8(_) => (ImageFormat::RGBA, DataType::Uint8),
                ImageRgb16(_) => (ImageFormat::RGB, DataType::Uint16),
                ImageRgba16(_) => (ImageFormat::RGBA,  DataType::Uint16),
                ImageRgb32F(_) => (ImageFormat::RGB,  DataType::Float32),
                ImageRgba32F(_) => (ImageFormat::RGBA, DataType::Float32),
                _ => {
                    return Err(io::Error::from(ErrorKind::InvalidData));
                }
            };


            return Ok(Self {
                path,
                format,
                data_type,
                compressed: false,
                mipmap_count: 1,
                width: image.width(),
                height: image.height(),
                bytes: image.as_bytes().to_vec(),
            });
        }

        return Err(io::Error::from(ErrorKind::InvalidData));
    }
}
