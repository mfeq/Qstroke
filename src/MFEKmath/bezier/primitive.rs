use super::Bezier;
use super::Vector;
use super::super::evaluate::Primitive;
use super::super::consts::SMALL_DISTANCE;

impl Primitive for Bezier {
    // returns two curves one before t and one after
    // https://www.malinc.se/m/DeCasteljauAndBezier.php
    fn subdivide(&self,  t:f64) -> Option<(Self, Self)>
    {
        if t == 0. || t == 1. {
            // if we're really close to 0 or 1 it doesn't make sense to split
            return None;
        }

        // easier to understand this operation when working in points
        // it's just a bit of lerping
        let points = self.to_control_points();

        // We lerp between the control points and their handles 
        let q0 = Vector::lerp(points[0], points[1], t);
        let q1 = Vector::lerp(points[1], points[2], t);
        let q2 = Vector::lerp(points[2], points[3], t);

        // next we calculate the halfways between the qs
        let r0 = Vector::lerp(q0, q1, t);
        let r1 = Vector::lerp(q1, q2, t);

        // and finally the half way between those two is the point at which we split the curve
        let s0 = Vector::lerp(r0, r1, t);

        // we reconstruct our two bezier curves from these points check out the link above
        // for a visualization
        let first = Self::from_points(points[0], q0, r0, s0);
        let second = Self::from_points(s0, r1, q2, points[3]);

        return Some((first, second));
    }
}