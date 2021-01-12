use glifparser::{Handle, WhichHandle, PointType};

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn from_point<T>(point: &glifparser::Point<T>) -> Self
    {
        return Vector { x: point.x as f64, y: point.y as f64 };
    }

    pub fn to_point<T>(self, handle_a: Handle, handle_b: Handle) -> glifparser::Point<T>
    {
        return glifparser::Point {
            x: self.x as f32,
            y: self.y as f32,
            a: handle_a,
            b: handle_b,
            data: None,
            name: None,
            ptype: PointType::Curve
        }
    }

    pub fn to_skia_point(self) -> (f32, f32)
    {
        return (self.x as f32, self.y as f32);
    }

    pub fn from_skia_point(p: &skia_safe::Point) -> Self
    {
        return Vector {x: p.x as f64, y: p.y as f64 }
    }

    pub fn from_handle<T>(point: &glifparser::Point<T>, which: WhichHandle) -> Vector
    {
        let handle = match which {
            WhichHandle::A => point.a,
            WhichHandle::B => point.b,
            WhichHandle::Neither => Handle::Colocated,
        };

        match handle {
            Handle::At(x, y) => Vector {x: x as f64, y: y as f64},
            Handle::Colocated => Self::from_point(point),
        }
    }

    pub fn to_handle(self) -> Handle
    {
        Handle::At(self.x as f32, self.y as f32)
    }

    pub fn is_near(self, v1: Vector, eps: f64) -> bool
    {
        self.x - v1.x <= eps && self.x - v1.x >= -eps &&
        self.y - v1.y <= eps && self.y - v1.y >= -eps
    }

    pub fn add(self, v1: Vector) -> Self
    {
        Vector {x: self.x + v1.x, y: self.y + v1.y}
    }

    pub fn sub(self, v1: Vector) -> Self
    {
        Vector {x: self.x - v1.x, y: self.y - v1.y}
    }

    pub fn multiply_scalar(self, s: f64) -> Self
    {
        Vector {x: self.x * s, y: self.y * s}
    }

    pub fn magnitude(self) -> f64
    {
        f64::sqrt(f64::powi(self.x, 2) + f64::powi(self.y, 2))
    }
    
    pub fn distance(self, v1: Vector) -> f64
    {
        let v0 = self;
        f64::sqrt(f64::powi(v1.x - v0.x, 2) + f64::powi(v1.y - v0.y, 2))
    }

    pub fn normalize(self) -> Self
    {
        let magnitude = self.magnitude();
        Vector { x: self.x / magnitude, y: self.y / magnitude }
    }

    pub fn lerp(self, v1:Vector, t: f64) -> Self
    {
        let v0 = self;
        Vector {
            x: (1. - t) * v0.x + t * v1.x,
            y: (1. - t) * v0.y + t * v1.y
        }
    }
}

impl std::cmp::PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y;
    }
}

impl std::ops::Add<Vector> for Vector {
    type Output = Vector;
    
    fn add(self, v1: Vector) -> Vector { return self.add(v1);}
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;
    
    fn sub(self, v1: Vector) -> Vector { return self.add(v1);}
}

impl std::ops::Mul<f64> for Vector {
    type Output = Vector;
    
    fn mul(self, s: f64) -> Vector { return self.multiply_scalar(s);}
}

impl std::ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector { Vector{x: -self.x, y: -self.y} }
}