use crate::math::*;

pub struct FreeCamera {
    pub position: Vec3f,
    pub direction: Vec3f,
    pub pitch: f32,
    pub yaw: f32,
    pub view: Mat4f,
    pub projection: Mat4f,
}

impl FreeCamera {
    pub fn new_perspective(fov: f32, near: f32, far: f32) -> Self {
        let position = Vec3f::new(0.0, 0.0, -3.0);
        let direction = Vec3f::new(0.0, 0.0, 1.0);
        let view = Self::look_at(&position, &direction);
        FreeCamera {
            position,
            direction,
            view,
            pitch: 0.0,
            yaw: 90.0,
            projection: Mat4f::perspective(1.0, fov.to_radians(), near, far),
        }
    }

    pub fn update_view(&mut self) {
        self.view = Self::look_at(&self.position, &self.direction);
    }

    pub fn look_at(position: &Vec3f, direction: &Vec3f) -> Mat4f {
        let (px, py, pz) = (position.x, position.y, position.z);
        let (dx, dy, dz) = (direction.x, direction.y, direction.z);
        let (lx, ly, lz) = direction
            .cross(Vec3f::new(0.0, 1.0, 0.0))
            .normalise()
            .as_tuple();
        let (ux, uy, uz) = direction
            .cross(Vec3f::new(lx, ly, lz))
            .normalise()
            .as_tuple();

        Mat4f([
            lx,
            ly,
            lz,
            -lx * px - ly * py - lz * pz,
            ux,
            uy,
            uz,
            -ux * px - uy * py - uz * pz,
            dx,
            dy,
            dz,
            -dx * px - dy * py - dz * pz,
            0.0,
            0.0,
            0.0,
            1.0,
        ])
    }
}
