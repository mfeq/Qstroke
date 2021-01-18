use super::vector::Vector;

// An axis-aligned rectangle. Sort of a stub right now to make some function outputs more legible.
pub struct Rect {
    pub top: f64,
    pub bottom: f64,
    pub left: f64,
    pub right: f64
}

impl Rect {
    // We expand the rect to be the minimum axis aligned bounding box that holds both our current rect and the point.
    pub fn encapsulate(&self, p: Vector) -> Rect
    {
        let mut lx = self.left;
        let mut ly = self.bottom;
        let mut hx = self.right;
        let mut hy = self.top;
    
        if p.x > hx { hx = p.x }
        if p.y > hy { hy = p.y }
        if p.x < lx { lx = p.x }
        if p.y < ly { ly = p.y }
    
        return Rect {
            left: lx,
            right: hx,
            top: hy,
            bottom: ly
        };
    }

    // Taje a vec of points and return a minimum bounding box for that point.
    #[allow(non_snake_case)]
    pub fn AABB_from_points(points: Vec<Vector>) -> Self
    {
        let mut lx = f64::INFINITY;
        let mut ly = f64::INFINITY;
        let mut hx = -f64::INFINITY;
        let mut hy = -f64::INFINITY;
    
        for p in points {
            if p.x > hx { hx = p.x }
            if p.y > hy { hy = p.y }
            if p.x < lx { lx = p.x }
            if p.y < ly { ly = p.y }
        }
    
        return Rect {
            left: lx,
            right: hx,
            top: hy,
            bottom: ly
        };
    }

    pub fn encapsulate_rect(&self, other: Rect) -> Rect
    {
        let left_bottom = Vector{x: other.left, y: other.bottom};
        let right_top = Vector{x: other.right, y: other.top};
        return self.encapsulate(left_bottom).encapsulate(right_top)
    }
}
