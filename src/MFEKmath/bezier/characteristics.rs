/*
use super::super::line::*;
use super::super::vector::Vector;
use super::super::bezier::Bezier;
use super::super::consts::SMALL_DISTANCE;
use super::super::evaluate::Evaluate;


// This code is flo_curves modified to use our structures.
const SMALL_DIVISOR: f64 = 0.0000001;

impl Bezier {
    #[inline]
    pub fn characteristics(&self) -> CurveCategory {
        let p = self.to_control_points();

        characterize_cubic_bezier(p[0], p[1], p[2], p[3])
    }

    #[inline]
    pub fn features(&self, accuracy: f64) -> CurveFeatures {
        let p = self.to_control_points();

        features_for_cubic_bezier(p[0], p[1], p[2], p[3], accuracy)
    }
}

///
/// Possible types of a two-dimensional cubic bezier curve
///
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CurveCategory {
    /// The control points are all at the same position
    Point,

    /// The control points are in a line
    Linear,

    /// A simple curve that does not change direction or self-intersect
    Arch,

    /// A curve that changes direction once
    SingleInflectionPoint,

    /// A curve that changes direction twice
    DoubleInflectionPoint,

    /// A curve that can be represented as a quadratic curve rather than a cubic one
    Parabolic,

    /// A curve with a cusp (an abrupt change in direction)
    Cusp,

    /// A curve containing a loop
    Loop
}

///
/// Describes the features of a two-dimensional cubic bezier curve
///
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum CurveFeatures {
    /// All control points are in the same position
    Point,

    /// The control points are in a line
    Linear,

    /// A simple curve that does not change direction or self-intersect
    Arch,

    /// A curve that changes direction once (and the t value where this occurs)
    SingleInflectionPoint(f64),

    /// A curve that changes direction twice (and the t value where this occurs)
    DoubleInflectionPoint(f64, f64),

    /// A curve that can be represented as a quadratic curve rather than a cubic one
    Parabolic,

    /// A curve with a cusp
    Cusp,

    /// A curve containing a loop and the two t values where it self-intersects
    Loop(f64, f64)
}


///
/// Computes an affine transform that translates from an arbitrary bezier curve to one that has the first three control points
/// fixed at w1 = (0,0), w2 = (0, 1) and w3 = (1, 1).
/// 
/// Bezier curves maintain their properties when transformed so this provides a curve with equivalent properties to the input
/// curve but only a single free point (w4). This will return 'None' for the degenerate cases: where two points overlap or
/// where the points are collinear.
///
fn canonical_curve_transform(w1: Vector, w2: Vector, w3: Vector) -> Option<(f64, f64, f64, f64, f64, f64)> {
    // Fetch the coordinates
    let (x0, y0) = (w1.x, w1.y);
    let (x1, y1) = (w2.x, w2.y);
    let (x2, y2) = (w3.x, w3.y);

    let a_divisor = (y2-y1)*(x0-x1)-(x2-x1)*(y0-y1);
    if a_divisor.abs() > SMALL_DIVISOR {
        // Transform is:
        // 
        // [ a, b, c ]   [ x ]
        // [ d, e, f ] . [ y ]
        // [ 0, 0, 1 ]   [ 1 ]
        // 
        // This will move w1 to 0,0, w2 to 0, 1 and w3 to 1, 1, which will form our canonical curve that we use for the classification algorithm
        let a = (-(y0-y1)) / a_divisor;
        let b = (-(x0-x1)) / ((x2-x1)*(y0-y1)-(y2-y1)*(x0-x1));
        let c = -a*x0 - b*y0;
        let d = (y1-y2) / ((x0-x1)*(y2-y1) - (x2-x1)*(y0-y1));
        let e = (x1-x2) / ((y0-y1)*(x2-x1) - (y2-y1)*(x0-x1));
        let f = -d*x0 - e*y0;

        Some((a, b, c, d, e, f))
    } else {
        // Is a degenerate case (points overlap or line up)
        None
    }
}

///
/// Converts a set of points to a 'canonical' curve
/// 
/// This is the curve such that w1 = (0.0), w2 = (1, 0) and w3 = (1, 1), if such a curve exists. The return value is the point w4
/// for this curve.
///
fn to_canonical_curve(w1: Vector, w2: Vector, w3: Vector, w4: Vector) -> Option<Vector> {
    // Retrieve the affine transform for the curve
    if let Some((a, b, c, d, e, f)) = canonical_curve_transform(w1, w2, w3) {
        // Calculate the free point w4 based on the transform
        let x4  = w4.x;
        let y4  = w4.y;

        let x   = a*x4 + b*y4 + c;
        let y   = d*x4 + e*y4 + f;

        Some(Vector::from_components(x, y))
    } else {
        None
    }
}

///
/// Returns the category of a curve given its characteristic point in the canonical form
///
#[inline]
fn characterize_from_canonical_point(b4: (f64, f64)) -> CurveCategory {
    // These coefficients can be used to characterise the curve
    let (x, y)  = b4;
    let delta   = x*x - 2.0*x + 4.0*y - 3.0;

    if delta.abs() <= f64::EPSILON {
        // Curve has a cusp (but we don't know if it's in the range 0<=t<=1)
        if x <= 1.0 {
            // Cusp is within the curve
            CurveCategory::Cusp
        } else {
            // Cusp is outside of the region of this curve
            CurveCategory::Arch
        }
    } else if delta <= 0.0 {
        // Curve has a loop (but we don't know if it's in the range 0<=t<=1)
        if x > 1.0 {
            // Arch or inflection point
            if y > 1.0 {
                CurveCategory::SingleInflectionPoint
            } else {
                CurveCategory::Arch
            }
        } else if x*x - 3.0*x + 3.0*y >= 0.0 {
            if x*x + y*y + x*y - 3.0*x >= 0.0 {
                // Curve lies within the loop region
                CurveCategory::Loop
            } else {
                // Loop is outside of 0<=t<=1 (double point is t < 0)
                CurveCategory::Arch
            }
        } else {
            // Loop is outside of 0<=t<=1 (double point is t > 1)
            CurveCategory::Arch
        }
    } else {
        if y >= 1.0 {
            CurveCategory::SingleInflectionPoint
        } else if x <= 0.0 {
            CurveCategory::DoubleInflectionPoint
        } else {
            if (x-3.0).abs() <= f64::EPSILON && (y-0.0).abs() <= f64::EPSILON {
                CurveCategory::Parabolic
            } else {
                CurveCategory::Arch
            }
        }
    }
}

///
/// Determines the characteristics of a particular bezier curve: whether or not it is an arch, or changes directions
/// (has inflection points), or self-intersects (has a loop)
///
pub fn characterize_cubic_bezier(w1: Vector, w2: Vector, w3: Vector, w4: Vector) -> CurveCategory {
    // b4 is the end point of an equivalent curve with the other control points fixed at (0, 0), (0, 1) and (1, 1) 
    let b4          = to_canonical_curve(w1, w2, w3, w4);

    if let Some(b4) = b4 {
        let x       = b4.x;
        let y       = b4.y;

        characterize_from_canonical_point((x, y))
    } else {
        // Degenerate case: there's no canonical form for this curve
        if w2.is_near(w3, SMALL_DISTANCE) {
            if w2.is_near(w1, SMALL_DISTANCE) {
                if w3.is_near(w4, SMALL_DISTANCE) {
                    // All 4 control points at the same position
                    CurveCategory::Point
                } else {
                    // 3 control points at the same position (makes a line)
                    CurveCategory::Linear
                }
            } else if w3.is_near(w4, SMALL_DISTANCE) {
                // 3 control points at the same position (makes a line)
                CurveCategory::Linear
            } else {
                // w2 and w3 are the same. If w1, w2, w3 and w4 are collinear then we have a straight line, otherwise we have a curve with an inflection point.
                let line        = Line {from: w1, to: w3};
                let (a, b, c)   = line_coefficients_2d(line);

                let distance    = a*w4.x + b*w4.y + c;
                if distance.abs() < SMALL_DISTANCE {
                    // w1, w3 and w4 are collinear (and w2 is the same as w3)
                    CurveCategory::Linear
                } else {
                    // Cubic with inflections at t=0 and t=1 (both control points in the same place but start and end point in different places)
                    CurveCategory::DoubleInflectionPoint
                }
            }
        } else {
            // w1, w2, w3 must be collinear (w2 and w3 are known not to overlap)
            let line        = Line {from: w2, to: w3};
            let (a, b, c)   = line_coefficients_2d(line);

            let distance    = a*w4.x + b*w4.y + c;
            if distance.abs() < SMALL_DISTANCE {
                // All 4 points are in a line
                CurveCategory::Linear
            } else {
                // w2, w3, w4 are not in a line, we can reverse the curve to get a firm result
                characterize_cubic_bezier(w4, w3, w2, w1)
            }
        }
    }
}


///
/// The location of the inflection points for a curve (t-values)
///
enum InflectionPoints {
    Zero,
    One(f64),
    Two(f64, f64)
}

///
/// Finds the inflection points for a curve that has been reduced to our canonical form, given the free point b4
///
fn find_inflection_points(b4: (f64, f64)) -> InflectionPoints {
    // Compute coefficients
    let (x4, y4)    = b4;
    let a           = -3.0+x4+y4;
    let b           = 3.0-x4;

    if a.abs() <= f64::EPSILON {
        // No solution
        InflectionPoints::Zero
    } else {
        // Solve the quadratic for this curve
        let lhs = (-b)/(2.0*a);
        let rhs = (4.0*a + b*b).sqrt()/(2.0*a);

        let t1  = lhs - rhs;
        let t2  = lhs + rhs;

        // Want points between 0 and 1
        if t1 < 0.0 || t1 > 1.0 {
            if t2 < 0.0 || t2 > 1.0 {
                InflectionPoints::Zero
            } else {
                InflectionPoints::One(t2)
            }
        } else {
            if t2 < 0.0 || t2 > 1.0 {
                InflectionPoints::One(t1)
            } else {
                InflectionPoints::Two(t1, t2)
            }
        }
    }
}

impl Into<CurveFeatures> for InflectionPoints {
    #[inline]
    fn into(self) -> CurveFeatures {
        match self {
            InflectionPoints::Zero          => CurveFeatures::Arch,
            InflectionPoints::One(t)        => CurveFeatures::SingleInflectionPoint(t),
            InflectionPoints::Two(t1, t2)   => CurveFeatures::DoubleInflectionPoint(t1, t2)
        }
    }
}

///
/// Determines the characteristics of a paritcular bezier curve: whether or not it is an arch, or changes directions
/// (has inflection points), or self-intersects (has a loop)
///
pub fn features_for_cubic_bezier(w1: Vector, w2: Vector, w3: Vector, w4: Vector, accuracy: f64) -> CurveFeatures {
    // b4 is the end point of an equivalent curve with the other control points fixed at (0, 0), (0, 1) and (1, 1) 
    let b4          = to_canonical_curve(w1, w2, w3, w4);

    if let Some(b4) = b4 {
        // For the inflection points, we rely on the fact that the canonical curve is generated by an affine transform of the original
        // (and the features are invariant in such a situation)
        let x       = b4.x;
        let y       = b4.y;

        match characterize_from_canonical_point((x, y)) {
            CurveCategory::Arch                     => CurveFeatures::Arch,
            CurveCategory::Linear                   => CurveFeatures::Linear,
            CurveCategory::Cusp                     => CurveFeatures::Cusp,
            CurveCategory::Parabolic                => CurveFeatures::Parabolic,
            CurveCategory::Point                    => CurveFeatures::Point,
            CurveCategory::DoubleInflectionPoint    |
            CurveCategory::SingleInflectionPoint    => find_inflection_points((x, y)).into(),
            CurveCategory::Loop                     => {
                let curve       = Bezier::from_control_points(w1, w2, w3, w4);
                let loop_pos    = find_self_intersection_point(&curve, accuracy);

                // TODO: if we can't find the loop_pos, we could probably find a cusp position instead
                loop_pos.map(|(t1, t2)| CurveFeatures::Loop(t1, t2))
                    .unwrap_or(CurveFeatures::Arch)
            }
        }
    } else {
        // Degenerate case: there's no canonical form for this curve
        if w2.is_near(w3, SMALL_DISTANCE) {
            if w2.is_near(w1, SMALL_DISTANCE) {
                if w3.is_near(w4, SMALL_DISTANCE) {
                    // All 4 control points at the same position
                    CurveFeatures::Point
                } else {
                    // 3 control points at the same position (makes a line)
                    CurveFeatures::Linear
                }
            } else if w3.is_near(w4, SMALL_DISTANCE) {
                // 3 control points at the same position (makes a line)
                CurveFeatures::Linear
            } else {
                // w2 and w3 are the same. If w1, w2, w3 and w4 are collinear then we have a straight line, otherwise we have a curve with an inflection point.
                let line        = Line { from: w1, to: w3 };
                let (a, b, c)   = line_coefficients_2d(line);

                let distance    = a*w4.x + b*w4.y + c;
                if distance.abs() < SMALL_DISTANCE {
                    // w1, w3 and w4 are collinear (and w2 is the same as w3)
                    CurveFeatures::Linear
                } else {
                    // Cubic with inflections at t=0 and t=1 (both control points in the same place but start and end point in different places)
                    CurveFeatures::DoubleInflectionPoint(0.0, 1.0)
                }
            }
        } else {
            // w1, w2, w3 must be collinear (w2 and w3 are known not to overlap)
            let line        = Line {from: w2, to: w3};
            let (a, b, c)   = line_coefficients_2d(line);

            let distance    = a*w4.x + b*w4.y + c;
            if distance.abs() < SMALL_DISTANCE {
                // All 4 points are in a line
                CurveFeatures::Linear
            } else {
                // w2, w3, w4 are not in a line, we can reverse the curve to get a firm result
                match features_for_cubic_bezier(w4, w3, w2, w1, accuracy) {
                    CurveFeatures::SingleInflectionPoint(t)         => CurveFeatures::SingleInflectionPoint(1.0-t),
                    CurveFeatures::DoubleInflectionPoint(t1, t2)    => CurveFeatures::DoubleInflectionPoint(1.0-t1, 1.0-t2),
                    CurveFeatures::Loop(t1, t2)                     => CurveFeatures::Loop(1.0-t1, 1.0-t2),
                    other                                           => other
                }
            }
        }
    }
}
*/