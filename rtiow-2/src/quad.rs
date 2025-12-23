use std::rc::Rc;

use crate::{
    aabb::AABB,
    hittable::*,
    hittable_list::*,
    material::Material,
    vec3::{Point3, Vec3},
};
use crate::{hittable::*, interval::*, ray::*};

//See notes for the derivation
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Rc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Rc<dyn Material>) -> Quad {
        let bbox_diagonal1 = AABB::new_point(q, q + u + v);
        let bbox_diagonal2 = AABB::new_point(q + u, q + v);

        let bbox = AABB::new_boxes(&bbox_diagonal1, &bbox_diagonal2);

        let n = Vec3::cross(u, v);
        let normal = Vec3::unit_vector(n);
        let d = Vec3::dot(normal, q);
        let w = normal / Vec3::dot(normal, n);

        Quad {
            q: q,
            u: u,
            v: v,
            w: w,
            mat: mat,
            bbox: bbox,
            normal: normal,
            d: d,
        }
    }

    pub fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0., 1.);

        if !unit_interval.contains(a) || !unit_interval.contains(b) {
            return false;
        }

        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom = Vec3::dot(self.normal, r.direction());

        if denom.abs() < 1e-8 {
            return false;
        }

        let t = (self.d - Vec3::dot(self.normal, r.origin())) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);

        let planar_hitpt_vector = intersection - self.q;
        let alpha = Vec3::dot(self.w, Vec3::cross(planar_hitpt_vector, self.v));
        let beta = Vec3::dot(self.w, Vec3::cross(self.u, planar_hitpt_vector));

        if !Quad::is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.u = alpha;
        rec.v = beta;
        rec.mat = self.mat.clone();
        rec.set_face_normal(r, self.normal);

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

//constructs a HittableList hittable. This is literally like a new primitive but it just a fake that propages the hit function to the sides
//I hate that this works
pub fn boxx(a: Point3, b: Point3, mat: Rc<dyn Material>) -> Rc<dyn Hittable> {
    let mut sides = HittableList::new();

    let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3::new(max.x() - min.x(), 0., 0.);
    let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

    sides.add(Rc::new(Quad::new(
        Point3::new(min.x(), min.y(), max.z()),
        dx,
        dy,
        mat.clone(),
    )));

    sides.add(Rc::new(Quad::new(
        Point3::new(max.x(), min.y(), max.z()),
        -dz,
        dy,
        mat.clone(),
    )));

    sides.add(Rc::new(Quad::new(
        Point3::new(max.x(), min.y(), min.z()),
        -dx,
        dy,
        mat.clone(),
    )));

    sides.add(Rc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dz,
        dy,
        mat.clone(),
    )));

    sides.add(Rc::new(Quad::new(
        Point3::new(min.x(), max.y(), max.z()),
        dx,
        -dz,
        mat.clone(),
    )));

    sides.add(Rc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dx,
        dz,
        mat.clone(),
    )));

    Rc::new(sides)
}
