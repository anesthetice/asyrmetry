use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, Default)]
pub struct Sphere3D {
    /// greater or equal to 0
    pub radius: f32,
    /// between 0 and π
    pub theta: f32,
    /// between 0 and 2π
    pub phi: f32,
}

impl Sphere3D {
    pub fn new(radius: f32, theta: f32, phi: f32) -> Self {
        Self { radius, theta, phi }
    }
}

impl std::ops::Add for Sphere3D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            (self.radius + rhs.radius).max(0.0),
            (self.theta + rhs.theta) % PI,
            (self.phi + rhs.phi) % 2.0 * PI,
        )
    }
}
