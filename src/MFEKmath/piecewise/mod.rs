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
// in the collection and 1 is the end of the last. It does not currently support arbitrary cuts.
pub struct Piecewise<T: Evaluate> {
    // this should definitely change to private at some point with an iterator or getter to access
    pub curves: Vec<T>,
}

// TODO: Move these functions to a more appropriate submodule.
impl<T: Evaluate+Primitive> Piecewise<Piecewise<T>>
{
    // we split the primitive that contains t at t
    pub fn cut_at_t(&self, t: f64) -> Self
    {
        let mut output = Vec::new();
        for contour in &self.curves {
            output.push(contour.cut_at_t(t));
        }

        return Piecewise{
            curves: output,
        };
    }
}

impl<T: Evaluate+Primitive> Piecewise<T>
{    
    pub fn cut_at_t(&self, t: f64) -> Piecewise<T>
    {
        let mut new_curves = Vec::new();
        for bez in &self.curves {
            let subdivisions = bez.subdivide(t);

            new_curves.push(subdivisions.0);
            new_curves.push(subdivisions.1);
        }

        return Piecewise {
            curves: new_curves
        }
    }
}