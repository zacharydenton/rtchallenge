use crate::object::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersection {
    pub t: f32,
    pub object_id: ObjectId,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Intersections {
    pub t0: f32,
    pub t1: f32,
    iterator: usize,
}

/// A container for the two nearest intersections with a single object.
impl Intersections {
    pub fn new() -> Self {
        Intersections {
            t0: std::f32::INFINITY,
            t1: std::f32::INFINITY,
            iterator: 0,
        }
    }

    pub fn len(&self) -> usize {
        if self.t1 < std::f32::INFINITY {
            2
        } else if self.t0 < std::f32::INFINITY {
            1
        } else {
            0
        }
    }

    pub fn push(&mut self, t: f32) {
        if t < self.t0 {
            self.t1 = self.t0;
            self.t0 = t;
        } else if t < self.t1 {
            self.t1 = t;
        }
    }
}

impl Iterator for Intersections {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.iterator == 0 && self.t0 < std::f32::INFINITY {
            self.iterator = 1;
            Some(self.t0)
        } else if self.iterator == 1 && self.t1 < std::f32::INFINITY {
            self.iterator = 2;
            Some(self.t1)
        } else {
            None
        }
    }
}
