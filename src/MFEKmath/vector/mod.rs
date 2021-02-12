mod skia;
mod glif;
mod flo;

use super::coordinate::Coordinate;
use super::coordinate::Coordinate2D;

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

#[macro_export]
macro_rules! vec2 {
    ($x: expr, $y: expr) => {
        Vector {x: $x, y: $y}
    };
}

impl Vector {
    pub fn from_components(x: f64, y: f64) -> Self
    {
        Vector{ x: x, y: y }
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

    pub fn mul(self, v1: Vector) -> Self
    {
        vec2!(self.x * v1.x, self.y * v1.y)
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

    pub fn dot(self, v1: Vector) -> f64
    {
        self.x * v1.x + self.y * v1.y
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
    
    fn sub(self, v1: Vector) -> Vector { return self.sub(v1);}
}

impl std::ops::Mul<Vector> for Vector {
    type Output = Vector;
    
    fn mul(self, s: Vector) -> Vector { return self.mul(s);}
}

impl std::ops::Mul<f64> for Vector {
    type Output = Vector;
    
    fn mul(self, s: f64) -> Vector { return self.multiply_scalar(s);}
}

impl std::ops::Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector { Vector{x: -self.x, y: -self.y} }
}

impl Coordinate for Vector {
    fn magnitude(self) -> f64 
    {
        self.magnitude()
    }

    fn distance(self, v1: Self) -> f64
    {
        self.distance(v1)
    }

    fn lerp(self, v1: Self, t: f64) -> Self
    {
        self.lerp(v1, t)
    }
}