use super::vector::Vector;
use glifparser::{WhichHandle};

mod evaluate;
mod primitive;
mod flo;

#[derive(Clone, Debug)]
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

        return Self::from_points(p, h1, h2, np);
    }

    pub fn from_points(p0: Vector, p1: Vector, p2: Vector, p3: Vector) -> Self
    {
        let x0 = p0.x; let y0 = p0.y;
        let x1 = p1.x; let y1 = p1.y;
        let x2 = p2.x; let y2 = p2.y;
        let x3 = p3.x; let y3 = p3.y;

        Self {
            A: (x3 - 3. * x2 + 3. * x1 - x0),
            B: (3. * x2 - 6. * x1 + 3. * x0),
            C: (3. * x1 - 3. * x0),
            D: x0,
            
            E: (y3 - 3. * y2 + 3. * y1 - y0),
            F: (3. * y2 - 6. * y1 + 3. * y0),
            G: (3. * y1 - 3. * y0),
            H: y0,
        }
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

    pub fn reverse(&self) -> Self 
    {
        let bz = self.to_control_points();
        Bezier::from_points(bz[3], bz[2], bz[1], bz[0])
    }

}
