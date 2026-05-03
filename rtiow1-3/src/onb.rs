use crate::Vec3;

//we need this because distrubutions generate their directions in local space where its z is directly up
//however we want that dir in world space, so by equating our normal to this local z we get where it is
pub struct ONB {
    axis: [Vec3; 3],
}

impl ONB {
    pub fn new(n: Vec3) -> ONB {
        let w = Vec3::unit_vector(n);
        let a = if w.x().abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };

        let u = Vec3::unit_vector(Vec3::cross(w, a));
        let v = Vec3::cross(w, u);

        ONB { axis: [u, v, w] }
    }

    pub fn transform(&self, v: Vec3) -> Vec3 {
        v.x() * self.axis[0] + v.y() * self.axis[1] + v.z() * self.axis[2]
    }

    pub fn x(&self) -> Vec3 {
        self.axis[0]
    }
    pub fn y(&self) -> Vec3 {
        self.axis[1]
    }
    pub fn w(&self) -> Vec3 {
        self.axis[2]
    }
}
