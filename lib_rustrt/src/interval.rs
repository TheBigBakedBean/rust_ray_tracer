use crate::INFINITY;

#[derive(Debug, Clone, Copy)]
pub struct Interval{
    pub min: f64,
    pub max: f64,
}

impl Interval{
    pub const fn new(min: f64, max: f64)->Self{
        Self{ min, max }
    }
    pub const fn size(&self) -> f64{
        self.max - self.min
    }
    pub const fn contains(&self, x: f64) -> bool{
        self.min <= x && x <= self.max
    }
    pub const fn surrounds(&self, x: f64) -> bool{
        self.min < x && x < self.max
    }
    pub const fn clamp(&self, x: f64) -> f64{
        if x < self.min {self.min} else if x > self.max {self.max} else {x}
    }
}

impl Default for Interval{
    fn default()->Self{
        Self{
            min: INFINITY,
            max: -INFINITY,
        }
    }
}

pub const EMPTY: Interval = Interval::new(INFINITY, -INFINITY);
pub const UNIVERSE: Interval = Interval::new(-INFINITY, INFINITY);