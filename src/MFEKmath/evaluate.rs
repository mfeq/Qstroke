use super::vector::Vector;
use super::rect::Rect;
use super::coordinate::Coordinate;
// Any object in a piecewise MUST implement this trait! This trait essentially says that our struct
// can be evaluated with respect to time t and return an x, y pair. It also needs to be able to give us
// a derivative and a bounding box.
// Could probably use a better name. Maybe Primitive as they're the building blocks of our glyph.
pub trait Evaluate
{
    type EvalResult: Coordinate;
    fn evaluate(&self, t: f64) -> Self::EvalResult; 
    fn derivative(&self, u: f64) -> Self::EvalResult;
    fn bounds(&self) -> Rect; // returns an AABB that contains all points 
    fn apply_transform<F>(&self, transform: F) -> Self where F: Fn(&Self::EvalResult) -> Self::EvalResult;
    fn start_point(&self) -> Self::EvalResult;
    fn end_point(&self) -> Self::EvalResult;
}

pub trait EvaluateTransform: Evaluate {
    fn translate(&self, t: Self::EvalResult) -> Self;
    fn scale(&self, s: Self::EvalResult) -> Self;
}

impl<T> EvaluateTransform for T where T: Evaluate {
    fn translate(&self, t: T::EvalResult) -> Self
    {
        let transform = |v: &T::EvalResult| {
            return *v + t;
        };
    
        return self.apply_transform(&transform);
    }
    
    fn scale(&self, s: T::EvalResult) -> Self
    {
    
        let transform = |v: &T::EvalResult| {
            return *v * s;
        };

        return self.apply_transform(&transform);
    }
}

// This trait is implemented for a primitive shape like a line, bezier, spiro, etc within the piecewise.
pub trait Primitive: Sized + Clone
{
    fn subdivide(&self, t: f64) -> Option<(Self, Self)> where Self: Sized;
}