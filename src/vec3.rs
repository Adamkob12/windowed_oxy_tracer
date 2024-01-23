use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct V3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

//useful vector functions

impl V3 {
    pub fn length(&self) -> f64 {
        f64::sqrt(self.x.powi(2) + self.y.powi(2) + self.z.powi(2))
    }

    pub fn normalize(&self) -> Self {
        *self / self.length()
    }
}

impl ops::Add for V3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub for V3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul<f64> for V3 { //multiply vector by scalar
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl ops::Mul<V3> for f64 { //multiply scalar by vector
    type Output = V3;

    fn mul(self, vector: V3) -> V3 {
        V3 {
            x: vector.x * self,
            y: vector.y * self,
            z: vector.z * self,
        }
    }
}

impl ops::Div<f64> for V3 { //multiply vector by scalar
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Self {
            x: self.x * (1.0 /scalar),
            y: self.y * (1.0 /scalar),
            z: self.z * (1.0 / scalar),
        }
    }
}

pub fn dot(v: V3, w: V3) -> f64{
    v.x * w.x + v.y * w.y + v.z * w.z
} 

pub fn cross(v: V3, w: V3) -> V3 {
    V3 {
        x: v.y * w.z - v.z * w.y,
        y: v.z * w.x - v.x * w.z,
        z: v.x * w.y - v.y * w.x,
    }
}

pub fn new_vec(x: f64, y: f64, z: f64) -> V3 {
    V3 {
        x: x,
        y: y,
        z: z,
    }
}
