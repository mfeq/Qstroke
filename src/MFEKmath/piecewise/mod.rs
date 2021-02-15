use consts::SMALL_T_DISTANCE;
use glifparser::*;

pub mod glif;
mod skia;
mod evaluate;

use super::*;
use super::consts::SMALL_DISTANCE;
use super::vector::*;
use super::bezier::Bezier;
use super::evaluate::{Evaluate, Primitive};


// This struct models a simple piecewise function. It maps 0-1 such that 0 is the beginning of the first curve
// in the collection and 1 is the end of the last.
pub struct Piecewise<T: Evaluate> {
    // This supports 
    pub cuts: Vec<f64>,
    // this should definitely change to private at some point with an iterator or getter to access
    pub segs: Vec<T>
}

impl<T: Evaluate> Piecewise<T> {
    pub fn new(segs: Vec<T>, _cuts: Option<Vec<f64>>) -> Self
    {
        match _cuts {
            Some(cuts) => {
                return Self {
                    cuts: cuts,
                    segs: segs
                }
            }
            
            // if we are given just a list of segments we generate the cuts ourselves
            _ => {
                let mut out_cuts: Vec<f64> = Vec::new();

                out_cuts.push(0.);

                let seg_iter = segs.iter().enumerate().peekable();
                let seg_len = segs.len();

                for (i, seg) in seg_iter {
                    out_cuts.push((i+1) as f64 / seg_len as f64);
                }

                return Self {
                    cuts: out_cuts,
                    segs: segs
                }
            }
        }
    }

    // implementation ripped from lib2geom, performs a binary search to find our segment
    fn seg_n(&self, t: f64) -> usize
    {
        let (mut low, mut high) = (0, self.segs.len() - 1);
        if t < *self.cuts.first().unwrap() { return 0; }
        if t > *self.cuts.last().unwrap() { return self.segs.len() - 1; }

        while low < high {
            let mid = (high + low) / 2;
            let mv = self.cuts[mid];

            if mv < t {
                if t < self.cuts[mid+1] { return mid; } else { low = mid + 1; }
            } else if  t < mv {
                if self.cuts[mid-1] < t { return mid; } else { high = mid - 1; }
            } else {
                return mid;
            }
        }

        return low;
    }

    fn seg_t(&self, t: f64) -> f64 
    {
        let i = self.seg_n(t);
        return (t - self.cuts[i]) / (self.cuts[i+1] - self.cuts[i]);
    }
}

// TODO: Move these functions to a more appropriate submodule.
impl<T: Evaluate<EvalResult = Vector>+Primitive> Piecewise<Piecewise<T>>
{
    // we split the primitive that contains t at t
    pub fn subdivide(&self, t: f64) -> Self
    {
        let mut output = Vec::new();
        for contour in &self.segs {
            output.push(contour.subdivide(t));
        }

        return Piecewise::new(output, None);
    }
}

impl<T: Evaluate<EvalResult = Vector>+Primitive> Piecewise<T>
{    
    pub fn is_closed(&self) -> bool
    {
        if self.start_point().is_near(self.end_point(),SMALL_DISTANCE)
        {
            return true;
        }
        return false;
    }
    
    pub fn subdivide(&self, t: f64) -> Piecewise<T>
    {
        let mut new_segments = Vec::new();
        for primitive in &self.segs {
            let subdivisions = primitive.subdivide(t);

            match subdivisions {
                Some(subs) => {         
                    new_segments.push(subs.0);
                    new_segments.push(subs.1);
                }
                _ => {
                    new_segments.push(primitive.clone());
                }
            }
        }

        return Piecewise::new(new_segments, None);
    }

    pub fn cut_at_t(&self, t: f64) -> Piecewise<T>
    {
        let mut new_segments = Vec::new();
        let seg_num = self.seg_n(t);
        let primitive = &self.segs[seg_num];
        let seg_time = self.seg_t(t);

        let iter = self.segs.iter().enumerate();
        for (i, seg) in iter {
            if i == seg_num {
                let subdivisions = seg.subdivide(seg_time);

                match subdivisions {
                    Some(subs) => {         
                        new_segments.push(subs.0);
                        new_segments.push(subs.1);
                    }
                    _ => {
                        new_segments.push(primitive.clone());
                    }
                }        
            } else {
                new_segments.push(primitive.clone());
            }
        }

        return Piecewise::new(new_segments, None);
    }
}

// Returns a primitive and the range of t values that it covers.
pub struct SegmentIterator<T: Evaluate+Primitive+Sized> {
    piecewise: Piecewise<T>,
    counter: usize
}

impl<T: Evaluate+Primitive+Sized> SegmentIterator<T> {
    pub fn new(pw: Piecewise<T>) -> Self {
        Self {
            piecewise: pw,
            counter: 0
        }
    }
}

impl<T: Evaluate+Primitive+Sized> Iterator for SegmentIterator<T> {
    type Item = (T, f64, f64); // primitive, start time, end time

    fn next(&mut self) -> Option<Self::Item>
    {
        if self.counter == self.piecewise.segs.len() {
            return None;
        }

        let start_time = self.piecewise.cuts[self.counter];
        let end_time = self.piecewise.cuts[self.counter + 1];
        let item = &self.piecewise.segs[self.counter];

        self.counter = self.counter + 1;
        return Some((item.clone(), start_time, end_time));
    }
}