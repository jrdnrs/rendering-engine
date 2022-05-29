pub mod model;
pub mod shader;
pub mod texture;

use crate::components::Renderable;
use model::*;
use shader::*;
use texture::*;

use glow::{self as gl, HasContext};
use std::{collections::VecDeque, marker::PhantomData};

pub struct AssetsManager<'a> {
    gl: &'a gl::Context,

    // TODO: I think we should do this differently, using bit keys maybe? This overhead gets much worse when
    // there are more layers to sort
    /// Shader(ID) > Meshes(ID) > Renderables
    pub shader_buckets: Vec<Vec<Vec<Renderable>>>,

    pub mesh_manager: AssetManager<Mesh, MeshID>,
    pub material_manager: AssetManager<Material, MaterialID>,
    pub shader_program_manager: AssetManager<Program<'a>, ShaderProgramID>,
    pub texture_manager: AssetManager<Texture<'a>, TextureID>,
}

impl<'a> AssetsManager<'a> {
    pub fn new(gl: &'a gl::Context) -> Self {
        AssetsManager {
            gl,
            shader_buckets: Vec::new(),
            mesh_manager: AssetManager::new(),
            material_manager: AssetManager::new(),
            shader_program_manager: AssetManager::new(),
            texture_manager: AssetManager::new(),
        }

        // TODO: load default texture and material
    }

    /// This is to maintain the structure of the nested vecs that are used as sorting buckets.
    /// I don't really like this idea, and will probably move to sorting by some derived bit key soon
    fn refresh_sorting_buckets(&mut self) {
        // HACK: using mesh/shader alive_count won't hold when we are able to remove meshes/shaders.
        // what we actually want is the biggest id (index) held by a mesh/shader that is alive, as that is how
        // many vecs we need. as removing is not implemented, this will just be the number that is alive
        
        let delta = self.shader_program_manager.alive_count - self.shader_buckets.len() as u32;
        for _ in 0..delta {
            self.shader_buckets.push(Vec::new())
        }

        for shader_bucket in self.shader_buckets.iter_mut() {
            let delta = self.mesh_manager.alive_count - shader_bucket.len() as u32;
            for _ in 0..delta {
                shader_bucket.push(Vec::new())
            }
        }
    }

    pub fn load_texture(&mut self, path: &'static str) -> Result<TextureID, String> {
        match image::open(path) {
            Ok(img) => {
                let texture = Texture::new(self.gl, &img, TextureConfig::default());
                let id = self.texture_manager.load(texture);
                Ok(id)
            }
            Err(err) => Err(err.to_string()),
        }
    }

    fn remove_texture(&mut self, texture_id: TextureID) {
        // problem: texture handles are uploaded as a Material into ssbo
        // solution: we make the texture non resident, and reupload all materials but now point dead texture ids to
        // a default texture instead? this seems slow, better idea? I guess we kind of have to do this, i just don't
        // like that we iterate through all materials, when we want only ones with this texture. If textures don't
        // change that often, maybe it's fine
    }

    pub fn load_shader(&mut self, path: &'static str) -> ShaderProgramID {
        let mut program = shader::Program::new(self.gl);
        program.add_shaders(path);
        let id = self.shader_program_manager.load(program);
        self.refresh_sorting_buckets();

        id
    }

    fn remove_shader(&mut self, shader_id: ShaderProgramID) {

        // problem: what if a renderable references a dead shader?
        // solution: I think we just don't render it?

        // problem: shader bucket exists
        // solution: remove shader bucket, but that will offset the index!!!
    }

    pub fn load_mesh(&mut self, mesh: Mesh) -> MeshID {
        let id = self.mesh_manager.load(mesh);
        self.refresh_sorting_buckets();

        id
    }

    fn remove_mesh(&mut self, mesh_id: MeshID) {
        // problem: what if a renderable references a dead mesh?
        // solution: I think we just don't render it?

        // problem: each shader bucket will have a vec for this mesh
        // solution: remove mesh buckets, but that will offset the index!!!
    }

    pub fn load_material(&mut self, material: Material) -> MaterialID {
        self.material_manager.load(material)
    }

