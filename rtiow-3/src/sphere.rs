use crate::aabb::AABB;
use crate::hittable::*;
use crate::interval::Interval;
use crate::material::Material;
use crate::onb::ONB;
use crate::ray::Ray;
use crate::utils::random_double;
use crate::vec3::{Point3, Vec3};
use std::f64::INFINITY;
use std::f64::consts::PI;
use std::rc::Rc;

pub struct Sphere {
    center: Ray,
    radius: f64,
    mat: Rc<dyn Material>,
    bbox: AABB,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Rc<dyn Material>) -> Sphere {
        let rvec = Vec3::new(radius, radius, radius);

        Sphere {
            center: Ray::new(center, Vec3::new(0., 0., 0.), 0.),
            radius: radius.max(0.),
            mat: mat,
            bbox: AABB::new_point(center - rvec, center + rvec),
        }
    }

    //a moving sphere to create motion blur. More specifically we shoot rays at random times (where the sphere is at the center at time = 0 and at the center_to at time = 1)
    pub fn new_to(center: Point3, center_to: Point3, radius: f64, mat: Rc<dyn Material>) -> Sphere {
        let rvec = Vec3::new(radius, radius, radius);
        let path = Ray::new(center, center_to - center, 0.);
        let bbox1 = AABB::new_point(path.at(0.0) - rvec, path.at(0.0) + rvec);
        let bbox2 = AABB::new_point(path.at(1.0) - rvec, path.at(1.0) + rvec);

        Sphere {
            center: path,
            radius: radius.max(0.),
            mat: mat,
            bbox: AABB::new_boxes(&bbox1, &bbox2),
        }
    }

    //point on a unit sphere to uv calculations
    //see notes for derivation
    pub fn get_sphere_uv(p: &Point3, u: &mut f64, v: &mut f64) -> () {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }

    fn random_to_sphere(radius: f64, dsq: f64) -> Vec3 {
        let r1 = random_double();
        let r2 = random_double();
        let z = 1.0 + r2 * ((1.0 - radius * radius / dsq).sqrt() - 1.0);

        let phi = 2.0 * PI * r1;
        let x = phi.cos() * (1.0 - z * z).sqrt();
        let y = phi.sin() * (1.0 - z * z).sqrt();

        Vec3::new(x, y, z)
    }
}

impl Hittable for Sphere {
    //ray-sphere intersection
    //Its understandable, at the end its just all math however I am curios on how people came up with these
    //also whenever there is a hit registered HitRecord is given all the necessary infofmartion
    //the HitRecord flow goes like: sphere -> hittalie_list -> camera -> material
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time());
        let oc: Vec3 = current_center - r.origin();
        let a = r.direction().squared_length();
        let h = Vec3::dot(r.direction(), oc);
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
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        Sphere::get_sphere_uv(&outward_normal, &mut rec.u, &mut rec.v);
        rec.mat = self.mat.clone(); //we use clone here because Rc counts references and gives a pointer back to the object, which you do by using clone().

        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }

    //needs derivation for notes
    fn pdf_value(&self, origin: Point3, dir: Vec3) -> f64 {
        let mut rec = HitRecord::new();
        if !self.hit(
            &Ray::new(origin, dir, 0.0),
            Interval::new(0.001, INFINITY),
            &mut rec,
        ) {
            return 0.0;
        }

        let dsq = (self.center.at(0.0) - origin).squared_length();
        let cos = (1.0 - self.radius * self.radius / dsq).sqrt();
        let solid = 2.0 * PI * (1.0 - cos);

        1.0 / solid
    }

    fn random(&self, origin: Vec3) -> Vec3 {
        let dir = self.center.at(0.0) - origin;
        let uvw = ONB::new(dir);

        uvw.transform(Sphere::random_to_sphere(self.radius, dir.squared_length()))
    }
}
