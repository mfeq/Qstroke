use glifparser::*;

pub mod glif;
mod skpath;
mod evaluate;

use super::*;
use super::vector::*;
use super::bezier::Bezier;
use super::evaluate::Evaluate;

// This struct models a simple piecewise function. It maps 0-1 such that 0 is the beginning of the first curve
// in the collection and 1 is the end of the last. It does not currently support arbitrary cuts.
pub struct Piecewise<T: Evaluate> {
    // this should definitely change to private at some point with an iterator or getter to access
    pub curves: Vec<T>,
}

// TODO: Move these functions to a more appropriate submodule.
impl Piecewise<Piecewise<Bezier>>
{

    pub fn subdivide(&self, t: f64) -> Self
    {
        let mut output = Vec::new();
        for contour in &self.curves {
            output.push(contour.subdivide(t));
        }

        return Piecewise{
            curves: output,
        };
    }
}

impl Piecewise<Bezier>
{    
    pub fn subdivide(&self, t: f64) -> Piecewise<Bezier>
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