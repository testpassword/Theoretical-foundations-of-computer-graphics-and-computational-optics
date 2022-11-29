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

    // Moller-Trumbore algorithm
    pub fn intersected(&self, ray: &Ray, t: f32) -> (bool, f32) {
        // todo: be careful not tested
        let e1 = self.vertices.1 - self.vertices.0;
        let e2 = self.vertices.2 - self.vertices.0;
        let pvec = ray.direction.cross(e2);
        let det = e1.dot(pvec);
        if det < Polygon::ACCURACY && det > -Polygon::ACCURACY { (false, t); }
        let inv_det = 1.0 / det;
        let tvec = ray.origin - self.vertices.0;
        let u = tvec.dot(pvec) * inv_det;
        if u < 0.0 || u > 1.0 { (false, t); }
        let qvec = tvec.cross(e1);
        let v = ray.direction.dot(qvec) * inv_det;
        if v < 0.0 || v + u > 1.0 { (false, t); }
        let n_t = e2.dot(qvec) * inv_det;
        (n_t > Polygon::ACCURACY, n_t)
    }
}
