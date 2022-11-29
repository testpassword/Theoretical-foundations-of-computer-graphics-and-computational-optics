use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, v: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + v.x,
            y: self.y + v.y,
            z: self.z + v.z
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, v: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - v.x,
            y: self.y - v.y,
            z: self.z - v.z
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: f32) -> Self::Output {
        Vec3 {
            x: self.x * v,
            y: self.y * v,
            z: self.z * v
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, v: f32) -> Self::Output {
        Vec3 {
            x: self.x / v,
            y: self.y / v,
            z: self.z / v
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Vec3 {
    pub fn len(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(self) -> Vec3 {
        let l = self.len();
        self / l
    }

    pub fn dot(self, v: Vec3) -> f32 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn cross(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x
        }
    }

    pub fn reflect(self, v: Vec3) -> Vec3 {
        self - v * (2.0 * self.dot(v))
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }
}

impl From<(f32, f32, f32)> for Vec3 {
    fn from(t: (f32, f32, f32)) -> Self {
        Vec3 {
            x: t.0,
            y: t.1,
            z: t.2
        }
    }
}

impl From<Vec<f32>> for Vec3 {
    fn from(v: Vec<f32>) -> Self {
        let get_safe = |i| -> f32 { v.get(i).unwrap_or(&0.0).clone() };
        Vec3 {
            x: get_safe(0),
            y: get_safe(1),
            z: get_safe(2)
        }
    }
}
