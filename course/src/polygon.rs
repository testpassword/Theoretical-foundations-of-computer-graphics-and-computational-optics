use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Polygon {
    pub vertices: (Vec3, Vec3, Vec3),
    pub material: Material,
}

impl Polygon {
    pub fn normal(&self) -> Vec3 {
        (self.vertices.1 - self.vertices.0).cross(self.vertices.2 - self.vertices.0).normalize()
    }

    pub fn normal_by_observer(&self, observer: Vec3) -> Vec3 {
        let normal = self.normal();
        if observer.normalize().dot(normal) < 0.0 {
            (self.vertices.2 - self.vertices.0).cross(self.vertices.1 - self.vertices.0).normalize()
        } else {
            normal
        }
    }

    pub fn intersected(&self, ray: Ray, t: f32) -> bool {
        let e1 = self.vertices.1 - self.vertices.0;
        let e2 = self.vertices.2 - self.vertices.0;
        let pVec = ray.direction.cross(e2);
        let det = e1.dot(pVec);
        if det < 1e-8 && det > -1e-8 {
            false
        }
        todo!()
    }
}
