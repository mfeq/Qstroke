use super::Interpolator;
use super::super::Vector;
use super::super::Evaluate;
use super::super::Rect;
use crate::vec2;

impl Evaluate for Interpolator
{
    type EvalResult = f64;
    
    fn at(&self, t: f64) -> f64
    {
        let interpolate_func = &self.func;
        return interpolate_func(self, t);
    }

    // Everything below this point should probably be moved into it's own trait sometime soon because
    // these functions don't exactly make sense here.
    fn tangent_at(&self, t: f64) -> f64
    {
        return 0.;
    }

    fn apply_transform<F>(&self, transform: F) -> Self where F: Fn(&f64) -> f64
    {
        return Self {
            start: transform(&self.start),
            finish: transform(&self.finish),
            func: self.func
        }
    }

    fn bounds(&self) -> Rect
    {
        return Rect::AABB_from_points(vec![vec2!(self.start, self.finish)]);
    }

    fn start_point(&self) -> Self::EvalResult
    {
        return self.start;
    }

    fn end_point(&self) -> Self::EvalResult
    {
        return self.finish;
    }
}