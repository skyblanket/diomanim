use serde::{Deserialize, Serialize};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
    pub fn one() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
    pub fn up() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }
    pub fn right() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }
    pub fn forward() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let len = self.length();
        if len > 0.0001 {
            Self::new(self.x / len, self.y / len, self.z / len)
        } else {
            Self::zero()
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        Self::new(
            self.x + (other.x - self.x) * t,
            self.y + (other.y - self.y) * t,
            self.z + (other.z - self.z) * t,
        )
    }

    pub fn distance(&self, other: &Self) -> f32 {
        (*other - *self).length()
    }
}

impl Vector2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }
    pub fn one() -> Self {
        Self::new(1.0, 1.0)
    }
    pub fn up() -> Self {
        Self::new(0.0, 1.0)
    }
    pub fn right() -> Self {
        Self::new(1.0, 0.0)
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let len = self.length();
        if len > 0.0001 {
            Self::new(self.x / len, self.y / len)
        } else {
            Self::zero()
        }
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn angle(&self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn rotate(&self, angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self::new(self.x * cos - self.y * sin, self.x * sin + self.y * cos)
    }

    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        Self::new(
            self.x + (other.x - self.x) * t,
            self.y + (other.y - self.y) * t,
        )
    }

    pub fn distance(&self, other: &Self) -> f32 {
        (*other - *self).length()
    }
}

// Operator overloads for Vector3
impl Add for Vector3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Vector3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f32> for Vector3 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl Div<f32> for Vector3 {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        Self::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl Neg for Vector3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}

// Operator overloads for Vector2
impl Add for Vector2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vector2 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f32> for Vector2 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self::new(self.x * scalar, self.y * scalar)
    }
}

impl Div<f32> for Vector2 {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        Self::new(self.x / scalar, self.y / scalar)
    }
}

impl Neg for Vector2 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3_creation() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vector3_constants() {
        assert_eq!(Vector3::zero(), Vector3::new(0.0, 0.0, 0.0));
        assert_eq!(Vector3::one(), Vector3::new(1.0, 1.0, 1.0));
        assert_eq!(Vector3::up(), Vector3::new(0.0, 1.0, 0.0));
        assert_eq!(Vector3::right(), Vector3::new(1.0, 0.0, 0.0));
        assert_eq!(Vector3::forward(), Vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_vector3_length() {
        let v = Vector3::new(3.0, 4.0, 0.0);
        assert_eq!(v.length(), 5.0);
    }

    #[test]
    fn test_vector3_normalized() {
        let v = Vector3::new(3.0, 4.0, 0.0);
        let n = v.normalized();
        assert!((n.length() - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_vector3_dot() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(&v2), 32.0); // 1*4 + 2*5 + 3*6
    }

    #[test]
    fn test_vector3_cross() {
        let v1 = Vector3::new(1.0, 0.0, 0.0);
        let v2 = Vector3::new(0.0, 1.0, 0.0);
        let cross = v1.cross(&v2);
        assert_eq!(cross, Vector3::new(0.0, 0.0, 1.0));
    }

    #[test]
    fn test_vector3_lerp() {
        let v1 = Vector3::new(0.0, 0.0, 0.0);
        let v2 = Vector3::new(10.0, 10.0, 10.0);
        let mid = v1.lerp(&v2, 0.5);
        assert_eq!(mid, Vector3::new(5.0, 5.0, 5.0));
    }

    #[test]
    fn test_vector3_distance() {
        let v1 = Vector3::new(0.0, 0.0, 0.0);
        let v2 = Vector3::new(3.0, 4.0, 0.0);
        assert_eq!(v1.distance(&v2), 5.0);
    }

    #[test]
    fn test_vector3_add() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        assert_eq!(v1 + v2, Vector3::new(5.0, 7.0, 9.0));
    }

    #[test]
    fn test_vector3_sub() {
        let v1 = Vector3::new(5.0, 7.0, 9.0);
        let v2 = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v1 - v2, Vector3::new(4.0, 5.0, 6.0));
    }

    #[test]
    fn test_vector3_mul() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v * 2.0, Vector3::new(2.0, 4.0, 6.0));
    }

    #[test]
    fn test_vector3_div() {
        let v = Vector3::new(2.0, 4.0, 6.0);
        assert_eq!(v / 2.0, Vector3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_vector3_neg() {
        let v = Vector3::new(1.0, -2.0, 3.0);
        assert_eq!(-v, Vector3::new(-1.0, 2.0, -3.0));
    }

    #[test]
    fn test_vector2_creation() {
        let v = Vector2::new(1.0, 2.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
    }

    #[test]
    fn test_vector2_length() {
        let v = Vector2::new(3.0, 4.0);
        assert_eq!(v.length(), 5.0);
    }

    #[test]
    fn test_vector2_normalized() {
        let v = Vector2::new(3.0, 4.0);
        let n = v.normalized();
        assert!((n.length() - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_vector2_rotate() {
        let v = Vector2::new(1.0, 0.0);
        let rotated = v.rotate(std::f32::consts::PI / 2.0);
        assert!((rotated.x - 0.0).abs() < 0.0001);
        assert!((rotated.y - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_vector2_angle() {
        let v = Vector2::new(1.0, 1.0);
        assert!((v.angle() - std::f32::consts::PI / 4.0).abs() < 0.0001);
    }
}
