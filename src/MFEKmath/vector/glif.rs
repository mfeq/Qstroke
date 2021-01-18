use super::Vector;
use glifparser::{Handle, WhichHandle, PointType};

impl Vector {
    pub fn from_point<T>(point: &glifparser::Point<T>) -> Self
    {
        Vector { x: point.x as f64, y: point.y as f64 }
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

}