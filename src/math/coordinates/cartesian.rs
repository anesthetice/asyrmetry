#[derive(Debug, Copy, Clone, Default)]
pub struct Cart3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Cart3D {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.y
    }
    pub fn dist(&self, other: Self) -> f32 {
        let delta = other - *self;
        delta.dot(delta).sqrt()
    }
}

impl From<super::Sphere3D> for Cart3D {
    fn from(value: super::Sphere3D) -> Self {
        Self {
            x: value.radius * value.theta.sin() * value.phi.cos(),
            y: value.radius * value.theta.sin() * value.phi.sin(),
            z: value.radius * value.theta.cos(),
        }
    }
}

impl std::ops::Add for Cart3D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.y)
    }
}

impl std::ops::AddAssign for Cart3D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl std::ops::Sub for Cart3D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.y)
    }
}

impl std::ops::SubAssign for Cart3D {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
