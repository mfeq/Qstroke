use super::Vector;
use flo_curves::{Coordinate, Coordinate2D};

impl Coordinate2D for Vector {
    ///
    /// X component of this coordinate
    /// 
    #[inline]
    fn x(&self) -> f64 {
        self.x
    }

    ///
    /// Y component of this coordinate
    /// 
    #[inline]
    fn y(&self) -> f64 {
        self.y
    }
}

impl Coordinate for Vector {
    #[inline]
    fn from_components(components: &[f64]) -> Vector {
        Vector::from_components(components[0], components[1])
    }

    #[inline]
    fn origin() -> Vector {
        Vector{x: 0.0, y: 0.0}
    }

    #[inline]
    fn len() -> usize { 2 }

    #[inline]
    fn get(&self, index: usize) -> f64 { 
        match index {
            0 => self.x,
            1 => self.y,
            _ => panic!("Coord2 only has two components")
        }
    }

    fn from_biggest_components(p1: Vector, p2: Vector) -> Vector {
        Vector::from_components(f64::from_biggest_components(p1.x, p2.x), f64::from_biggest_components(p1.y, p2.y))
    }

    fn from_smallest_components(p1: Vector, p2: Vector) -> Vector {
        Vector::from_components(f64::from_smallest_components(p1.x, p2.x), f64::from_smallest_components(p1.y, p2.y))
    }

    #[inline]
    fn distance_to(&self, target: &Vector) -> f64 {
        let dist_x = target.x-self.x;
        let dist_y = target.y-self.y;

        f64::sqrt(dist_x*dist_x + dist_y*dist_y)
    }

    #[inline]
    fn dot(&self, target: &Self) -> f64 {
        self.x*target.x + self.y*target.y
    }
}