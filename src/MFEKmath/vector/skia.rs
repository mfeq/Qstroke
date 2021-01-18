use super::Vector;

impl Vector {
    pub fn to_skia_point(self) -> (f32, f32)
    {
        return (self.x as f32, self.y as f32);
    }

    pub fn from_skia_point(p: &skia_safe::Point) -> Self
    {
        return Vector {x: p.x as f64, y: p.y as f64 }
    }
}