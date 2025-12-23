use crate::aabb::AABB;
use crate::interval::*;
use crate::material::Lambertian;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::f64::INFINITY;
use std::mem::offset_of;
use std::rc::Rc;

//interface (java) equivalent in rust
pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> AABB;
}

//https://stackoverflow.com/questions/49834414/what-is-the-rust-equivalent-of-cs-shared-ptr
#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            mat: Rc::new(Lambertian::new(Vec3::new(0., 0., 0.))),
            t: 0.,
            u: 0.,
            v: 0.,
            front_face: false,
        }
    }
    //we want the normal to be always against the ray, which is why there is a dot check
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) -> () {
        let front_face = Vec3::dot(r.direction(), outward_normal) < 0.;
        self.front_face = front_face;
        self.normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

//I initially thought that moving the ray instead of the object was bad and less intuitive then I realized that
//there is no easy way of moving/rotating all the points and returning the same object with all other fields kept
//It is certainly possible but this is the minimal and smart solution

pub struct Translate {
    object: Rc<dyn Hittable>,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: Rc<dyn Hittable>, offset: Vec3) -> Translate {
        let bbox = object.bounding_box() + offset;

        Translate {
            object: object,
            offset: offset,
            bbox: bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let offset_r = Ray::new(r.origin() - self.offset, r.direction(), r.time());

        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }

        rec.p = rec.p + self.offset;

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub struct RotateY {
    object: Rc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl RotateY {
    pub fn new(object: Rc<dyn Hittable>, angle: f64) -> RotateY {
        let radians = angle.to_radians();

        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1. - i as f64) * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1. - j as f64) * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1. - k as f64) * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let test = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(test[c]);
                        max[c] = max[c].max(test[c]);
                    }
                }
            }
        }

        RotateY {
            object: object,
            sin_theta: sin_theta,
            cos_theta: cos_theta,
            bbox: AABB::new_point(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let origin = Point3::new(
            (self.cos_theta * r.origin().x()) - (self.sin_theta * r.origin().z()),
            r.origin().y(),
            (self.sin_theta * r.origin().x()) + (self.cos_theta * r.origin().z()),
        );

        let direction = Vec3::new(
            (self.cos_theta * r.direction().x()) - (self.sin_theta * r.direction().z()),
            r.direction().y(),
            (self.sin_theta * r.direction().x()) + (self.cos_theta * r.direction().z()),
        );

        let rotated_r = Ray::new(origin, direction, r.time());

        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        rec.p = Point3::new(
            (self.cos_theta * rec.p.x()) + (self.sin_theta * rec.p.z()),
            rec.p.y(),
            (-self.sin_theta * rec.p.x()) + (self.cos_theta * rec.p.z()),
        );

        rec.normal = Vec3::new(
            (self.cos_theta * rec.normal.x()) + (self.sin_theta * rec.normal.z()),
            rec.normal.y(),
            (-self.sin_theta * rec.normal.x()) + (self.cos_theta * rec.normal.z()),
        );

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
