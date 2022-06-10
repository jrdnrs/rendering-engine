#![allow(dead_code)]

use std::ops::{Index, IndexMut};

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
    
    pub fn from_rows(rows: &[[f32; 4]; 4]) -> Self {
        Mat4f({[
            rows[0][0], rows[1][0], rows[2][0], rows[3][0],
            rows[0][1], rows[1][1], rows[2][1], rows[3][1],
            rows[0][2], rows[1][2], rows[2][2], rows[3][2],
            rows[0][3], rows[1][3], rows[2][3], rows[3][3],
        ]})
    }

    pub fn from_columns(cols: &[[f32; 4]; 4]) -> Self {
        Mat4f({[
            cols[0][0], cols[0][1], cols[0][2], cols[0][3],
            cols[1][0], cols[1][1], cols[1][2], cols[1][3],
            cols[2][0], cols[2][1], cols[2][2], cols[2][3],
            cols[3][0], cols[3][1], cols[3][2], cols[3][3],
        ]})
    }

    pub fn zero() -> Self {
        Mat4f([0.0; 16])
    }


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
            x: self[(0, 0)] * rhs.x
            +  self[(0, 1)] * rhs.y
            +  self[(0, 2)] * rhs.z
            +  self[(0, 3)] * rhs.w, 
            y: self[(1, 0)] * rhs.x
            +  self[(1, 1)] * rhs.y
            +  self[(1, 2)] * rhs.z
            +  self[(1, 3)] * rhs.w,
            z: self[(2, 0)] * rhs.x
            +  self[(2, 1)] * rhs.y
            +  self[(2, 2)] * rhs.z
            +  self[(2, 3)] * rhs.w, 
            w: self[(3, 0)] * rhs.x
            +  self[(3, 1)] * rhs.y
            +  self[(3, 2)] * rhs.z
            +  self[(3, 3)] * rhs.w, }
    }

    pub fn perspective(aspect_ratio: f32, fov_rad: f32, near: f32, far: f32) -> Self {
        let a = aspect_ratio;
        let f = 1.0 / (fov_rad / 2.0).tan();

        // transformed Z, Zt = (g * Z + h) / Z
        // division by Z occurs in graphics pipeline
        let g = - (far + near) / (far - near);
        let h = - (2.0 * far * near) / (far - near);

        let mut matrix = Mat4f::zero();
        
        matrix[(0, 0)] = f / a;
        matrix[(1, 1)] = f;
        matrix[(2, 2)] = g;
        matrix[(2, 3)] = h;
        matrix[(3, 2)] = -1.0;

        matrix
    }

    pub fn orthographic(aspect_ratio: f32, size: f32, near: f32, far: f32) -> Self {
        // let a = aspect_ratio;
        let w = size;
        let h = w ;

        let r = w / 2.0;
        let l = -r;
        let t = h / 2.0;
        let b = -t;
        

        let mut matrix = Mat4f::identity();
        
        matrix[(0, 0)] = 2.0 / (r - l);
        matrix[(0, 3)] = - (r + l) / (r - l);
        matrix[(1, 1)] = 2.0 / (t - b);
        matrix[(1, 3)] = - (t + b) / (t - b);
        matrix[(2, 2)] = -2.0 / (far - near);
        matrix[(2, 3)] = - (far + near) / (far - near);

        matrix
    }

    pub fn translate(x: f32, y: f32, z: f32) -> Self {
        let mut matrix = Mat4f::identity();
        
        matrix[(0, 3)] = x;
        matrix[(1, 3)] = y;
        matrix[(2, 3)] = z;

        matrix
    }

    pub fn scale(x: f32, y: f32, z: f32) -> Self {
        let mut matrix = Mat4f::identity();
        
        matrix[(0, 0)] = x;
        matrix[(1, 1)] = y;
        matrix[(2, 2)] = z;

        matrix
    }

    pub fn rotate(rad: f32, direction: &Vec3f) -> Self {
        let (x, y, z) = direction.normalise().as_tuple();

        let cos = rad.cos();
        let omcos= 1.0 - cos;
        let sin = rad.sin();


        let mut matrix = Mat4f::identity();
        
        matrix[(0, 0)] = cos + x*x * omcos;
        matrix[(0, 1)] = x * y * omcos - (z * sin);
        matrix[(0, 2)] = x * z * omcos + (y * sin);
        matrix[(1, 0)] = y * x * omcos + (z * sin);
        matrix[(1, 1)] = cos + y*y * omcos;
        matrix[(1, 2)] = y * z * omcos - (x * sin);
        matrix[(2, 0)] = z * x * omcos - (y * sin);
        matrix[(2, 1)] = z * y * omcos + (x * sin);
        matrix[(2, 2)] = cos + z*z * omcos;

        matrix
    }

    pub fn rotate_around_x(rad: f32) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();


        let mut matrix = Mat4f::identity();
        
        matrix[(1, 1)] = cos;
        matrix[(1, 2)] = sin;
        matrix[(2, 1)] = -sin;
        matrix[(2, 2)] = cos;

        matrix
    }

    pub fn rotate_around_y(rad: f32) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();

        let mut matrix = Mat4f::identity();
        
        matrix[(0, 0)] = cos;
        matrix[(0, 2)] = -sin;
        matrix[(2, 0)] = sin;
        matrix[(2, 2)] = cos;

        matrix
    }

    pub fn rotate_around_z(rad: f32) -> Self {
        let cos = rad.cos();
        let sin = rad.sin();

        let mut matrix = Mat4f::identity();
        
        matrix[(0, 0)] = cos;
        matrix[(0, 1)] = sin;
        matrix[(1, 0)] = -sin;
        matrix[(1, 1)] = cos;

        matrix
    }

    pub fn transpose(&self) -> Self {
        Mat4f([
            self[0], self[4], self[8], self[12],
            self[1], self[5], self[9], self[13],
            self[2], self[6], self[10], self[14],
            self[3], self[7], self[11], self[15],
        ])
    }

    /// column-major
    pub fn as_slice(&self) -> &[f32] {
        &self.0
    }
}

