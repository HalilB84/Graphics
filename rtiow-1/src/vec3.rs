use crate::utils::{random_double, random_double_range};
use std::ops::{Add, Div, Mul, Neg, Sub};

pub type Point3 = Vec3;

//structs are not copy by default
#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    //apparently if we don't have this it can lead to infinites and NaNs (whatever those are)
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                u.e[1] * v.e[2] - u.e[2] * v.e[1],
                u.e[2] * v.e[0] - u.e[0] * v.e[2],
                u.e[0] * v.e[1] - u.e[1] * v.e[0],
            ],
        }
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        *v / v.length()
    }

    //a random direction within a circle (not sphere as z always 0) to be multipled by a scalar in camera.rs
    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(
                random_double_range(-1., 1.),
                random_double_range(-1., 1.),
                0.,
            );
            if p.squared_length() < 1. {
                return p;
            }
        }
    }

    pub fn random() -> Vec3 {
        Vec3::new(random_double(), random_double(), random_double())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_double_range(min, max),
            random_double_range(min, max),
            random_double_range(min, max),
        )
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1., 1.);
            let lensq = p.squared_length();
            if 1e-160 < lensq && lensq <= 1. {
                return p / lensq.sqrt();
            }
        }
    }

    //Project vector v into n, here it is important that the direction of v is negated as it points into the surface, and we want positice numbers
    //Then to get the vector multipy that by the normal and at this amount twice to the v vector that goes into the surface to get the reflected vector
    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v + 2.0 * Vec3::dot(&-*v, n) * *n
    }

    //See the note for derivation:
    //https://en.wikipedia.org/wiki/Snell%27s_law#Vector_form

    //cos_theta is the magnitude of the incoming ray's component parallel to the normal
    //(cos theta because a . b = |a||b|cos theta where in this case a and b are unit vectors)
    //which is multiplied by the normal to get the vertical component of the incoming ray

    //r_out_perp is the sideways component (perpendicular to the normal hence the name) of the refracted ray calculated from the fact that sin theta  prime = (eta / eta prime) * sin theta. This eq translates to R_T prime = (eta / eta prime) (sideways component of the incoming ray) the sideways component of the incoming ray calculated by subtracting the vertical component from the total incoming ray.

    //the rest just follows the pythagorean thoerem to get the parallel component of the refracted ray
    //which then you can just sum of those two components to get the refracted ray
    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = Vec3::dot(&-*uv, n).min(1.0); //the reason why its -uv in the code is because the incoming ray and the normal point to oppsite directions -> which would give a negative value however we want the angle to be positive so we treat the incoming ray as an outgoing ray to make values work
        let r_out_perp = etai_over_etat * (*uv + cos_theta * *n); 
        let r_out_parallel = -(((1.0 - r_out_perp.squared_length()).abs()).sqrt()) * *n;
        r_out_perp + r_out_parallel
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + _rhs.e[0],
                self.e[1] + _rhs.e[1],
                self.e[2] + _rhs.e[2],
            ],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - _rhs.e[0],
                self.e[1] - _rhs.e[1],
                self.e[2] - _rhs.e[2],
            ],
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * _rhs.e[0],
                self.e[1] * _rhs.e[1],
                self.e[2] * _rhs.e[2],
            ],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, _rhs: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] * _rhs, self.e[1] * _rhs, self.e[2] * _rhs],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, _rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [self * _rhs.e[0], self * _rhs.e[1], self * _rhs.e[2]],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, _rhs: f64) -> Vec3 {
        (1.0 / _rhs) * self
    }
}
