use std::{f64::consts::PI, rc::Rc};

use crate::{
    hittable::Hittable,
    onb::ONB,
    utils::random_double,
    vec3::{Point3, Vec3},
};

pub trait PDF {
    fn value(&self, _dir: Vec3) -> f64;

    fn generate(&self) -> Vec3;
}

pub struct SpherePDF;

impl SpherePDF {
    pub fn new() -> SpherePDF {
        SpherePDF {}
    }
}

impl PDF for SpherePDF {
    fn value(&self, _dir: Vec3) -> f64 {
        1.0 / (4.0 * PI)
    }

    fn generate(&self) -> Vec3 {
        Vec3::random_unit_vector()
    }
}

pub struct CosinePDF {
    uvw: ONB,
}

impl CosinePDF {
    pub fn new(normal: Vec3) -> CosinePDF {
        CosinePDF {
            uvw: ONB::new(normal),
        }
    }
}

impl PDF for CosinePDF {
    fn value(&self, dir: Vec3) -> f64 {
        let cos = Vec3::dot(Vec3::unit_vector(dir), self.uvw.w());
        (cos / PI).max(0.0)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.transform(Vec3::random_cosine_dir())
    }
}

pub struct HittablePDF {
    objects: Rc<dyn Hittable>,
    origin: Point3,
}

impl HittablePDF {
    pub fn new(obj: Rc<dyn Hittable>, orig: Point3) -> HittablePDF {
        HittablePDF {
            objects: obj,
            origin: orig,
        }
    }
}

impl PDF for HittablePDF {
    fn value(&self, dir: Vec3) -> f64 {
        self.objects.pdf_value(self.origin, dir)
    }

    fn generate(&self) -> Vec3 {
        self.objects.random(self.origin)
    }
}

pub struct MixturePDF {
    p: [Rc<dyn PDF>; 2],
}

impl MixturePDF {
    pub fn new(p0: Rc<dyn PDF>, p1: Rc<dyn PDF>) -> MixturePDF {
        MixturePDF { p: [p0, p1] }
    }
}

impl PDF for MixturePDF {
    fn value(&self, dir: Vec3) -> f64 {
        0.5 * self.p[0].value(dir) + 0.5 * self.p[1].value(dir)
    }

    fn generate(&self) -> Vec3 {
        if random_double() < 0.5 {
            return self.p[0].generate();
        } else {
            return self.p[1].generate();
        }
    }
}
