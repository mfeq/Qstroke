mod vector;
mod piecewise;
mod rect;
mod bezier;
mod arclenparameterization;

pub use self::vector::*;
pub use self::piecewise::*;
pub use self::rect::*;
pub use self::bezier::*;
pub use self::arclenparameterization::*;

// stub PointData out here, really not sure how I should be handnling this because we need a concrete
// type to construct our own glif
pub struct PointData;

// Any object in a piecewise MUST implement this trait! This trait essentially says that our struct
// can be evaluated with respect to time t and return an x, y pair. It also needs to be able to give us
// a derivative and a bounding box.
// Could probably use a better name. Maybe Primitive as they're the building blocks of our glyph.
pub trait Evaluate
{
    fn evaluate(&self, t: f64) -> Vector; 
    fn derivative(&self, u: f64) -> Vector;
    fn bounds(&self) -> Rect; // returns an AABB that contains all points 
    fn apply_transform<F>(&self, transform: F) -> Self where F: Fn(&Vector) -> Vector;
}

pub trait EvaluateTransforms: Evaluate {
    fn translate(&self, x: f64, y: f64) -> Self;
    fn scale(&self, x: f64, y: f64) -> Self;
}

impl<T> EvaluateTransforms for T where T: Evaluate {
    fn translate(&self, x: f64, y: f64) -> Self
    {
        let transform = |v: &Vector| {
            return Vector{x: v.x + x, y: v.y + y};
        };
    
        return self.apply_transform(&transform);
    }
    
    fn scale(&self, x: f64, y: f64) -> Self
    {
    
        let transform = |v: &Vector| {
            return Vector{x: v.x * x, y: v.y * y};
        };
    
        return self.apply_transform(&transform);
    }
}

pub trait Parameterization
{
    fn parameterize(&self, u: f64) -> f64;
}

/*
use glifparser::{ WhichHandle, Contour};
use glifparser::{Outline, Handle, PointType};
*/