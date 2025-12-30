//this file is really smart about the way it manages stuff

use crate::{aabb::AABB, hittable::*, hittable_list::HittableList, interval::Interval, ray::*};
use std::cmp::Ordering;
use std::rc::Rc;

pub struct BvhNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

impl BvhNode {
    pub fn new(mut list: HittableList) -> BvhNode {
        let len = list.objects.len();
        BvhNode::actual(&mut list.objects, 0, len) //if list is mutable then list.objects are also mutable (https://doc.rust-lang.org/book/ch05-01-defining-structs.html)
    }

    fn actual(objects: &mut Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> BvhNode {
        let mut bbox = AABB::EMPTY;

        for object_index in start..end {
            bbox = AABB::new_boxes(&bbox, &objects[object_index].bounding_box());
        }

        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => BvhNode::box_x_compare,
            1 => BvhNode::box_y_compare,
            _ => BvhNode::box_z_compare,
        };

        let object_span = end - start;

        let left: Rc<dyn Hittable>;
        let right: Rc<dyn Hittable>;

        //ok so visualize the BVH tree, the root note is the node that covers all the objects on the scene
        //each node has 2 child nodes, left and right. left and right is determined by choosing a random axis, sorting objects about that axis
        //and then putting half to left and the other half to right. You do this recursively untill you reach individual spheres
        //now here its important to know that in the case of 3 nodes being left to divide we put 2 to one 1 to the other.
        //well how do we deal with the 1 node that is left? We established that BvhNode needs left and right to be constructed.
        //this is why when there is 1 node left we copy the node itself as its children and end the recursion there.

        //also note that when we reach the leaf, left and right are the Sphere hittables
        //while all the other left and rights are BvhNode hittables
        //why is this important? --> see comments on the hit function below
        if object_span == 1 {
            left = objects[start].clone(); //increment referernce to the sphere 
            right = objects[start].clone(); //either sphere, quad or hittable grouping of such primitives (HittableList)
        } else if object_span == 2 {
            left = objects[start].clone();
            right = objects[start + 1].clone();
        } else {
            objects[start..end].sort_by(|a, b| comparator(a, b));

            let mid = start + object_span / 2;

            //these recursive calls are fine because the mutable reference is just passed down not created again
            left = Rc::new(BvhNode::actual(objects, start, mid));
            right = Rc::new(BvhNode::actual(objects, mid, end));
        }

        //also here note that since the leafmost left and rights are spheres thier boundingbox is called that build the whole tree.
        let bbox1 = left.bounding_box();
        let bbox2 = right.bounding_box();

        BvhNode {
            left: left,
            right: right,
            bbox: AABB::new_boxes(&bbox1, &bbox2),
        }
    }

    //reminder that things are sorted by one corner of the box, meaning the bbox can overlap if an pbjects bbox leaks into to neighborhing bbox
    fn box_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>, axis_index: usize) -> Ordering {
        let bbox1 = a.bounding_box();
        let bbox2 = b.bounding_box();

        let a_axis_interval = bbox1.axis_interval(axis_index);
        let b_axis_interval = bbox2.axis_interval(axis_index);

        if a_axis_interval.min < b_axis_interval.min {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    fn box_x_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        BvhNode::box_compare(a, b, 0)
    }

    fn box_y_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        BvhNode::box_compare(a, b, 1)
    }

    fn box_z_compare(a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>) -> Ordering {
        BvhNode::box_compare(a, b, 2)
    }
}

//when the hit function is called for the root node of the BVH tree in hittable_list
//this function recurses all the way down to the leaf. But remember the leaf left and rights are other Hittables not BvhNodes!
//this means that other_hittable.hit() is called instead of this function. This is why it works
impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, ray_t) {
            //bbox also uses ray_t as we dont care about bboxes further if we hit something closer
            return false;
        }

        //if we hit the left box, pass in rec.t as the max for the right hit to not overwrite rec
        let hit_left = self.left.hit(r, ray_t, rec);
        let t_max = if hit_left { rec.t } else { ray_t.max };
        let hit_right = self.right.hit(r, Interval::new(ray_t.min, t_max), rec);

        hit_left || hit_right
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
