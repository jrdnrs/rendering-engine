use super::{Vec3f, Vec4f};

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2f {
    pub x: f32,
    pub y: f32,
}

impl Vec2f {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2f { x, y }
    }

    pub fn uniform(a: f32) -> Self {
        Vec2f { x: a, y: a }
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

    pub fn normalise(&self) -> Self {
        let m = self.magnitude();

        Vec2f {
            x: self.x / m,
            y: self.y / m,
        }
    }

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn dot(&self, rhs: Self) -> f32 {
        let (ax, ay) = self.as_tuple();
        let (bx, by) = rhs.as_tuple();

        ax * bx + ay * by
    }

    pub fn sqrt(&self) -> Self {
        let (ax, ay) = self.as_tuple();

        Vec2f {
            x: ax.sqrt(),
            y: ay.sqrt(),
        }
    }
}

impl std::convert::From<Vec4f> for Vec2f {
    fn from(input: Vec4f) -> Self {
        Vec2f {
            x: input.x,
            y: input.y,
        }
    }
}

impl std::convert::From<Vec3f> for Vec2f {
    fn from(input: Vec3f) -> Self {
        Vec2f {
            x: input.x,
            y: input.y,
        }
    }
}

impl std::ops::Index<usize> for Vec2f {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            n => panic!("Vec2f index '{}' out of bounds", n),
        }
    }
}

impl std::ops::IndexMut<usize> for Vec2f {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            n => panic!("Vec2f index '{}' out of bounds", n),
        }
    }
}

impl std::ops::Add for Vec2f {
    type Output = Vec2f;

    fn add(self, rhs: Self) -> Self::Output {
        let (ax, ay) = self.as_tuple();
        let (bx, by) = rhs.as_tuple();

        Vec2f {
            x: ax + bx,
            y: ay + by,
        }
    }
}

impl std::ops::AddAssign for Vec2f {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl std::ops::Add<f32> for Vec2f {
    type Output = Vec2f;

    fn add(self, rhs: f32) -> Self::Output {
        let (ax, ay) = self.as_tuple();

        Vec2f {
            x: ax + rhs,
            y: ay + rhs,
        }
    }
}

impl std::ops::AddAssign<f32> for Vec2f {
    fn add_assign(&mut self, rhs: f32) {
        self.x += rhs;
        self.y += rhs;
    }
}

impl std::ops::Sub for Vec2f {
    type Output = Vec2f;

    fn sub(self, rhs: Self) -> Self::Output {
        let (ax, ay) = self.as_tuple();
        let (bx, by) = rhs.as_tuple();

        Vec2f {
            x: ax - bx,
            y: ay - by,
        }
    }
}

impl std::ops::SubAssign for Vec2f {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl std::ops::Sub<f32> for Vec2f {
    type Output = Vec2f;

    fn sub(self, rhs: f32) -> Self::Output {
        let (ax, ay) = self.as_tuple();

        Vec2f {
            x: ax - rhs,
            y: ay - rhs,
        }
    }
}

impl std::ops::SubAssign<f32> for Vec2f {
    fn sub_assign(&mut self, rhs: f32) {
        self.x -= rhs;
        self.y -= rhs;
    }
}

impl std::ops::Mul<f32> for Vec2f {
    type Output = Vec2f;

    fn mul(self, rhs: f32) -> Self::Output {
        let (ax, ay) = self.as_tuple();

        Vec2f {
            x: ax * rhs,
            y: ay * rhs,
        }
    }
}

impl std::ops::MulAssign<f32> for Vec2f {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl std::ops::Div<f32> for Vec2f {
    type Output = Vec2f;

    fn div(self, rhs: f32) -> Self::Output {
        let (ax, ay) = self.as_tuple();

        Vec2f {
            x: ax / rhs,
            y: ay / rhs,
        }
    }
}

impl std::ops::DivAssign<f32> for Vec2f {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl std::ops::Neg for Vec2f {
    type Output = Vec2f;

    fn neg(self) -> Self::Output {
        Vec2f {
            x: -self.x,
            y: -self.y,
        }
    }
}
