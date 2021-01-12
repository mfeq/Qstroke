use super::Evaluate;
use super::vector::Vector;
use super::rect::Rect;
use glifparser::{WhichHandle};

mod evaluate;

#[derive(Clone)]
#[allow(non_snake_case)]
pub struct Bezier {
    A:f64, B:f64, C:f64, D:f64,
    E:f64, F:f64, G:f64, H:f64,
}

impl Bezier {
    // this function should accept lines, quadratic, and cubic segments and return a valid set of cubic bezier coefficients
    pub fn from<T>(point: &glifparser::Point<T>, next_point: &glifparser::Point<T>) -> Self
    {
        let p = Vector::from_point(point);
        let np = Vector::from_point(next_point);
        let h1 = Vector::from_handle(point, WhichHandle::A);
        let h2 = Vector::from_handle(next_point, WhichHandle::B);

        return Self::from_control_points(p, h1, h2, np);
    }

    pub fn from_control_points(p0: Vector, p1: Vector, p2: Vector, p3: Vector) -> Self
    {
        let x0 = p0.x; let y0 = p0.y;
        let x1 = p1.x; let y1 = p1.y;
        let x2 = p2.x; let y2 = p2.y;
        let x3 = p3.x; let y3 = p3.y;

        return Self {
            A: (x3 - 3. * x2 + 3. * x1 - x0),
            B: (3. * x2 - 6. * x1 + 3. * x0),
            C: (3. * x1 - 3. * x0),
            D: x0,
            
            E: (y3 - 3. * y2 + 3. * y1 - y0),
            F: (3. * y2 - 6. * y1 + 3. * y0),
            G: (3. * y1 - 3. * y0),
            H: y0,
        };
    }

    pub fn to_control_points(&self) -> [Vector; 4]
    {
        let output: [Vector; 4] = [
            Vector {x: self.D, y: self.H},
            Vector {x: (self.D + self.C / 3.), y: (self.H + self.G / 3.)},
            Vector {x: (self.D + 2. * self.C / 3. + self.B / 3.), y: (self.H + 2. * self.G / 3. + self.F / 3.)}, 
            Vector {x: (self.D + self.C + self.B + self.A), y: (self.H + self.G + self.F + self.E)},
        ];

        return output;
    }

    pub fn to_control_points_vec(&self) -> Vec<Vector>
    {
        let controlps = self.to_control_points();

        let mut output = Vec::new();
        for p in &controlps {
            output.push(p.clone());
        }

        return output;
    }

    // returns two curves one before t and one after
    // https://www.malinc.se/m/DeCasteljauAndBezier.php
    pub fn subdivide(&self,  t:f64) -> (Bezier, Bezier)
    {
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
        let first = Self::from_control_points(points[0], q0, r0, s0);
        let second = Self::from_control_points(s0, r1, q2, points[3]);

        return (first, second);
    }
}
