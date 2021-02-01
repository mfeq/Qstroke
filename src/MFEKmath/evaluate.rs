use super::vector::Vector;
use super::rect::Rect;

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
    fn start_point(&self) -> Vector;
    fn end_point(&self) -> Vector;
}

pub trait EvaluateTransform: Evaluate {
    fn translate(&self, x: f64, y: f64) -> Self;
    fn scale(&self, x: f64, y: f64) -> Self;
}

impl<T> EvaluateTransform for T where T: Evaluate {
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

// This trait is implemented for a primitive shape like a line, bezier, spiro, etc within the piecewise.
pub trait Primitive
{
    fn subdivide(&self, t: f64) -> (Self, Self) where Self: Sized;
}