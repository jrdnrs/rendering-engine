use super::Vec4f;

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3f {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3f { x, y, z }
    }

    pub fn uniform(a: f32) -> Self {
        Vec3f { x: a, y: a, z: a }
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

    pub fn dot(&self, rhs: Self) -> f32 {
        let (ax, ay, az) = self.as_tuple();
        let (bx, by, bz) = rhs.as_tuple();

        ax * bx + ay * by + az * bz
    }

    pub fn sqrt(&self) -> Self {
        let (ax, ay, az) = self.as_tuple();

        Vec3f {
            x: ax.sqrt(),
            y: ay.sqrt(),
            z: az.sqrt(),
        }
    }
}

impl std::convert::From<Vec4f> for Vec3f {
    fn from(input: Vec4f) -> Self {
        Vec3f {
            x: input.x,
            y: input.y,
            z: input.z,
        }
    }
}

impl std::ops::Index<usize> for Vec3f {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            n => panic!("Vec3f index '{}' out of bounds", n),
        }
    }
}

impl std::ops::IndexMut<usize> for Vec3f {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            n => panic!("Vec3f index '{}' out of bounds", n),
        }
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

impl std::ops::Add<f32> for Vec3f {
    type Output = Vec3f;

    fn add(self, rhs: f32) -> Self::Output {
        let (ax, ay, az) = self.as_tuple();

        Vec3f {
            x: ax + rhs,
            y: ay + rhs,
            z: az + rhs,
        }
    }
}

impl std::ops::AddAssign<f32> for Vec3f {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
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

impl std::ops::Sub<f32> for Vec3f {
    type Output = Vec3f;

    fn sub(self, rhs: f32) -> Self::Output {
        let (ax, ay, az) = self.as_tuple();

        Vec3f {
            x: ax - rhs,
            y: ay - rhs,
            z: az - rhs,
        }
    }
}

impl std::ops::SubAssign<f32> for Vec3f {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
    }
}

impl std::ops::Mul<f32> for Vec3f {
    type Output = Vec3f;

    fn mul(self, rhs: f32) -> Self::Output {
        let (ax, ay, az) = self.as_tuple();

        Vec3f {
            x: ax * rhs,
            y: ay * rhs,
            z: az * rhs,
        }
    }
}

impl std::ops::MulAssign<f32> for Vec3f {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl std::ops::Div<f32> for Vec3f {
    type Output = Vec3f;

    fn div(self, rhs: f32) -> Self::Output {
        let (ax, ay, az) = self.as_tuple();

        Vec3f {
            x: ax / rhs,
            y: ay / rhs,
            z: az / rhs,
        }
    }
}

impl std::ops::DivAssign<f32> for Vec3f {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

impl std::ops::Neg for Vec3f {
    type Output = Vec3f;

    fn neg(self) -> Self::Output {
        Vec3f {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}