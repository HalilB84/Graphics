use crate::color::*;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::utils::random_double;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

//True Lambertian Reflection
//its not explained why this is a better model of reality so reseRch
//this is a diffuse material which means that on the microscopic level the surface is rough which scatters light in a random direction
pub struct Lambertian {
    albedo: Color, //albedo is how much color is reflected (of ecah color channel)
}

impl Lambertian {
    pub fn new(color: Color) -> Lambertian {
        Lambertian { albedo: color }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        //if its a bad direction fallback to the normal
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenation = self.albedo;
        true
    }
}

//Metal surfaces instead of diffusing reflection perfectly reflects rays -> This is because at the microscopic level the surface is smooth
//we simulate fuzz by addung some randomness to the direction of the reflected ray -> if it goes back into the sphere quick, dot check
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(color: Color, fuzz: f64) -> Metal {
        Metal {
            albedo: color,
            fuzz: fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = Vec3::reflect(&r_in.direction(), &rec.normal);
        reflected = Vec3::unit_vector(&reflected) + self.fuzz * Vec3::random_unit_vector();

        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
    }
}

pub struct Dialectric {
    refraction_index: f64,
}

//things like water, glass all that shabang that light bends when it enters the material
impl Dialectric {
    pub fn new(refraction_index: f64) -> Dialectric {
        Dialectric {
            refraction_index: refraction_index,
        }
    }

    //schlick's approximation that is not explained in the book -> reseRch
    fn reflectance(&self, cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

//see refract in vec3.rs for more explanation but the high level idea is that the light bends according to its refraction index
impl Material for Dialectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = Vec3::unit_vector(&r_in.direction());
        let cos_theta = Vec3::dot(&-unit_direction, &rec.normal).min(1.0); //again the scalar is calculated with -uv because we want positive angles.
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt(); //trig identity

        //some angles cant refracts because there is no solution to the snell equation -> light has to reflect
        let cannot_refract = ri * sin_theta > 1.0;
        let direction;

        //according to the book self.reflectance needs to be used for glass? not explained more than that
        if cannot_refract || self.reflectance(cos_theta, ri) > random_double() {
            direction = Vec3::reflect(&unit_direction, &rec.normal);
        } else {
            direction = Vec3::refract(&unit_direction, &rec.normal, ri);
        }

        *scattered = Ray::new(rec.p, direction);
        *attenuation = Color::new(1.0, 1.0, 1.0);
        true
    }
}
