#![allow(dead_code)]

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Vec4f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vec4f {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Vec4f { x, y, z, w }
    }

    pub fn as_tuple(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.z, self.w)
    }

    pub fn as_vec3f(&self) -> Vec3f {
        Vec3f {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }

    pub fn normalise(&self) -> Self {
        let m = self.magnitude();

        Vec4f {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
            w: self.w / m,
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }
}

impl std::ops::Add for Vec4f {
    type Output = Vec4f;

    fn add(self, rhs: Self) -> Self::Output {
        let (ax, ay, az, aw) = self.as_tuple();
        let (bx, by, bz, bw) = rhs.as_tuple();

        Vec4f {
            x: ax + bx,
            y: ay + by,
            z: az + bz,
            w: aw + bw,
        }
    }
}

impl std::ops::AddAssign for Vec4f {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl std::ops::Sub for Vec4f {
    type Output = Vec4f;

    fn sub(self, rhs: Self) -> Self::Output {
        let (ax, ay, az, aw) = self.as_tuple();
        let (bx, by, bz, bw) = rhs.as_tuple();

        Vec4f {
            x: ax - bx,
            y: ay - by,
            z: az - bz,
            w: aw - bw,
        }
    }
}

impl std::ops::SubAssign for Vec4f {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3f { x, y, z }
    }

    pub fn from_slice(slice: &[f32]) -> Self {
        Vec3f {
            x: slice[0],
            y: slice[1],
            z: slice[2],
        }
    }

    pub fn from_array(slice: [f32; 3]) -> Self {
        Vec3f {
            x: slice[0],
            y: slice[1],
            z: slice[2],
        }
    }

    pub fn as_tuple(&self) -> (f32, f32, f32) {
        (self.x, self.y, self.z)
    }

    pub fn normalise(&self) -> Self {
        let m = self.magnitude();

        Vec3f {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn cross(&self, rhs: Self) -> Self {
        let (ax, ay, az) = self.as_tuple();
        let (bx, by, bz) = rhs.as_tuple();

        Vec3f {
            x: ay * bz - az * by,
            y: az * bx - ax * bz,
            z: ax * by - ay * bx,
        }
    }

    pub fn scalar(&self, rhs: f32) -> Self {
        let (ax, ay, az) = self.as_tuple();

        Vec3f {
            x: ax * rhs,
            y: ay * rhs,
            z: az * rhs,
        }
    }

    pub fn dot(&self, rhs: Self) -> f32 {
        let (ax, ay, az) = self.as_tuple();
        let (bx, by, bz) = rhs.as_tuple();

        ax * bx + ay * by + az * bz
    }
}

impl std::ops::Add for Vec3f {
    type Output = Vec3f;

    fn add(self, rhs: Self) -> Self::Output {
        let (ax, ay, az) = self.as_tuple();
        let (bx, by, bz) = rhs.as_tuple();

        Vec3f {
            x: ax + bx,
            y: ay + by,
            z: az + bz,
        }
    }
}

impl std::ops::AddAssign for Vec3f {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub for Vec3f {
    type Output = Vec3f;

    fn sub(self, rhs: Self) -> Self::Output {
        let (ax, ay, az) = self.as_tuple();
        let (bx, by, bz) = rhs.as_tuple();

        Vec3f {
            x: ax - bx,
            y: ay - by,
            z: az - bz,
        }
    }
}

impl std::ops::SubAssign for Vec3f {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2f { x, y }
    }

    pub fn from_slice(slice: &[f32]) -> Self {
        Vec2f {
            x: slice[0],
            y: slice[1],
        }
    }

    pub fn from_array(slice: [f32; 2]) -> Self {
        Vec2f {
            x: slice[0],
            y: slice[1],
        }
    }

    pub fn as_tuple(&self) -> (f32, f32) {
        (self.x, self.y)
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct Vec2u {
    pub x: u32,
    pub y: u32,
}

impl Vec2u {
    pub fn new(x: u32, y: u32) -> Self {
        Vec2u { x, y }
    }

    pub fn from_slice(slice: &[u32]) -> Self {
        Vec2u {
            x: slice[0],
            y: slice[1],
        }
    }

    pub fn from_array(slice: [u32; 2]) -> Self {
        Vec2u {
            x: slice[0],
            y: slice[1],
        }
    }

    pub fn as_tuple(&self) -> (u32, u32) {
        (self.x, self.y)
    }
}

#[derive(Clone, Copy, Default)]
pub struct Mat4f(pub [f32; 16]);

impl Mat4f {
    #![cfg_attr(rustfmt, rustfmt_skip)]
    
    pub fn identity() -> Self {
        Mat4f([
            1.0, 0.0, 0.0, 0.0, 
            0.0, 1.0, 0.0, 0.0, 
            0.0, 0.0, 1.0, 0.0, 
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn mul_vec4(&self, rhs: Vec4f) -> Vec4f {
        Vec4f { 
            x: self.0[0] * rhs.x
            + self.0[1] * rhs.y
            + self.0[2] * rhs.z
            + self.0[3] * rhs.w, 
            y: self.0[4] * rhs.x
            + self.0[5] * rhs.y
            + self.0[6] * rhs.z
            + self.0[7] * rhs.w,
            z: self.0[8] * rhs.x
            + self.0[9] * rhs.y
            + self.0[10] * rhs.z
            + self.0[11] * rhs.w, 
            w: self.0[12] * rhs.x
            + self.0[13] * rhs.y
            + self.0[14] * rhs.z
            + self.0[15] * rhs.w, }
    }

    pub fn perspective(aspect_ratio: f32, fov_rad: f32, near: f32, far: f32) -> Self {
        let a = aspect_ratio;
        let f = 1.0 / (fov_rad / 2.0).tan();

        // transformed Z, Zt = (g * Z + h) / Z
        // division by Z occurs in graphics pipeline
        let g = - (far + near) / (far - near);
        let h = - (2.0 * far * near) / (far - near);

        
        Mat4f([
            f / a, 0.0, 0.0, 0.0,
            0.0, f, 0.0, 0.0,
            0.0, 0.0, g, h,
            0.0, 0.0, -1.0, 0.0,
        ])
    }

    pub fn orthographic(aspect_ratio: f32, size: f32, near: f32, far: f32) -> Self {
        // let a = aspect_ratio;
        let w = size;
        let h = w ;

        let r = w / 2.0;
        let l = -r;
        let t = h / 2.0;
        let b = -t;
        
        Mat4f([
            2.0 / (r-l), 0.0, 0.0, -(r+l)/(r-l), 
            0.0, 2.0/(t-b), 0.0, -(t+b)/(t-b), 
            0.0, 0.0, -2.0/(far-near), -(far + near)/(far - near), 
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        Mat4f([
            1.0, 0.0, 0.0, x, 
            0.0, 1.0, 0.0, y, 
            0.0, 0.0, 1.0, z, 
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        Mat4f([
            x, 0.0, 0.0, 0.0, 
            0.0, y, 0.0, 0.0, 
            0.0, 0.0, z, 0.0, 
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn rotate(rad: f32, direction: &Vec3f) -> Self {
        let (x, y, z) = direction.normalise().as_tuple();

        let cos = rad.cos();
        let omcos= 1.0 - cos;
        let sin = rad.sin();

        Mat4f([
                cos + x*x * omcos,
                x * y * omcos - (z * sin),
                x * z * omcos + (y * sin),
                0.0,
            
                y * x * omcos + (z * sin),
                cos + y*y * omcos,
                y * z * omcos - (x * sin),
                0.0,
            
                z * x * omcos - (y * sin),
                z * y * omcos + (x * sin),
                cos + z*z * omcos,
                0.0,
            
                0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn rotate_around_x(rad: f32) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();

        Mat4f([
            1.0, 0.0, 0.0, 0.0,
            0.0, cos, sin, 0.0,
            0.0, -sin, cos, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn rotate_around_y(rad: f32) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();

        Mat4f([
            cos, 0.0, -sin, 0.0,
            0.0, 1.0, 0.0, 0.0,
            sin, 0.0, cos, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn rotate_around_z(rad: f32) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();

        Mat4f([
            cos, sin, 0.0, 0.0,
            -sin, cos, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn transpose(&self) -> Self {
        Mat4f([
            self.0[0], self.0[4], self.0[8], self.0[12],
            self.0[1], self.0[5], self.0[9], self.0[13],
            self.0[2], self.0[6], self.0[10], self.0[14],
            self.0[3], self.0[7], self.0[11], self.0[15],
        ])
    }

    /// row-major
    pub fn as_slice(&self) -> &[f32] {
        &self.0
    }

  
}

/// Multiplies as if self is on right hand side
impl std::ops::MulAssign for Mat4f {
    fn mul_assign(&mut self, lhs: Self) {
        self.0[0] = lhs.0[0] * self.0[0]
            + lhs.0[1] * self.0[4]
            + lhs.0[2] * self.0[8]
            + lhs.0[3] * self.0[12];
        self.0[1] = lhs.0[0] * self.0[1]
            + lhs.0[1] * self.0[5]
            + lhs.0[2] * self.0[9]
            + lhs.0[3] * self.0[13];
        self.0[2] = lhs.0[0] * self.0[2]
            + lhs.0[1] * self.0[6]
            + lhs.0[2] * self.0[10]
            + lhs.0[3] * self.0[14];
        self.0[3] = lhs.0[0] * self.0[3]
            + lhs.0[1] * self.0[7]
            + lhs.0[2] * self.0[11]
            + lhs.0[3] * self.0[15];

        self.0[4] = lhs.0[4] * self.0[0]
            + lhs.0[5] * self.0[4]
            + lhs.0[6] * self.0[8]
            + lhs.0[7] * self.0[12];
        self.0[5] = lhs.0[4] * self.0[1]
            + lhs.0[5] * self.0[5]
            + lhs.0[6] * self.0[9]
            + lhs.0[7] * self.0[13];
        self.0[6] = lhs.0[4] * self.0[2]
            + lhs.0[5] * self.0[6]
            + lhs.0[6] * self.0[10]
            + lhs.0[7] * self.0[14];
        self.0[7] = lhs.0[4] * self.0[3]
            + lhs.0[5] * self.0[7]
            + lhs.0[6] * self.0[11]
            + lhs.0[7] * self.0[15];

        self.0[8] = lhs.0[8] * self.0[0]
            + lhs.0[9] * self.0[4]
            + lhs.0[10] * self.0[8]
            + lhs.0[11] * self.0[12];
        self.0[9] = lhs.0[8] * self.0[1]
            + lhs.0[9] * self.0[5]
            + lhs.0[10] * self.0[9]
            + lhs.0[11] * self.0[13];
        self.0[10] = lhs.0[8] * self.0[2]
            + lhs.0[9] * self.0[6]
            + lhs.0[10] * self.0[10]
            + lhs.0[11] * self.0[14];
        self.0[11] = lhs.0[8] * self.0[3]
            + lhs.0[9] * self.0[7]
            + lhs.0[10] * self.0[11]
            + lhs.0[11] * self.0[15];

        self.0[12] = lhs.0[12] * self.0[0]
            + lhs.0[13] * self.0[4]
            + lhs.0[14] * self.0[8]
            + lhs.0[15] * self.0[12];
        self.0[13] = lhs.0[12] * self.0[1]
            + lhs.0[13] * self.0[5]
            + lhs.0[14] * self.0[9]
            + lhs.0[15] * self.0[13];
        self.0[14] = lhs.0[12] * self.0[2]
            + lhs.0[13] * self.0[6]
            + lhs.0[14] * self.0[10]
            + lhs.0[15] * self.0[14];
        self.0[15] = lhs.0[12] * self.0[3]
            + lhs.0[13] * self.0[7]
            + lhs.0[14] * self.0[11]
            + lhs.0[15] * self.0[15];
    }
}

impl std::ops::Mul for Mat4f {
    type Output = Mat4f;

    fn mul(self, rhs: Self) -> Self {
        let mut c = Mat4f::identity();

        c.0[0] = self.0[0] * rhs.0[0]
            + self.0[1] * rhs.0[4]
            + self.0[2] * rhs.0[8]
            + self.0[3] * rhs.0[12];
        c.0[1] = self.0[0] * rhs.0[1]
            + self.0[1] * rhs.0[5]
            + self.0[2] * rhs.0[9]
            + self.0[3] * rhs.0[13];
        c.0[2] = self.0[0] * rhs.0[2]
            + self.0[1] * rhs.0[6]
            + self.0[2] * rhs.0[10]
            + self.0[3] * rhs.0[14];
        c.0[3] = self.0[0] * rhs.0[3]
            + self.0[1] * rhs.0[7]
            + self.0[2] * rhs.0[11]
            + self.0[3] * rhs.0[15];

        c.0[4] = self.0[4] * rhs.0[0]
            + self.0[5] * rhs.0[4]
            + self.0[6] * rhs.0[8]
            + self.0[7] * rhs.0[12];
        c.0[5] = self.0[4] * rhs.0[1]
            + self.0[5] * rhs.0[5]
            + self.0[6] * rhs.0[9]
            + self.0[7] * rhs.0[13];
        c.0[6] = self.0[4] * rhs.0[2]
            + self.0[5] * rhs.0[6]
            + self.0[6] * rhs.0[10]
            + self.0[7] * rhs.0[14];
        c.0[7] = self.0[4] * rhs.0[3]
            + self.0[5] * rhs.0[7]
            + self.0[6] * rhs.0[11]
            + self.0[7] * rhs.0[15];

        c.0[8] = self.0[8] * rhs.0[0]
            + self.0[9] * rhs.0[4]
            + self.0[10] * rhs.0[8]
            + self.0[11] * rhs.0[12];
        c.0[9] = self.0[8] * rhs.0[1]
            + self.0[9] * rhs.0[5]
            + self.0[10] * rhs.0[9]
            + self.0[11] * rhs.0[13];
        c.0[10] = self.0[8] * rhs.0[2]
            + self.0[9] * rhs.0[6]
            + self.0[10] * rhs.0[10]
            + self.0[11] * rhs.0[14];
        c.0[11] = self.0[8] * rhs.0[3]
            + self.0[9] * rhs.0[7]
            + self.0[10] * rhs.0[11]
            + self.0[11] * rhs.0[15];

        c.0[12] = self.0[12] * rhs.0[0]
            + self.0[13] * rhs.0[4]
            + self.0[14] * rhs.0[8]
            + self.0[15] * rhs.0[12];
        c.0[13] = self.0[12] * rhs.0[1]
            + self.0[13] * rhs.0[5]
            + self.0[14] * rhs.0[9]
            + self.0[15] * rhs.0[13];
        c.0[14] = self.0[12] * rhs.0[2]
            + self.0[13] * rhs.0[6]
            + self.0[14] * rhs.0[10]
            + self.0[15] * rhs.0[14];
        c.0[15] = self.0[12] * rhs.0[3]
            + self.0[13] * rhs.0[7]
            + self.0[14] * rhs.0[11]
            + self.0[15] * rhs.0[15];

        c
    }
}

unsafe impl bytemuck::Pod for Mat4f {}
unsafe impl bytemuck::Zeroable for Mat4f {}

mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn mat4_mul_test() {
        let a = Mat4f([
            1.0, 3.0, 5.0, 7.0, 6.0, 5.0, 4.0, 3.0, 1.0, 2.0, 3.0, 4.0, 9.0, 8.0, 7.0, 6.0,
        ]);

        let b = Mat4f([
            9.0, 11.0, 13.0, 15.0, 1.0, 2.0, 3.0, 4.0, 10.0, 11.0, 12.0, 13.0, 1.0, 3.0, 5.0, 7.0,
        ]);

        assert_eq!(
            [
                69.0, 93.0, 117.0, 141.0, 102.0, 129.0, 156.0, 183.0, 45.0, 60.0, 75.0, 90.0,
                165.0, 210.0, 255.0, 300.0,
            ],
            (a * b).as_slice()
        );
    }

    #[test]
    fn cross_test() {
        let a = Vec3f::new(27.0, 45.0, 7.0);
        let b = Vec3f::new(4.0, 21.0, 83.0);

        assert_eq!((3588.0, -2213.0, 387.0), a.cross(b).as_tuple())
    }

    #[test]
    fn dot_test() {
        let a = Vec3f::new(27.0, 45.0, 7.0);
        let b = Vec3f::new(4.0, 21.0, 83.0);

        assert_eq!(1634.0, a.dot(b))
    }
}
