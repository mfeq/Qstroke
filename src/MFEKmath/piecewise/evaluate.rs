use super::vector::*;
use super::rect::*;
use super::bezier::*;
use super::piecewise::*;
use super::super::evaluate::*;

// Implements the evaluate trait for Piecewise
impl<T: Evaluate> Evaluate for Piecewise<T> {
    // return the x, y of our curve at time t
    fn evaluate(&self, t: f64) -> Vector
    {
        // there needs to be better handling than this probably through a fail/success
        if self.curves.len() == 0 {panic!("Can't evaluate an empty piecewise!")}

        // we multiply t by our segments then subtract the floored version of this value from the original to get
        // our offset t for that curve
        let modified_time = (self.curves.len()) as f64 * t;
        let curve_index = modified_time.floor().min((self.curves.len() - 1) as f64) as usize;
        let offset_time = modified_time - curve_index as f64;

        let ref dir = self.curves[curve_index];

        return dir.evaluate(offset_time);  
    }

    // returns the derivative at time t
    fn derivative(&self, t: f64) -> Vector
    {
        // there needs to be better handling than this probably through a fail/success
        if self.curves.len() == 0 {panic!("Can't find derivative for an empty piecewise!")}

        // we multiply t by our segments then subtract the floored version of this value from the original to get
        // our offset t for that curve
        let modified_time = (self.curves.len()) as f64 * t;
        let curve_index = modified_time.floor().min((self.curves.len() - 1) as f64) as usize;
        let offset_time = modified_time - curve_index as f64;

        let ref dir = self.curves[curve_index];

        return dir.derivative(offset_time);  
    }

    fn bounds(&self) -> Rect {
        // again maybe success/failure? These are mainly here to catch bugs right now.
        if self.curves.len() == 0 {panic!("An empty piecewise knows no bounds!")}

        let mut output = Rect {
            left: f64::INFINITY,
            bottom: f64::INFINITY,
            right: -f64::INFINITY,
            top: -f64::INFINITY,
        };

        for curve in &self.curves {
            output = output.encapsulate_rect(curve.bounds());
        }

        return output;
    }

    fn apply_transform<F>(&self, transform: F) -> Self where F: Fn(&Vector) -> Vector
    {
        let mut output = Vec::new();
        for contour in &self.curves {
            output.push(contour.apply_transform(&transform));
        }

        return Piecewise{
            curves: output,
        };
    }
}
