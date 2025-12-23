use crate::aabb::AABB;
use crate::hittable::*;
use crate::interval::Interval;
use crate::ray::Ray;
use std::rc::Rc;

//Rc puts the object on heap because rust needs to know the size of the object inside vec at compile time which is not the case for hittable.
pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    bbox: AABB,
}

//hittable list is quite literally the world. It contains all the hittable objects hence the name
impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
            bbox: AABB {
                x: Interval::EMPTY,
                y: Interval::EMPTY,
                z: Interval::EMPTY,
            },
        }
    }

    pub fn new_list(object: Rc<dyn Hittable>) -> HittableList {
        let mut objects = Vec::new();
        objects.push(object);
        HittableList {
            objects: objects,
            bbox: AABB {
                x: Interval::EMPTY,
                y: Interval::EMPTY,
                z: Interval::EMPTY,
            },
        }
    }

    pub fn add(&mut self, obj: Rc<dyn Hittable>) -> () {
        self.bbox = AABB::new_boxes(&self.bbox, &obj.bounding_box());
        self.objects.push(obj);
    }

    pub fn clear(&mut self) -> () {
        self.objects.clear();
    }
}

//the world propages the hit function to all of the spheres and returns the closest one
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
