use std::f64::INFINITY;
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min: min, max: max }
    }

    pub fn merge(a: &Interval, b: &Interval) -> Interval {
        Interval {
            min: if a.min <= b.min { a.min } else { b.min },
            max: if a.max >= b.max { a.max } else { b.max },
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self, t: f64) -> bool {
        t >= self.min && t <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        x > self.min && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }

        if x > self.max {
            return self.max;
        }

        x
    }

    //the reason why we have this is because a rays direction could be 0 and also be perfectly aligned with boundary of the bbox
    //this would result in a NaN and the small padding is to prevent that (?)
    //Its not used for spheres so I dont get how this fixes stuff just yet?
    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;
        Interval {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    pub const EMPTY: Interval = Interval {
        min: INFINITY,
        max: -INFINITY,
    };

    pub const UNIVERSE: Interval = Interval {
        min: -INFINITY,
        max: INFINITY,
    };
}

impl Add<f64> for Interval {
    type Output = Interval;

    fn add(self, displacement: f64) -> Interval {
        Interval {
            min: self.min + displacement,
            max: self.max + displacement,
        }
    }
}
