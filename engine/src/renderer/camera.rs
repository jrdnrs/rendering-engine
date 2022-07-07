use crate::math::*;

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
            projection: Mat4f::orthographic(size, near, far),
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

        let mut matrix = Mat4f::identity();

        matrix[(0, 0)] = lx;
        matrix[(0, 1)] = ly;
        matrix[(0, 2)] = lz;
        matrix[(0, 3)] = -lx * px - ly * py - lz * pz;
        matrix[(1, 0)] = ux;
        matrix[(1, 1)] = uy;
        matrix[(1, 2)] = uz;
        matrix[(1, 3)] = -ux * px - uy * py - uz * pz;
        matrix[(2, 0)] = dx;
        matrix[(2, 1)] = dy;
        matrix[(2, 2)] = dz;
        matrix[(2, 3)] = -dx * px - dy * py - dz * pz;

        matrix
    }
}
