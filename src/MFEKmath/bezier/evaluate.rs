use super::super::{Vector, Evaluate, Rect, Bezier};
use super::super::consts::SMALL_DISTANCE;
use flo_curves::BezierCurve;

use flo_curves::bezier::{derivative4, de_casteljau3, de_casteljau4, characterize_curve};

impl Evaluate for Bezier {
    type EvalResult = Vector;
    
    fn at(&self, t: f64) -> Vector
    {
        let d1 = BezierCurve::start_point(self);
        let (d2, d3) = BezierCurve::control_points(self);
        let d4 = BezierCurve::end_point(self);

        de_casteljau4(t, d1, d2, d3, d4)
    }

    
    fn tangent_at(&self, t: f64) -> Vector
    {
            // Extract the points that make up this curve
            let w1          = BezierCurve::start_point(self);
            let (w2, w3)    = BezierCurve::control_points(self);
            let w4          = BezierCurve::end_point(self);
    
            // If w1 == w2 or w3 == w4 there will be an anomaly at t=0.0 and t=1.0 
            // (it's probably mathematically correct to say there's no tangent at these points but the result is surprising and probably useless in a practical sense)
            let t = if t == 0.0 { f64::EPSILON }        else { t };
            let t = if t == 1.0 { 1.0-f64::EPSILON }    else { t };
    
            // Get the deriviative
            let (d1, d2, d3) = derivative4(w1, w2, w3, w4);
    
            // Get the tangent and the point at the specified t value
            let tangent     = de_casteljau3(t, d1, d2, d3);
    
            tangent
    }

    fn apply_transform<F>(&self, transform: F) -> Self where F: Fn(&Vector) -> Vector
    {
        let original_points = self.to_control_points();
        let tp: [Vector; 4] = [
            transform(&original_points[0]),
            transform(&original_points[1]),
            transform(&original_points[2]),
            transform(&original_points[3]),
        ];

        return Bezier::from_points(tp[0], tp[1], tp[2], tp[3]);
    }

    fn bounds(&self) -> Rect
    {
        return Rect::AABB_from_points(self.to_control_points_vec());
    }

    fn start_point(&self) -> Vector
    {
        return self.to_control_points()[0];

    }

    fn end_point(&self) -> Vector
    {
        return self.to_control_points()[3];
    }

}