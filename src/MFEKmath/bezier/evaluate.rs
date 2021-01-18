use super::super::{Vector, Evaluate, Rect, Bezier};

impl Evaluate for Bezier {

    fn evaluate(&self, t: f64) -> Vector
    {
        Vector {
            x: self.A * t * t * t + self.B * t * t + self.C * t + self.D,
            y: self.E * t * t * t + self.F * t * t + self.G * t + self.H
        }
    }

    
    fn derivative(&self, _u: f64) -> Vector
    {
        // TODO: Fix the code calling this and remove this terrible cludge. Luckily this is currently
        // only called in the PaP code.
        let u = f64::clamp(_u, 0., 1.);
        let e = 1e-10;
        let offset = 1e-3;
        let mut t = u;

        // this is a bit of a hack because we might be sampling off-curve
        // so we just shift t over a really tiny amount to keep our samples bounded
        // between 0-1
        if u + e > 1. { t = t - offset};
        if u - e < 0. { t = t + offset};

        // calculate the tangent vector for the point
        Vector {
            x: 3. * self.A * t * t + 2. * self.B * t + self.C,
            y: 3. * self.E * t * t + 2. * self.F * t + self.G
        }
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
}