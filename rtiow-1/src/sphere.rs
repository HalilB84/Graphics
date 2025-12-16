use crate::hittable::*;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Sphere {
        Sphere {
            center: center,
            radius: radius.max(0.),
            mat: mat,
        }
    }
}

impl Hittable for Sphere {
    //ray-sphere intersection
    //Its understandable, at the end its just all math however I am curios on how people came up with these
    //also whenever there is a hit registered HitRecord is given all the necessary infofmartion
    //the HitRecord flow goes like: sphere -> hittalie_list -> camera -> material
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc: Vec3 = self.center - r.origin();
        let a = r.direction().squared_length();
        let h = Vec3::dot(&r.direction(), &oc);
        let c = oc.squared_length() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0. {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;

        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone(); //we use clone here because Rc counts references and gives a pointer back to the object, which you do by using clone().

        true
    }
}
