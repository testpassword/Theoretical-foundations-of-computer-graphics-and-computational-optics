use std::f32::consts::PI;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Polygon<'s> {
    pub vertices: (Vec3, Vec3, Vec3),
    pub material: &'s Material,
}

impl Polygon<'_> {
    const ACCURACY: f32 = 1e-8;

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

    // https://ru.wikipedia.org/wiki/%D0%90%D0%BB%D0%B3%D0%BE%D1%80%D0%B8%D1%82%D0%BC_%D0%9C%D0%BE%D0%BB%D0%BB%D0%B5%D1%80%D0%B0_%E2%80%94_%D0%A2%D1%80%D1%83%D0%BC%D0%B1%D0%BE%D1%80%D0%B0
    pub fn intersected(&self, ray: &Ray, t: f32) -> (bool, f32) {
        // todo: be careful not tested
        let e1 = self.vertices.1 - self.vertices.0;
        let e2 = self.vertices.2 - self.vertices.0;
        let p_vec = ray.direction.cross(e2);
        let det = e1.dot(p_vec);
        if det < Polygon::ACCURACY && det > -Polygon::ACCURACY { (false, t); }
        let inv_det = 1.0 / det;
        let t_vec = ray.origin - self.vertices.0;
        let u = t_vec.dot(p_vec) * inv_det;
        if u < 0.0 || u > 1.0 { (false, t); }
        let q_vec = t_vec.cross(e1);
        let v = ray.direction.dot(q_vec) * inv_det;
        if v < 0.0 || v + u > 1.0 { (false, t); }
        let n_t = e2.dot(q_vec) * inv_det;
        (n_t > Polygon::ACCURACY, n_t)
    }
}
