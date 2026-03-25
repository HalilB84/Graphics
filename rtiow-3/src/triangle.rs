use crate::{Material, Point3, aabb::AABB, hittable::*, interval::Interval, ray::*, vec3::Vec3};
use std::rc::Rc;

pub struct Triangle {
    a: Point3,
    b: Point3,
    c: Point3,
    mat: Rc<dyn Material>,
    bbox: AABB,
}

impl Triangle {
    pub fn new(a: Point3, b: Point3, c: Point3, mat: Rc<dyn Material>) -> Triangle {
        let x_interval: Interval =
            Interval::new(a[0].min(b[0]).min(c[0]), a[0].max(b[0]).max(c[0]));
        let y_interval: Interval =
            Interval::new(a[1].min(b[1]).min(c[1]), a[1].max(b[1]).max(c[1]));
        let z_interval: Interval =
            Interval::new(a[2].min(b[2]).min(c[2]), a[2].max(b[2]).max(c[2]));

        let bbox = AABB::new(x_interval, y_interval, z_interval);

        Triangle {
            a: a,
            b: b,
            c: c,
            mat: mat,
            bbox: bbox,
        }
    }
}

//https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
//to be added to notes
impl Hittable for Triangle {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let e1 = self.b - self.a;
        let e2 = self.c - self.a;

        let ray_cross_e2 = Vec3::cross(r.direction(), e2);
        let det = Vec3::dot(e1, ray_cross_e2);

        if det.abs() < f64::EPSILON {
            return false;
        }

        let inv_det = 1.0 / det;
        let s = r.origin() - self.a;
        let u = inv_det * Vec3::dot(s, ray_cross_e2);
        if u < 0.0 || u > 1.0 {
            return false;
        }

        let s_cross_e1 = Vec3::cross(s, e1);
        let v = inv_det * Vec3::dot(r.direction(), s_cross_e1);
        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        let t = inv_det * Vec3::dot(e2, s_cross_e1);

        if t > f64::EPSILON && ray_t.contains(t) {
            rec.t = t;
            rec.p = r.origin() + r.direction() * t;
            rec.u = u;
            rec.v = v;
            rec.mat = self.mat.clone();
            rec.set_face_normal(r, Vec3::unit_vector(Vec3::cross(e1, e2)));

            return true;
        }

        false
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
