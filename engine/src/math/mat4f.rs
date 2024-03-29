use super::{vec3f::Vec3f, vec4f::Vec4f};

#[derive(Debug, Clone, Copy, Default)]
pub struct Mat4f(pub [f32; 16]);

impl Mat4f {
    #![cfg_attr(rustfmt, rustfmt_skip)]
    
    pub fn uniform(a: f32) -> Self {
        Mat4f([a; 16])
    }

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

      pub fn identity() -> Self {
        Mat4f([
            1.0, 0.0, 0.0, 0.0, 
            0.0, 1.0, 0.0, 0.0, 
            0.0, 0.0, 1.0, 0.0, 
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn perspective(aspect_ratio: f32, fov_rad: f32, near: f32, far: f32) -> Self {
        let a = aspect_ratio;
        let f = 1.0 / (fov_rad / 2.0).tan();

        // transformed Z, Zt = (g * Z + h) / Z
        // division by Z occurs in graphics pipeline
        let g = - (far + near) / (far - near);
        let h = - (2.0 * far * near) / (far - near);

        let mut matrix = Mat4f::uniform(0.0);
        
        matrix[(0, 0)] = f / a;
        matrix[(1, 1)] = f;
        matrix[(2, 2)] = g;
        matrix[(2, 3)] = h;
        matrix[(3, 2)] = -1.0;

        matrix
    }

    pub fn orthographic(size: f32, near: f32, far: f32) -> Self {
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

impl std::ops::Index<(usize, usize)> for Mat4f {
    type Output = f32;

    fn index(&self, row_col: (usize, usize)) -> &Self::Output {
        &self.0[row_col.1 * 4 + row_col.0]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Mat4f {
    fn index_mut(&mut self, row_col: (usize, usize)) -> &mut Self::Output {
        &mut self.0[row_col.1 * 4 + row_col.0]
    }
}

impl std::ops::Index<usize> for Mat4f {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for Mat4f {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl std::ops::Add for Mat4f {
    type Output = Mat4f;

    fn add(self, rhs: Self) -> Self::Output {
        let mut matrix = Mat4f::uniform(0.0);

        for i in 0..16 {
            matrix[i] = self[i] + rhs[i];
        }

        matrix
    }
}

impl std::ops::AddAssign for Mat4f {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..16 {
            self[i] += rhs[i];
        }
    }
}

impl std::ops::Add<f32> for Mat4f {
    type Output = Mat4f;

    fn add(self, rhs: f32) -> Self::Output {
        let mut matrix = Mat4f::uniform(0.0);

        for i in 0..16 {
            matrix[i] = self[i] + rhs;
        }

        matrix
    }
}

impl std::ops::AddAssign<f32> for Mat4f {
    fn add_assign(&mut self, rhs: f32) {
        for i in 0..16 {
            self[i] += rhs;
        }
    }
}

impl std::ops::Sub for Mat4f {
    type Output = Mat4f;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut matrix = Mat4f::uniform(0.0);

        for i in 0..16 {
            matrix[i] = self[i] - rhs[i];
        }

        matrix
    }
}

impl std::ops::SubAssign for Mat4f {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..16 {
            self[i] -= rhs[i];
        }
    }
}

impl std::ops::Sub<f32> for Mat4f {
    type Output = Mat4f;

    fn sub(self, rhs: f32) -> Self::Output {
        let mut matrix = Mat4f::uniform(0.0);

        for i in 0..16 {
            matrix[i] = self[i] - rhs;
        }

        matrix
    }
}

impl std::ops::SubAssign<f32> for Mat4f {
    fn sub_assign(&mut self, rhs: f32) {
        for i in 0..16 {
            self[i] -= rhs;
        }
    }
}

impl std::ops::Mul for Mat4f {
    type Output = Mat4f;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut matrix = Mat4f::uniform(0.0);

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

impl std::ops::Mul<f32> for Mat4f {
    type Output = Mat4f;

    fn mul(self, rhs: f32) -> Self::Output {
        let mut matrix = Mat4f::uniform(0.0);

        for i in 0..16 {
            matrix[i] = self[i] * rhs;
        }

        matrix
    }
}

impl std::ops::MulAssign<f32> for Mat4f {
    fn mul_assign(&mut self, rhs: f32) {
        for i in 0..16 {
            self[i] *= rhs;
        }
    }
}

impl std::ops::Mul<Vec4f> for Mat4f {
    type Output = Vec4f;

    fn mul(self, rhs: Vec4f) -> Self::Output {
        Vec4f {
            x: self[(0, 0)] * rhs.x
                + self[(0, 1)] * rhs.y
                + self[(0, 2)] * rhs.z
                + self[(0, 3)] * rhs.w,
            y: self[(1, 0)] * rhs.x
                + self[(1, 1)] * rhs.y
                + self[(1, 2)] * rhs.z
                + self[(1, 3)] * rhs.w,
            z: self[(2, 0)] * rhs.x
                + self[(2, 1)] * rhs.y
                + self[(2, 2)] * rhs.z
                + self[(2, 3)] * rhs.w,
            w: self[(3, 0)] * rhs.x
                + self[(3, 1)] * rhs.y
                + self[(3, 2)] * rhs.z
                + self[(3, 3)] * rhs.w,
        }
    }
}

impl std::ops::Div<f32> for Mat4f {
    type Output = Mat4f;

    fn div(self, rhs: f32) -> Self::Output {
        let mut matrix = Mat4f::uniform(0.0);

        for i in 0..16 {
            matrix[i] = self[i] / rhs;
        }

        matrix
    }
}

impl std::ops::DivAssign<f32> for Mat4f {
    fn div_assign(&mut self, rhs: f32) {
        for i in 0..16 {
            self[i] /= rhs;
        }
    }
}

impl std::ops::Neg for Mat4f {
    type Output = Mat4f;

    fn neg(self) -> Self::Output {
        let mut matrix = Mat4f::uniform(0.0);

        for i in 0..16 {
            matrix[i] = -self[i];
        }

        matrix
    }
}

unsafe impl bytemuck::Pod for Mat4f {}
unsafe impl bytemuck::Zeroable for Mat4f {}
