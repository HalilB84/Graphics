use std::{f64::INFINITY, rc::Rc};

use crate::{
    color::Color,
    hittable::{HitRecord, Hittable},
    interval::*,
    material::*,
    texture::Texture,
    utils::random_double,
};

//much much much more detailed topics: https://www.scratchapixel.com/lessons/3d-basic-rendering/volume-rendering-for-developers/intro-volume-rendering.html
pub struct ConstantMedium {
    boundary: Rc<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Rc<dyn Material>,
}

impl ConstantMedium {
    pub fn new_tex(
        boundary: Rc<dyn Hittable>,
        density: f64,
        tex: Rc<dyn Texture>,
    ) -> ConstantMedium {
        ConstantMedium {
            boundary: boundary,
            neg_inv_density: -1. / density,
            phase_function: Rc::new(Isotorpic::new_tex(tex)),
        }
    }

    pub fn new(boundary: Rc<dyn Hittable>, density: f64, albedo: Color) -> ConstantMedium {
        ConstantMedium {
            boundary: boundary,
            neg_inv_density: -1. / density,
            phase_function: Rc::new(Isotorpic::new(albedo)),
        }
    }
}

//only works for convex shapes
//the current code checks for  2 intersection points but a hollow sphere might have 4
//if the ray doesnt bounce in the original half of the hollow sphere we assume it skips the whole volume which is not the case
impl Hittable for ConstantMedium {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let mut rec1: HitRecord = HitRecord::new();
        let mut rec2: HitRecord = HitRecord::new();

        if !self.boundary.hit(r, Interval::UNIVERSE, &mut rec1) {
            return false;
        }

        if !self
            .boundary
            .hit(r, Interval::new(rec1.t + 0.0001, INFINITY), &mut rec2)
        {
            return false;
        }

        if rec1.t >= rec2.t {
            return false;
        }

        if rec1.t < ray_t.min {
            rec1.t = ray_t.min;
        }

        if rec2.t > ray_t.max {
            rec2.t = ray_t.max;
        }

        if rec1.t < 0. {
            rec1.t = 0.;
        }

        //because rec2.t - rec1.t is not accurate (beacuse td where d might be different)
        //we get the length of the direction vector, multiply the t distance to get the actual distance
        //then according to the density random double ln formula the ray either boucess at some point in the cloud or passes through with not hits
        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;

        //from what I currently understand every small step you take in the volume has a probability of scattering
        //instead of moving each steps we find the scatter point once randomly.
        //Because each step has a probability of scattering each step it survives multiplies, which the overall value drops exponentially
        //here we use ln to model that scatter distance
        let hit_distance = self.neg_inv_density * random_double().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.mat = self.phase_function.clone();
        true
    }

    fn bounding_box(&self) -> crate::aabb::AABB {
        self.boundary.bounding_box()
    }
}