    fn remove_material(&mut self, material_id: MaterialID) {
        // problem: this material has been uploaded to ssbo
        // solution: just leave it there, we'll make sure not to reference it when rendering future renderables
        // and the id will be recycled when a new material is added

        // problem: if doing the above, a material that takes the place of an old one will appear visible to renderables
        // that were referencing that place but were expecting an older material
        // solution: the id we give out has a version and we check this as well

        // problem: what do we do when a renderable references a material that is dead?
        // solution: we replace material id when rendering with id of the default material
    }
}

const ENTITY_INDEX_BITS: u32 = 22;
const ENTITY_INDEX_MASK: u32 = (1 << ENTITY_INDEX_BITS) - 1;
const MINIMUM_FREE_SPACES: u32 = 0; // keep asset_id list packed, as they directly correspond to actual assets

pub trait AssetManagerTrait {
    type AssetType;
    type AssetIDType;

    fn new() -> Self
    where
        Self: Sized;
    fn load(&mut self, asset: Self::AssetType) -> Self::AssetIDType;
    fn alive(&self, asset_id: &Self::AssetIDType) -> bool;
    fn remove(&mut self, asset_id: Self::AssetIDType);
}

pub trait AssetIDTrait {
    fn new(id: u32) -> Self
    where
        Self: Sized;
    fn index(&self) -> u32;
    fn version(&self) -> u32;
}

pub struct AssetManager<Asset, AssetID: AssetIDTrait> {
    pub assets: Vec<Asset>,
    pub alive_count: u32,
    asset_versions: Vec<u32>,
    free_spaces: VecDeque<u32>,
    _id_type: PhantomData<AssetID>,
}

impl<T, U: AssetIDTrait> AssetManagerTrait for AssetManager<T, U> {
    type AssetType = T;
    type AssetIDType = U;

    fn new() -> Self {
        Self {
            assets: Vec::new(),
            alive_count: 0,
            asset_versions: Vec::new(),
            free_spaces: VecDeque::new(),
            _id_type: PhantomData,
        }
    }

    fn load(&mut self, asset: T) -> U {
        let id: u32;
        if self.free_spaces.len() as u32 > MINIMUM_FREE_SPACES {
            let index = self.free_spaces.pop_front().unwrap();
            id = (self.asset_versions[index as usize] << ENTITY_INDEX_BITS) | index;
            self.assets[index as usize] = asset;
        } else {
            self.asset_versions.push(0);
            id = self.asset_versions.len() as u32 - 1;
            self.assets.push(asset)
        }

        self.alive_count += 1;
        U::new(id)
    }

    fn alive(&self, asset_id: &U) -> bool {
        self.asset_versions[asset_id.index() as usize] == asset_id.version()
    }

    fn remove(&mut self, asset_id: U) {
        if self.alive(&asset_id) {
            let index = asset_id.index();
            self.asset_versions[index as usize] += 1;
            self.free_spaces.push_back(index);
            self.alive_count -= 1;
            // we don't actually remove the asset
            // as the assetID is now regarded dead, the renderable that holds it will be treated differently
            // eventually this asset place will be overwritten the next time one is created
        }
    }
}

// TODO: make this a procedural derive macro
macro_rules! impl_AssetID {
    ($id:ident) => {
        impl AssetIDTrait for $id {
            fn new(id: u32) -> Self {
                Self { id }
            }

            fn index(&self) -> u32 {
                self.id & ENTITY_INDEX_MASK
            }
            fn version(&self) -> u32 {
                self.id >> ENTITY_INDEX_BITS
            }
        }
    };
}

impl_AssetID!(MeshID);
#[derive(Clone, Copy)]
pub struct MeshID {
    id: u32,
}

impl_AssetID!(TextureID);
#[derive(Clone, Copy)]
pub struct TextureID {
    id: u32,
}

impl_AssetID!(ShaderProgramID);
#[derive(Clone, Copy)]
pub struct ShaderProgramID {
    id: u32,
}

impl_AssetID!(MaterialID);
#[derive(Clone, Copy)]
pub struct MaterialID {
    id: u32,
}
