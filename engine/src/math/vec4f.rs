#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
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

    pub fn uniform(a: f32) -> Self {
        Vec4f {
            x: a,
            y: a,
            z: a,
            w: a,
        }
    }

    pub fn from_slice(slice: &[f32]) -> Self {
        Vec4f {
            x: slice[0],
            y: slice[1],
            z: slice[2],
            w: slice[3],
        }
    }

    pub fn from_array(slice: [f32; 4]) -> Self {
        Vec4f {
            x: slice[0],
            y: slice[1],
            z: slice[2],
            w: slice[3],
        }
    }

    pub fn as_tuple(&self) -> (f32, f32, f32, f32) {
        (self.x, self.y, self.z, self.w)
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

    pub fn dot(&self, rhs: Self) -> f32 {
        let (ax, ay, az, aw) = self.as_tuple();
        let (bx, by, bz, bw) = rhs.as_tuple();

        ax * bx + ay * by + az * bz + aw * bw
    }

    pub fn sqrt(&self) -> Self {
        let (ax, ay, az, aw) = self.as_tuple();

        Vec4f {
            x: ax.sqrt(),
            y: ay.sqrt(),
            z: az.sqrt(),
            w: aw.sqrt(),
        }
    }
}

impl std::ops::Index<usize> for Vec4f {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            n => panic!("Vec4f index '{}' out of bounds", n),
        }
    }
}

impl std::ops::IndexMut<usize> for Vec4f {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            n => panic!("Vec4f index '{}' out of bounds", n),
        }
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

impl std::ops::Add<f32> for Vec4f {
    type Output = Vec4f;

    fn add(self, rhs: f32) -> Self::Output {
        let (ax, ay, az, aw) = self.as_tuple();

        Vec4f {
            x: ax + rhs,
            y: ay + rhs,
            z: az + rhs,
            w: aw + rhs,
        }
    }
}

impl std::ops::AddAssign<f32> for Vec4f {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
        self.w += rhs;
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

impl std::ops::Sub<f32> for Vec4f {
    type Output = Vec4f;

    fn sub(self, rhs: f32) -> Self::Output {
        let (ax, ay, az, aw) = self.as_tuple();

        Vec4f {
            x: ax - rhs,
            y: ay - rhs,
            z: az - rhs,
            w: aw - rhs,
        }
    }
}

impl std::ops::SubAssign<f32> for Vec4f {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
        self.w -= rhs;
    }
}

impl std::ops::Mul<f32> for Vec4f {
    type Output = Vec4f;

    fn mul(self, rhs: f32) -> Self::Output {
        let (ax, ay, az, aw) = self.as_tuple();

        Vec4f {
            x: ax * rhs,
            y: ay * rhs,
            z: az * rhs,
            w: aw * rhs,
        }
    }
}

impl std::ops::MulAssign<f32> for Vec4f {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl std::ops::Div<f32> for Vec4f {
    type Output = Vec4f;

    fn div(self, rhs: f32) -> Self::Output {
        let (ax, ay, az, aw) = self.as_tuple();

        Vec4f {
            x: ax / rhs,
            y: ay / rhs,
            z: az / rhs,
            w: aw / rhs,
        }
    }
}

impl std::ops::DivAssign<f32> for Vec4f {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}


impl std::ops::Neg for Vec4f {
    type Output = Vec4f;

    fn neg(self) -> Self::Output {
        Vec4f {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w
        }
    }
}