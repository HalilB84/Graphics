use crate::{
    interval::*,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use std::ops::Add;

//AABB (Axis aligned bounding box) is a way to specify the box containing the object that is parallel to each axis
//the idea is pretty simple, find the box that contains the object by testing min maxs / merge boxes to find bbox of miltiple objects
#[derive(Copy, Clone, Debug)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> AABB {
        let mut bbox = AABB { x: x, y: y, z: z };

        bbox.pad_to_minumums(); //the reason we pad to minumums is because

        bbox
    }

    pub fn new_point(a: Point3, b: Point3) -> AABB {
        let mut bbox = AABB {
            x: if a[0] <= b[0] {
                Interval::new(a[0], b[0])
            } else {
                Interval::new(b[0], a[0])
            },
            y: if a[1] <= b[1] {
                Interval::new(a[1], b[1])
            } else {
                Interval::new(b[1], a[1])
            },
            z: if a[2] <= b[2] {
                Interval::new(a[2], b[2])
            } else {
                Interval::new(b[2], a[2])
            },
        };

        bbox.pad_to_minumums();

        bbox
    }

    pub fn new_boxes(bbox1: &AABB, bbox2: &AABB) -> AABB {
        AABB {
            x: Interval::merge(&bbox1.x, &bbox2.x),
            y: Interval::merge(&bbox1.y, &bbox2.y),
            z: Interval::merge(&bbox1.z, &bbox2.z),
        }
    }

    pub fn axis_interval(&self, n: usize) -> &Interval {
        if n == 1 {
            return &self.y;
        }
        if n == 2 {
            return &self.z;
        }
        &self.x
    }

    //the hit function tests whether the ray intersects with the planes making the box
    //note that unless the ray is parallel to these planes it is guranteed to intersect. The case where its parallel is fine because of how division by 0 works (yields -inf or +inf dpeending on if its inside/outside)
    //at any case a NaN where t0 = 0/0 always results in false when compared, even after the padding area if the ray is magically parallel and on one of the planes it will be a miss
    //there should be no case where both t0 and t1 are NaNs bc of pad minumum

    //after getting all these t values we compare the interval each axis and if ray_tmax > ray_t.min everything is superb

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 as usize {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.min - ray_orig[axis]) * adinv;
            let t1 = (ax.max - ray_orig[axis]) * adinv;

            if t0 < t1 {
                if t0 > ray_t.min {
                    ray_t.min = t0;
                }
                if t1 < ray_t.max {
                    ray_t.max = t1;
                }
            } else {
                if t1 > ray_t.min {
                    ray_t.min = t1;
                }
                if t0 < ray_t.max {
                    ray_t.max = t0;
                }
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }

        true
    }

    pub fn longest_axis(&self) -> i64 {
        if self.x.size() > self.y.size() {
            return if self.x.size() > self.z.size() { 0 } else { 2 };
        } else {
            return if self.y.size() > self.z.size() { 1 } else { 2 };
        }
    }

    fn pad_to_minumums(&mut self) -> () {
        let delta = 0.0001;

        if self.x.size() < delta {
            self.x = self.x.expand(delta);
        }
        if self.y.size() < delta {
            self.y = self.y.expand(delta);
        }
        if self.z.size() < delta {
            self.z = self.z.expand(delta);
        }
    }

    pub const EMPTY: AABB = AABB {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };

    pub const UNIVERSE: AABB = AABB {
        x: Interval::UNIVERSE,
        y: Interval::UNIVERSE,
        z: Interval::UNIVERSE,
    };
}

impl Add<Vec3> for AABB {
    type Output = AABB;

    fn add(self, offset: Vec3) -> AABB {
        AABB {
            x: self.x + offset.x(),
            y: self.y + offset.y(),
            z: self.z + offset.z(),
        }
    }
}
