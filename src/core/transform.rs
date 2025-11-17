use crate::core::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Transform {
    pub position: Vector3,
    pub rotation: Quaternion,
    pub scale: Vector3,
}

impl Transform {
    pub fn new() -> Self {
        Self {
            position: Vector3::zero(),
            rotation: Quaternion::identity(),
            scale: Vector3::one(),
        }
    }

    pub fn from_position(position: Vector3) -> Self {
        Self {
            position,
            rotation: Quaternion::identity(),
            scale: Vector3::one(),
        }
    }

    pub fn from_translation(x: f32, y: f32, z: f32) -> Self {
        Self::from_position(Vector3::new(x, y, z))
    }

    pub fn with_rotation(mut self, rotation: Quaternion) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_scale(mut self, scale: Vector3) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_uniform_scale(mut self, scale: f32) -> Self {
        self.scale = Vector3::new(scale, scale, scale);
        self
    }
    
    pub fn scale_uniform(&mut self, scale: f32) {
        self.scale = Vector3::new(scale, scale, scale);
    }

    pub fn translate(&mut self, offset: Vector3) {
        self.position = self.position + offset;
    }

    pub fn rotate(&mut self, mut rotation: Quaternion) {
        // Combine rotations by multiplication
        let q2 = self.rotation;
        
        // Quaternion multiplication: q1 * q2
        let w = rotation.w * q2.w - rotation.x * q2.x - rotation.y * q2.y - rotation.z * q2.z;
        let x = rotation.w * q2.x + rotation.x * q2.w + rotation.y * q2.z - rotation.z * q2.y;
        let y = rotation.w * q2.y - rotation.x * q2.z + rotation.y * q2.w + rotation.z * q2.x;
        let z = rotation.w * q2.z + rotation.x * q2.y - rotation.y * q2.x + rotation.z * q2.w;
        
        rotation.x = x;
        rotation.y = y;
        rotation.z = z;
        rotation.w = w;
        rotation.normalize();
        
        self.rotation = rotation;
    }

    pub fn look_at(&mut self, target: Vector3, up: Vector3) {
        let forward = (target - self.position).normalized();
        let right = up.cross(&forward).normalized();
        let up = forward.cross(&right);
        
        self.rotation = Quaternion::from_basis_vectors(&right, &up, &forward);
    }

    pub fn matrix(&self) -> Matrix4 {
        let translation = Matrix4::from_translation(self.position);
        let rotation = Matrix4::from_quaternion(self.rotation);
        let scale = Matrix4::from_scale(self.scale);
        
        translation * rotation * scale
    }

    pub fn inverse(&self) -> Self {
        let inv_rotation = self.rotation.inverse();
        let inv_scale = Vector3::new(1.0 / self.scale.x, 1.0 / self.scale.y, 1.0 / self.scale.z);
        let rotated = inv_rotation.rotate_vector(self.position * (-1.0));
        let inv_position = Vector3::new(rotated.x * inv_scale.x, rotated.y * inv_scale.y, rotated.z * inv_scale.z);
        
        Self {
            position: inv_position,
            rotation: inv_rotation,
            scale: inv_scale,
        }
    }

    pub fn lerp(&self, other: &Transform, t: f32) -> Self {
        Self {
            position: self.position.lerp(&other.position, t),
            rotation: self.rotation.slerp(&other.rotation, t),
            scale: self.scale.lerp(&other.scale, t),
        }
    }

    pub fn transform_point(&self, point: Vector3) -> Vector3 {
        let scaled = Vector3::new(point.x * self.scale.x, point.y * self.scale.y, point.z * self.scale.z);
        self.rotation.rotate_vector(scaled) + self.position
    }

    pub fn transform_vector(&self, vector: Vector3) -> Vector3 {
        let scaled = Vector3::new(vector.x * self.scale.x, vector.y * self.scale.y, vector.z * self.scale.z);
        self.rotation.rotate_vector(scaled)
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        let len = (x * x + y * y + z * z + w * w).sqrt();
        Self {
            x: x / len,
            y: y / len,
            z: z / len,
            w: w / len,
        }
    }

