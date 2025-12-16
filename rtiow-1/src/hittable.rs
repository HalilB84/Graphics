use std::rc::Rc;
use crate::interval::*;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

//https://stackoverflow.com/questions/49834414/what-is-the-rust-equivalent-of-cs-shared-ptr
#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Rc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3::new(0., 0., 0.),
            normal: Vec3::new(0., 0., 0.),
            mat: Rc::new(crate::material::Lambertian::new(Vec3::new(0., 0., 0.))),
            t: 0.,
            front_face: false,
        }
    }
    //we want the normal to be always against the ray, which is why there is a dot check
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) -> () {
        let front_face = Vec3::dot(&r.direction(), &outward_normal) < 0.;
        self.front_face = front_face;
        self.normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}
//interface (java) equivalent in rust
pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}
