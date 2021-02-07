use super::vector::*;
use super::rect::*;
use super::bezier::*;
use super::piecewise::*;
use super::super::evaluate::*;

// Implements the evaluate trait for Piecewise
impl<T: Evaluate> Evaluate for Piecewise<T> {
    type EvalResult = T::EvalResult;

    // return the x, y of our curve at time t
    fn evaluate(&self, t: f64) -> Self::EvalResult
    {
        /*
        // there needs to be better handling than this probably through a fail/success
        if self.segs.len() == 0 {panic!("Can't evaluate an empty piecewise!")}

        // we multiply t by our segments then subtract the floored version of this value from the original to get
        // our offset t for that curve
        let modified_time = (self.segs.len()) as f64 * t;
        let curve_index = modified_time.floor().min((self.segs.len() - 1) as f64) as usize;
        let offset_time = modified_time - curve_index as f64;
        */

        
        let curve_index = self.segN(t);
        let offset_time = self.segT(t);

        let ref dir = self.segs[curve_index];

        return dir.evaluate(offset_time);  
    }

    // returns the derivative at time t
    fn derivative(&self, t: f64) -> Self::EvalResult
    {
        /*
        // there needs to be better handling than this probably through a fail/success
        if self.segs.len() == 0 {panic!("Can't find derivative for an empty piecewise!")}

        // we multiply t by our segments then subtract the floored version of this value from the original to get
        // our offset t for that curve
        let modified_time = (self.segs.len()) as f64 * t;
        let curve_index = modified_time.floor().min((self.segs.len() - 1) as f64) as usize;
        let offset_time = modified_time - curve_index as f64;
        */

        let curve_index = self.segN(t);
        let offset_time = self.segT(t);

        let ref dir = self.segs[curve_index];

        return dir.derivative(offset_time);  
    }

    fn bounds(&self) -> Rect {
        // again maybe success/failure? These are mainly here to catch bugs right now.
        if self.segs.len() == 0 {panic!("An empty piecewise knows no bounds!")}

        let mut output = Rect {
            left: f64::INFINITY,
            bottom: f64::INFINITY,
            right: -f64::INFINITY,
            top: -f64::INFINITY,
        };

        for curve in &self.segs {
            output = output.encapsulate_rect(curve.bounds());
        }

        return output;
    }

    fn apply_transform<F>(&self, transform: F) -> Self where F: Fn(&Self::EvalResult) -> Self::EvalResult
    {
        let mut output = Vec::new();
        for contour in &self.segs {
            output.push(contour.apply_transform(&transform));
        }

        return Piecewise::new(output, Some(self.cuts.clone()))
    }

    
    fn start_point(&self) -> Self::EvalResult
    {
        if let Some(path_fcurve) = self.segs.first() {
            return path_fcurve.start_point();
        }

        // TODO: Add proper error handling to these functions.
        panic!("Empty piecewise has no start point.")
    }

    fn end_point(&self) -> Self::EvalResult
    {
        if let Some(path_lcurve) = self.segs.first() {
            return path_lcurve.start_point();
        }

        // TODO: Add proper error handling to these functions.
        panic!("Empty piecewise has no start point.")
    }

}