    pub fn identity() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }

    pub fn from_axis_angle(axis: Vector3, angle: f32) -> Self {
        let angle = angle * 0.5;
        let sin_angle = angle.sin();
        let cos_angle = angle.cos();
        
        Self::new(
            axis.x * sin_angle,
            axis.y * sin_angle,
            axis.z * sin_angle,
            cos_angle,
        )
    }

    pub fn from_euler_angles(roll: f32, pitch: f32, yaw: f32) -> Self {
        let (sr, cr) = (roll * 0.5).sin_cos();
        let (sp, cp) = (pitch * 0.5).sin_cos();
        let (sy, cy) = (yaw * 0.5).sin_cos();

        Self::new(
            sr * cp * cy - cr * sp * sy,
            cr * sp * cy + sr * cp * sy,
            cr * cp * sy - sr * sp * cy,
            cr * cp * cy + sr * sp * sy,
        )
    }

    pub fn from_basis_vectors(right: &Vector3, up: &Vector3, forward: &Vector3) -> Self {
        let trace = right.x + up.y + forward.z;
        
        if trace > 0.0 {
            let s = 0.5 / (trace + 1.0).sqrt();
            Self::new(
                (up.z - forward.y) * s,
                (forward.x - right.z) * s,
                (right.y - up.x) * s,
                0.25 / s,
            )
        } else if right.x > up.y && right.x > forward.z {
            let s = 2.0 * (1.0 + right.x - up.y - forward.z).sqrt();
            Self::new(
                0.25 * s,
                (up.x + right.y) / s,
                (forward.x + right.z) / s,
                (up.z - forward.y) / s,
            )
        } else if up.y > forward.z {
            let s = 2.0 * (1.0 + up.y - right.x - forward.z).sqrt();
            Self::new(
                (up.x + right.y) / s,
                0.25 * s,
                (forward.y + up.z) / s,
                (forward.x - right.z) / s,
            )
        } else {
            let s = 2.0 * (1.0 + forward.z - right.x - up.y).sqrt();
            Self::new(
                (forward.x + right.z) / s,
                (forward.y + up.z) / s,
                0.25 * s,
                (right.y - up.x) / s,
            )
        }
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn conjugate(&self) -> Self {
        Self::new(-self.x, -self.y, -self.z, self.w)
    }

    pub fn inverse(&self) -> Self {
        let len_sq = self.length().powi(2);
        Self::new(
            -self.x / len_sq,
            -self.y / len_sq,
            -self.z / len_sq,
            self.w / len_sq,
        )
    }

    pub fn normalize(&mut self) {
        let len = self.length();
        if len > 0.0001 {
            self.x /= len;
            self.y /= len;
            self.z /= len;
            self.w /= len;
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn slerp(&self, other: &Self, t: f32) -> Self {
        let mut dot = self.dot(other);
        let mut other = *other;

        if dot < 0.0 {
            dot = -dot;
            other = Self::new(-other.x, -other.y, -other.z, -other.w);
        }

        let t = t.clamp(0.0, 1.0);

        if dot > 0.9995 {
            return Self::new(
                self.x + t * (other.x - self.x),
                self.y + t * (other.y - self.y),
                self.z + t * (other.z - self.z),
                self.w + t * (other.w - self.w),
            ).normalized();
        }

        let theta_0 = dot.acos();
        let theta = theta_0 * t;
        let sin_theta = theta.sin();
        let sin_theta_0 = theta_0.sin();

        let s0 = ((1.0 - t) * theta).cos() - dot * sin_theta / sin_theta_0;
        let s1 = sin_theta / sin_theta_0;

        Self::new(
            self.x * s0 + other.x * s1,
            self.y * s0 + other.y * s1,
            self.z * s0 + other.z * s1,
            self.w * s0 + other.w * s1,
        )
    }

    pub fn normalized(&self) -> Self {
        let mut q = *self;
        q.normalize();
        q
    }

    pub fn rotate_vector(&self, vector: Vector3) -> Vector3 {
        let q_vec = Vector3::new(self.x, self.y, self.z);
        let t = q_vec.cross(&vector) * 2.0;
        let u = q_vec.cross(&t) * 2.0;
        
        vector + t * self.w + u
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Matrix4 {
    pub data: [[f32; 4]; 4],
}

impl Matrix4 {
    pub fn identity() -> Self {
        Self {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn from_translation(translation: Vector3) -> Self {
        let mut m = Self::identity();
        m.data[0][3] = translation.x;
        m.data[1][3] = translation.y;
        m.data[2][3] = translation.z;
        m
    }

    pub fn from_scale(scale: Vector3) -> Self {
        let mut m = Self::identity();
        m.data[0][0] = scale.x;
        m.data[1][1] = scale.y;
        m.data[2][2] = scale.z;
        m
    }

    pub fn from_quaternion(q: Quaternion) -> Self {
        let x2 = q.x + q.x;
        let y2 = q.y + q.y;
        let z2 = q.z + q.z;
        let xx2 = q.x * x2;
        let yy2 = q.y * y2;
        let zz2 = q.z * z2;
        let xy2 = q.x * y2;
        let xz2 = q.x * z2;
        let yz2 = q.y * z2;
        let wx2 = q.w * x2;
        let wy2 = q.w * y2;
        let wz2 = q.w * z2;

        Self {
            data: [
                [1.0 - (yy2 + zz2), xy2 + wz2, xz2 - wy2, 0.0],
                [xy2 - wz2, 1.0 - (xx2 + zz2), yz2 + wx2, 0.0],
                [xz2 + wy2, yz2 - wx2, 1.0 - (xx2 + yy2), 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn mul(&self, other: &Self) -> Self {
        let mut result = Self::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = 
                    self.data[i][0] * other.data[0][j] +
                    self.data[i][1] * other.data[1][j] +
                    self.data[i][2] * other.data[2][j] +
                    self.data[i][3] * other.data[3][j];
            }
        }
        result
    }

    pub fn transform_point(&self, point: Vector3) -> Vector3 {
        let x = self.data[0][0] * point.x + self.data[0][1] * point.y + self.data[0][2] * point.z + self.data[0][3];
        let y = self.data[1][0] * point.x + self.data[1][1] * point.y + self.data[1][2] * point.z + self.data[1][3];
        let z = self.data[2][0] * point.x + self.data[2][1] * point.y + self.data[2][2] * point.z + self.data[2][3];
        Vector3::new(x, y, z)
    }
}

impl Default for Matrix4 {
    fn default() -> Self {
        Self::identity()
    }
}

// Operator overloads for matrices
impl std::ops::Mul for Matrix4 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        self.mul_ref(&other)
    }
}

impl Matrix4 {
    pub fn mul_ref(&self, other: &Self) -> Self {
        let mut result = Self::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = 
                    self.data[i][0] * other.data[0][j] +
                    self.data[i][1] * other.data[1][j] +
                    self.data[i][2] * other.data[2][j] +
                    self.data[i][3] * other.data[3][j];
            }
        }
        result
    }
}