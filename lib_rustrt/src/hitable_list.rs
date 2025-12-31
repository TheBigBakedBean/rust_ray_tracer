use crate::hitable::{Hittable, HitRecord};
use crate::ray::Ray;

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
    pub fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>{
        let mut hit_rec: Option<HitRecord> = None;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects{
            match object.hit(ray, ray_tmin, closest_so_far){
                Some(e) => {
                    hit_rec = Some(e);
                    closest_so_far = e.t;
                },
                None => {},
            }
        }

        hit_rec
    }
}