use super::super::Vector;
use flo_curves::bezier::{BezierCurve, BezierCurveFactory};
use flo_curves::geo::Geo;
use flo_curves::Coordinate;

use super::Bezier;

impl Geo for Bezier {
    type Point = Vector;
}

impl BezierCurveFactory for Bezier {
    fn from_points(start: Vector, (control_point1, control_point2): (Vector, Vector), end: Vector) -> Self {
        let bez = Bezier::from_points(start, control_point1, control_point2, end);
        return bez;
    }
}

impl BezierCurve for Bezier {
    fn start_point(&self) -> Self::Point
    {
        self.to_control_points()[0]
    }

    fn end_point(&self) -> Self::Point
    {
        self.to_control_points()[3]
    }

    fn control_points(&self) -> (Self::Point, Self::Point)
    {
        let cp = self.to_control_points();
        (cp[1], cp[2])
    }
}