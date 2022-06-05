use crate::math::math::*;

pub enum Projection {
    Orthographic,
    Perspective,
}

pub struct Camera {
    pub position: Vec3f,
    pub direction: Vec3f,
    pub pitch: f32,
    pub yaw: f32,
    pub view: Mat4f,
    pub projection: Mat4f,
    pub fov_size: f32,
    pub near_plane: f32,
    pub far_plane: f32,
    pub projection_type: Projection,
}

impl Camera {
    pub fn new_perspective(fov: f32, near: f32, far: f32) -> Self {
        let position = Vec3f::new(0.0, 0.0, -3.0);
        let direction = Vec3f::new(0.0, 0.0, 1.0);
        let view = Self::look_at(&position, &direction);
        Camera {
            position,
            direction,
            view,
            pitch: 0.0,
            yaw: 90.0,
            projection: Mat4f::perspective(1.0, fov.to_radians(), near, far),
            fov_size: fov,
            near_plane: near,
            far_plane: far,
            projection_type: Projection::Perspective,
        }
    }

    pub fn new_orthographic(size: f32, near: f32, far: f32) -> Self {
        let position = Vec3f::new(0.0, 0.0, -3.0);
        let direction = Vec3f::new(0.0, 0.0, 1.0);
        let view = Self::look_at(&position, &direction);
        Camera {
            position,
            direction,
            view,
            pitch: 0.0,
            yaw: 90.0,
            projection: Mat4f::orthographic(1.0, size, near, far),
            fov_size: size,
            near_plane: near,
            far_plane: far,
            projection_type: Projection::Orthographic,
        }
    }

    pub fn update_view(&mut self) {
        self.view = Self::look_at(&self.position, &self.direction);
    }

    pub fn update_projection(&mut self, width: f32, height: f32) {
        match self.projection_type {
            Projection::Orthographic => {
                self.projection = Mat4f::orthographic(
                    width / height,
                    self.fov_size,
                    self.near_plane,
                    self.far_plane,
                )
            }
            Projection::Perspective => {
                self.projection = Mat4f::perspective(
                    width / height,
                    self.fov_size.to_radians(),
                    self.near_plane,
                    self.far_plane,
                )
            }
        };
    }

    pub fn look_at(position: &Vec3f, direction: &Vec3f) -> Mat4f {
        let (px, py, pz) = (position.x, position.y, position.z);
        let (dx, dy, dz) = (direction.x, direction.y, direction.z);
        let (lx, ly, lz) = Vec3f::new(0.0, 1.0, 0.0)
            .cross(*direction)
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
