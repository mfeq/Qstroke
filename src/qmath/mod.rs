use glifparser::{ WhichHandle, Contour};
use glifparser::{Outline, Handle, PointType};
use skia_safe::{self as skia, path, Path};

// stub PointData out here, really not sure how I should be handnling this because we need a concrete
// type to construct our own glif
pub struct PointData;

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

    pub fn to_point<T>(self, handleA: Handle, handleB: Handle) -> glifparser::Point<T>
    {
        return glifparser::Point {
            x: self.x as f32,
            y: self.y as f32,
            a: handleA,
            b: handleB,
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


// Any object in a piecewise MUST implement this trait! This trait essentially says that our struct
// can be evaluated with respect to time t and return an x, y pair. It also needs to be able to give us
// a derivative and a bounding box.
// Could probably use a better name. Maybe Primitive as they're the building blocks of our glyph.
pub trait Evaluate
{
    fn evaluate(&self, t: f64) -> Vector; 
    fn derivative(&self, u: f64) -> Vector;
    fn bounds(&self) -> Rect;
    fn apply_transform<F>(&self, transform: F) -> Self where F: Fn(&Vector) -> Vector;
}

pub trait EvaluateTransforms: Evaluate {
    fn translate(&self, x: f64, y: f64) -> Self;
    fn scale(&self, x: f64, y: f64) -> Self;
}

impl<T> EvaluateTransforms for T where T: Evaluate {
    fn translate(&self, x: f64, y: f64) -> Self
    {
        let transform = |v: &Vector| {
            return Vector{x: v.x + x, y: v.y + y};
        };
    
        return self.apply_transform(&transform);
    }
    
    fn scale(&self, x: f64, y: f64) -> Self
    {
    
        let transform = |v: &Vector| {
            return Vector{x: v.x * x, y: v.y * y};
        };
    
        return self.apply_transform(&transform);
    }
}

pub trait Parameterization
{
    fn parameterize(&self, u: f64) -> f64;
}

// We decompose the path from glifpoints into bezier curves which we store in this. It stores the curve
// as coefficients and implements the Evaluate trait for piecewise.
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

impl Evaluate for Bezier {
    // return the x, y of our curve at time t
    fn evaluate(&self, t: f64) -> Vector
    {
        Vector {
            x: self.A * t * t * t + self.B * t * t + self.C * t + self.D,
            y: self.E * t * t * t + self.F * t * t + self.G * t + self.H
        }
    }

    
    fn derivative(&self, _u: f64) -> Vector
    {
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
        /*
        let v0 = self.evaluate(t + e);
        let v1 = self.evaluate(t - e);

        return Vector {
            x: (v1.x - v0.x) / (2. * e),
            y: (v1.y - v0.y) / (2. * e)
        }
        */
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

        return Bezier::from_control_points(tp[0], tp[1], tp[2], tp[3]);
    }

    fn bounds(&self) -> Rect
    {
        return Rect::AABB_from_points(self.to_control_points_vec());
    }
}

// This struct models a simple piecewise function. It maps 0-1 such that 0 is the beginning of the first curve
// in the collection and 1 is the end of the last. It does not currently support arbitrary cuts.
pub struct Piecewise<T: Evaluate> {
    // this should definitely change to private at some point with a getter
    pub curves: Vec<T>,
}

impl<T: Evaluate> Evaluate for Piecewise<T> {
    // return the x, y of our curve at time t
    fn evaluate(&self, t: f64) -> Vector
    {
        // there needs to be better handling than this probably through a fail/success
        if self.curves.len() == 0 {panic!("Can't evaluate an empty piecewise!")}

        // we multiply t by our segments then subtract the floored version of this value from the original to get
        // our offset t for that curve
        let modified_time = (self.curves.len()) as f64 * t;
        let curve_index = modified_time.floor().min((self.curves.len() - 1) as f64) as usize;
        let offset_time = modified_time - curve_index as f64;

        let ref dir = self.curves[curve_index];

        return dir.evaluate(offset_time);  
    }

    // returns the derivative at time t
    fn derivative(&self, t: f64) -> Vector
    {
        // there needs to be better handling than this probably through a fail/success
        if self.curves.len() == 0 {panic!("Can't find derivative for an empty piecewise!")}

        // we multiply t by our segments then subtract the floored version of this value from the original to get
        // our offset t for that curve
        let modified_time = (self.curves.len()) as f64 * t;
        let curve_index = modified_time.floor().min((self.curves.len() - 1) as f64) as usize;
        let offset_time = modified_time - curve_index as f64;

        let ref dir = self.curves[curve_index];

        return dir.derivative(offset_time);  
    }

    fn bounds(&self) -> Rect {
        // again maybe success/failure? These are mainly here to catch bugs right now.
        if self.curves.len() == 0 {panic!("An empty piecewise knows no bounds!")}

        let mut output = Rect {
            left: f64::INFINITY,
            bottom: f64::INFINITY,
            right: -f64::INFINITY,
            top: -f64::INFINITY,
        };

        for curve in &self.curves {
            output = output.encapsulate_rect(curve.bounds());
        }

        return output;
    }

    fn apply_transform<F>(&self, transform: F) -> Self where F: Fn(&Vector) -> Vector
    {
        let mut output = Vec::new();
        for contour in &self.curves {
            output.push(contour.apply_transform(&transform));
        }

        return Piecewise{
            curves: output,
        };
    }
}

// I want to generalize as much of the functionality in these two typed implementations as possible. Some of the stuff
// like the to and from functions are likely to stay, but I'd really like to genericize subdivide and split.
impl Piecewise<Piecewise<Bezier>>
{
    pub fn to_skpath(&self) -> Path
    {
        let path = Path::new();
        return self.append_to_skpath(path);
    }

    pub fn from_skpath(ipath: &Path) -> Self {
        let mut contours: Vec<Piecewise<Bezier>> = Vec::new();
        let iter = path::Iter::new(ipath, false);
    
        let mut cur_contour: Vec<Bezier> = Vec::new();
        let mut last_point: Vector = Vector{x: 0., y: 0.}; // don't think we need this?
        for (v, vp) in iter {
            match v {
                path::Verb::Move => {
                    if !cur_contour.is_empty() {
                        contours.push(Piecewise { curves: cur_contour })
                    }
    
                    cur_contour = Vec::new();  
                    last_point = Vector::from_skia_point(vp.first().unwrap());
                }
    
                path::Verb::Line => {
                    let lp = Vector::from_skia_point(&vp[0]);
                    let np = Vector::from_skia_point(&vp[1]);
                    cur_contour.push(Bezier::from_control_points(lp, lp, np, np));
                    last_point = np;
                }
    
                path::Verb::Quad => {
                    let lp = last_point;
                    let h2 = Vector::from_skia_point(&vp[0]);
                    let np = Vector::from_skia_point(&vp[1]);
                    cur_contour.push(Bezier::from_control_points(lp, lp, h2, np));
                    last_point = np;
                }
    
                path::Verb::Cubic => {
                    let lp = Vector::from_skia_point(&vp[0]);
                    let h1 = Vector::from_skia_point(&vp[1]);
                    let h2 = Vector::from_skia_point(&vp[2]);
                    let np = Vector::from_skia_point(&vp[3]);
                    cur_contour.push(Bezier::from_control_points(lp, h1, h2, np));
                    last_point = np;
                }
    
                path::Verb::Close => {
                    contours.push(Piecewise { curves: cur_contour.clone()});
                    cur_contour = Vec::new();
                }
                
    
                // I might have to implement more verbs, but at the moment we're just converting
                // from glifparser output and these are all the supported primitives there.
                _ => { println!("{:?} {:?}", v, vp); panic!("Unsupported skia verb in skpath!"); }
            }
        }
    
        if !cur_contour.is_empty() {
            contours.push(Piecewise{ curves: cur_contour });
        }
    
        return Piecewise {
            curves: contours
        }
    }    

    pub fn append_to_skpath(&self, mut skpath: Path) -> Path {
        for contour in &self.curves {
            skpath = contour.append_to_skpath(skpath);
        }

        return skpath;
    }

    pub fn from_outline<U>(outline: &Outline<U>) -> Self
    {   
        let mut ret = Piecewise {
            curves: Vec::new(),
        };
    
        for contour in outline
        {
            ret.curves.push(Piecewise::from_contour(contour));
        }
    
        return ret;
    }

    pub fn to_outline(&self) -> Outline<Option<PointData>>
    {
        let mut output_outline: Outline<Option<PointData>> = Outline::new();

        for contour in &self.curves
        {
            output_outline.push(contour.to_contour());
        }

        return output_outline;
    }

    pub fn subdivide(&self, t: f64) -> Self
    {
        let mut output = Vec::new();
        for contour in &self.curves {
            output.push(contour.subdivide(t));
        }

        return Piecewise{
            curves: output,
        };
    }
}

impl Piecewise<Bezier>
{
    pub fn from_contour<U>(contour: &Contour<U>) -> Self
    {   
        let mut ret = Piecewise {
            curves: Vec::new(),
        };

        let mut lastpoint: Option<&glifparser::Point<U>> = None;
        let firstpoint = contour.first().unwrap();

        for point in contour
        {
            match lastpoint
            {
                None => {},
                Some(lastpoint) => {
                    ret.curves.push(Bezier::from(&lastpoint, point));
                }
            }

            lastpoint = Some(point);
        }

        if firstpoint.ptype != PointType::Move {
            ret.curves.push(Bezier::from(&lastpoint.unwrap(), firstpoint));
        }

        return ret
    }

    pub fn to_contour(&self) -> Contour<Option<PointData>>
    {
        let mut output_contour: Contour<Option<PointData>> = Vec::new();
        let mut last_curve: Option<[Vector; 4]> = None;

        for curve in &self.curves
        {                       
            let control_points = curve.to_control_points();

            let mut new_point = control_points[0].to_point(control_points[1].to_handle(), Handle::Colocated);

            // if this isn't the first point we need to backtrack and set our output point's b handle
            match last_curve
            {
                Some(lc) => {
                    // set the last output point's a handle to match the new curve
                    new_point.b = lc[2].to_handle();
                }
                None => {}
            }

            output_contour.push(new_point);

            last_curve = Some(control_points);
        }

        // we've got to connect the last point and the first point
        output_contour.first_mut().unwrap().b = Vector::to_handle(last_curve.unwrap()[2]);
    
        return output_contour;
    }

    
    pub fn append_to_skpath(&self, mut skpath: Path) -> Path
    {
        let mut first = true;
        for bez in &self.curves {
            let controlp = bez.to_control_points();

            if first {
                skpath.move_to(controlp[0].to_skia_point());
                first = false;
            }
            
            // we've got ourselves a line
            if controlp[0] == controlp[2] && controlp[1] == controlp[3] {
                skpath.line_to(controlp[3].to_skia_point());
            }

            skpath.cubic_to(controlp[1].to_skia_point(), controlp[2].to_skia_point(), controlp[3].to_skia_point());
        }

        return skpath;
    }


    pub fn subdivide(&self, t: f64) -> Piecewise<Bezier>
    {
        let mut new_curves = Vec::new();
        for bez in &self.curves {
            let subdivisions = bez.subdivide(t);

            new_curves.push(subdivisions.0);
            new_curves.push(subdivisions.1);
        }

        return Piecewise {
            curves: new_curves
        }
    }
}

// We build a table of total arc length along the line and use it to map 0-1
// to the arclength of the curve such that 0.5 is halfway along the curve by arc-length
pub struct ArcLengthParameterization
{
    pub arclens: Vec<f64>
}

impl ArcLengthParameterization
{
    pub fn from(evaluable: &impl Evaluate) -> Self
    {
        let mut output = Vec::new();
        // TODO: this is an arbitrary number and should be replaced with something more robust
        // TODO: preferably a tolerance value
        let arclen_cuts = 10000;
        let max_cuts = 10000 + 1;

        let mut prev_point = evaluable.evaluate(0.0);
        let mut sum = 0.0;
        output.push(sum);
        
        let mut i = 1;
        while i < max_cuts
        {
            let t = i as f64 / arclen_cuts as f64;
            let point = evaluable.evaluate(t);
            let dist = Vector::distance(point, prev_point);
            sum = sum + dist;
            output.push(sum);

            prev_point = point;
            i = i + 1;
        }

        return Self {
            arclens: output
        }
    }

    pub fn get_total_arclen(&self) -> f64
    {
        return *self.arclens.last().unwrap();
    }

    // Have to implement a custom binary search here because we're looking
    // not for an exact index but the index of the highest value that's less than
    // the target
    fn search_for_index(&self, target: f64) -> usize
    {
        let mut left = 0;
        let mut right = self.arclens.len() - 1;

        while left < right {
            let middle = (right+left)/2;

            if left == middle { return middle; }
            if right == middle { return left; }
            if self.arclens[middle] == target { return middle };

            if self.arclens[middle] < target {
                left = middle
            }
            else
            {
                right = middle;
            }
        }

        // This needs to be replaced with success/failure.
        panic!("Couldn't find the target arc length!")
    }
}

impl Parameterization for ArcLengthParameterization
{
    fn parameterize(&self, u: f64) -> f64
    {
        let target_arclen = u * self.arclens[self.arclens.len() - 1];
 
        let arclen_index = self.search_for_index(target_arclen);
        if target_arclen == self.arclens[arclen_index]
        {
           return arclen_index as f64 / (self.arclens.len() - 1) as f64;
        }
        else
        {
            let len_start = self.arclens[arclen_index];
            let len_end = self.arclens[arclen_index+1];
            let segment_len = len_end - len_start;

            let segment_fraction = (target_arclen - len_start) / segment_len;

            return (arclen_index as f64 + segment_fraction) / (self.arclens.len() - 1) as f64;
        }
    }
}