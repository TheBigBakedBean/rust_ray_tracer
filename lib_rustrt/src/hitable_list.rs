use crate::hitable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::interval::Interval;

use std::rc::Rc;
use std::option::Option;

#[derive(Debug, Default)]
pub struct HittableList{
    objects: Vec<Rc<dyn Hittable>>
}
impl HittableList{
    pub fn new(object: Rc<dyn Hittable>) -> Self{
        let mut list = HittableList::default();
        list.add(object);
        list
    }
    pub fn add(&mut self, object: Rc<dyn Hittable>){
        self.objects.push(object);
    }
    pub fn clear(&mut self){
        self.objects.clear();
    }

    /// Returns the closest hit to the camera. If there is no hit, returns `None`
    pub fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>{
        let mut hit_rec: Option<HitRecord> = None;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects{
            match object.hit(ray, Interval::new(ray_t.min, closest_so_far)){
                Some(e) => {
                    closest_so_far = e.t;
                    hit_rec = Some(e);
                },
                None => {},
            }
        }

        hit_rec
    }
}