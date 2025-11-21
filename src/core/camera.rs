use crate::core::{Matrix4, Transform, Vector3};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Camera {
    pub transform: Transform,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub near_clip: f32,
    pub far_clip: f32,
    pub orthographic: bool,
    pub orthographic_size: f32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            transform: Transform::new(),
            fov: 60.0_f32.to_radians(),
            aspect_ratio: 16.0 / 9.0,
            near_clip: 0.1,
            far_clip: 100.0,
            orthographic: false,
            orthographic_size: 5.0,
        }
    }

    pub fn with_position(mut self, position: Vector3) -> Self {
        self.transform.position = position;
        self
    }

    pub fn with_orientation(mut self, rotation: crate::core::Quaternion) -> Self {
        self.transform.rotation = rotation;
        self
    }

    pub fn with_fov(mut self, fov_degrees: f32) -> Self {
        self.fov = fov_degrees.to_radians();
        self
    }

    pub fn with_aspect_ratio(mut self, aspect: f32) -> Self {
        self.aspect_ratio = aspect;
        self
    }

    pub fn with_clipping_planes(mut self, near: f32, far: f32) -> Self {
        self.near_clip = near;
        self.far_clip = far;
        self
    }

    pub fn orthographic(mut self, size: f32) -> Self {
        self.orthographic = true;
        self.orthographic_size = size;
        self
    }

    pub fn perspective(mut self) -> Self {
        self.orthographic = false;
        self
    }

    pub fn view_matrix(&self) -> Matrix4 {
        let target = self.transform.position + self.forward();
        self.look_at_matrix(target, self.up())
    }

    pub fn projection_matrix(&self) -> Matrix4 {
        if self.orthographic {
            self.orthographic_projection_matrix()
        } else {
            self.perspective_projection_matrix()
        }
    }

    fn perspective_projection_matrix(&self) -> Matrix4 {
        let f = 1.0 / (self.fov * 0.5).tan();
        let nf = 1.0 / (self.near_clip - self.far_clip);

        Matrix4 {
            data: [
                [f / self.aspect_ratio, 0.0, 0.0, 0.0],
                [0.0, f, 0.0, 0.0],
                [0.0, 0.0, (self.far_clip + self.near_clip) * nf, -1.0],
                [0.0, 0.0, 2.0 * self.far_clip * self.near_clip * nf, 0.0],
            ],
        }
    }

    fn orthographic_projection_matrix(&self) -> Matrix4 {
        let width = self.orthographic_size * self.aspect_ratio;
        let height = self.orthographic_size;
        let nf = 1.0 / (self.near_clip - self.far_clip);

        Matrix4 {
            data: [
                [2.0 / width, 0.0, 0.0, 0.0],
                [0.0, 2.0 / height, 0.0, 0.0],
                [0.0, 0.0, 2.0 * nf, 0.0],
                [0.0, 0.0, (self.far_clip + self.near_clip) * nf, 1.0],
            ],
        }
    }

    fn look_at_matrix(&self, target: Vector3, up: Vector3) -> Matrix4 {
        let forward = (target - self.transform.position).normalized();
        let right = up.cross(&forward).normalized();
        let up = forward.cross(&right);

        let pos = self.transform.position;
        Matrix4 {
            data: [
                [right.x, right.y, right.z, -right.dot(&pos)],
                [up.x, up.y, up.z, -up.dot(&pos)],
                [-forward.x, -forward.y, -forward.z, forward.dot(&pos)],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn forward(&self) -> Vector3 {
        self.transform.rotation.rotate_vector(Vector3::forward())
    }

    pub fn right(&self) -> Vector3 {
        self.transform.rotation.rotate_vector(Vector3::right())
    }

    pub fn up(&self) -> Vector3 {
        self.transform.rotation.rotate_vector(Vector3::up())
    }

    pub fn screen_point_to_ray(
        &self,
        screen_x: f32,
        screen_y: f32,
        screen_width: f32,
        screen_height: f32,
    ) -> (Vector3, Vector3) {
        let x = (2.0 * screen_x / screen_width - 1.0) * self.aspect_ratio;
        let y = 1.0 - 2.0 * screen_y / screen_height;

        let origin = self.transform.position;
        let direction = if self.orthographic {
            let world_x = x * self.orthographic_size * self.aspect_ratio;
            let world_y = y * self.orthographic_size;

            let right = self.right() * world_x;
            let up = self.up() * world_y;

            (self.forward() + right + up).normalized()
        } else {
            let tan_fov = (self.fov * 0.5).tan();
            let dir_x = x * tan_fov * self.aspect_ratio;
            let dir_y = y * tan_fov;

            let right = self.right() * dir_x;
            let up = self.up() * dir_y;

            (self.forward() + right + up).normalized()
        };

        (origin, direction)
    }

    pub fn world_to_screen_point(
        &self,
        world_pos: Vector3,
        screen_width: f32,
        screen_height: f32,
    ) -> Option<(f32, f32)> {
        let view_proj = self.projection_matrix().mul(&self.view_matrix());
        let pos = view_proj.transform_point(world_pos);

        // Simple orthographic projection for now
        let ndc_x = pos.x;
        let ndc_y = pos.y;

        let screen_x = (ndc_x + 1.0) * 0.5 * screen_width;
        let screen_y = (1.0 - ndc_y) * 0.5 * screen_height;

        Some((screen_x, screen_y))
    }

    pub fn orbit_around(
        &mut self,
        target: Vector3,
        delta_yaw: f32,
        delta_pitch: f32,
        delta_zoom: f32,
    ) {
        let to_target = target - self.transform.position;
        let distance = to_target.length();

        if distance < 0.001 {
            return;
        }

        let new_distance = (distance + delta_zoom).max(0.1);
        let direction = to_target.normalized();

        let right = self.right();
        let up = self.up();

        let yaw_rotation = crate::core::Quaternion::from_axis_angle(up, delta_yaw);
        let pitch_rotation = crate::core::Quaternion::from_axis_angle(right, delta_pitch);

        let new_direction = pitch_rotation.rotate_vector(yaw_rotation.rotate_vector(direction));
        self.transform.position = target - new_direction * new_distance;

        self.transform.look_at(target, Vector3::up());
    }

    pub fn pan(&mut self, delta: Vector3) {
        self.transform.position = self.transform.position + delta;
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct CameraSettings {
    pub resolution: (u32, u32),
    pub fps: f32,
    pub pixel_aspect: f32,
}

impl CameraSettings {
    pub fn new(width: u32, height: u32, fps: f32) -> Self {
        Self {
            resolution: (width, height),
            fps,
            pixel_aspect: 1.0,
        }
    }

    pub fn hd() -> Self {
        Self::new(1920, 1080, 60.0)
    }

    pub fn fhd() -> Self {
        Self::new(1920, 1080, 60.0)
    }

    pub fn four_k() -> Self {
        Self::new(3840, 2160, 60.0)
    }

    pub fn from_aspect_ratio(width: u32, aspect: f32, fps: f32) -> Self {
        let height = (width as f32 / aspect) as u32;
        Self::new(width, height, fps)
    }
}