impl std::ops::Deref for Mat4f {
    type Target = [f32; 16];
    
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Mat4f {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Index<(usize, usize)> for Mat4f {
    type Output = f32;  

    fn index(&self, row_col: (usize, usize)) -> &Self::Output {
        &self.0[row_col.1 * 4 + row_col.0]
    }

}

impl IndexMut<(usize, usize)> for Mat4f {
    fn index_mut(&mut self, row_col: (usize, usize)) -> &mut Self::Output {
        &mut self.0[row_col.1 * 4 + row_col.0]
    }

}

impl Index<usize> for Mat4f {
    type Output = f32;  

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }

}

impl IndexMut<usize> for Mat4f {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl std::ops::Add for Mat4f {
    type Output = Mat4f;
    
    fn add(self, rhs: Self) -> Self::Output {
        let mut matrix = Mat4f::zero();

        for i in 0..16 {
            matrix[i] = self[i] + rhs[i];
        }

        matrix
    }
}

impl std::ops::Sub for Mat4f {
    type Output = Mat4f;
    
    fn sub(self, rhs: Self) -> Self::Output {
        let mut matrix = Mat4f::zero();

        for i in 0..16 {
            matrix[i] = self[i] - rhs[i];
        }

        matrix
    }
}

impl std::ops::Mul for Mat4f {
    type Output = Mat4f;

    fn mul(self, rhs: Self) -> Self {
        let mut matrix = Mat4f::zero();

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    matrix[(i, j)] += self[(i, k)] * rhs[(k, j)]
                }
            }
        }

        matrix
    }
}

/// Multiplies as if self is on right hand side
impl std::ops::MulAssign for Mat4f {
    fn mul_assign(&mut self, lhs: Self) {
        *self = lhs * *self;
    }
}



unsafe impl bytemuck::Pod for Mat4f {}
unsafe impl bytemuck::Zeroable for Mat4f {}

mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn mat4_mul_test() {
        let a = Mat4f::from_rows(&[
            [1.0, 3.0, 5.0, 7.0],
            [6.0, 5.0, 4.0, 3.0],
            [1.0, 2.0, 3.0, 4.0],
            [9.0, 8.0, 7.0, 6.0],
        ]);

        let b = Mat4f::from_rows(&[
            [9.0, 11.0, 13.0, 15.0],
            [1.0, 2.0, 3.0, 4.0],
            [10.0, 11.0, 12.0, 13.0],
            [1.0, 3.0, 5.0, 7.0],
        ]);

        let answer = Mat4f::from_rows(&[
            [69.0, 93.0, 117.0, 141.0],
            [102.0, 129.0, 156.0, 183.0],
            [45.0, 60.0, 75.0, 90.0],
            [165.0, 210.0, 255.0, 300.0],
        ]);

        assert_eq!(answer.as_slice(), (a * b).as_slice());
    }

    #[test]
    fn mat4_mul_assign_test() {
        let a = Mat4f::from_rows(&[
            [1.0, 3.0, 5.0, 7.0],
            [6.0, 5.0, 4.0, 3.0],
            [1.0, 2.0, 3.0, 4.0],
            [9.0, 8.0, 7.0, 6.0],
        ]);

        let mut b = Mat4f::from_rows(&[
            [9.0, 11.0, 13.0, 15.0],
            [1.0, 2.0, 3.0, 4.0],
            [10.0, 11.0, 12.0, 13.0],
            [1.0, 3.0, 5.0, 7.0],
        ]);

        b *= a;

        let answer = Mat4f::from_rows(&[
            [69.0, 93.0, 117.0, 141.0],
            [102.0, 129.0, 156.0, 183.0],
            [45.0, 60.0, 75.0, 90.0],
            [165.0, 210.0, 255.0, 300.0],
        ]);

        assert_eq!(answer.as_slice(), b.as_slice());
    }

    #[test]
    fn index_rowcol_test() {
        let a = Mat4f::from_rows(&[
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 7.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(7.0, a[(2, 2)]);
    }

    #[test]
    fn index_test() {
        let a = Mat4f::from_rows(&[
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 7.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);

        assert_eq!(7.0, a[10]);
    }

    #[test]
    fn vec3_cross_test() {
        let a = Vec3f::new(27.0, 45.0, 7.0);
        let b = Vec3f::new(4.0, 21.0, 83.0);

        assert_eq!((3588.0, -2213.0, 387.0), a.cross(b).as_tuple())
    }

    #[test]
    fn vec3_dot_test() {
        let a = Vec3f::new(27.0, 45.0, 7.0);
        let b = Vec3f::new(4.0, 21.0, 83.0);

        assert_eq!(1634.0, a.dot(b))
    }
}
