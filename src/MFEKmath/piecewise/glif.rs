use super::{Bezier, Outline, Piecewise, Vector};
use glifparser::{Contour, PointType, Handle};

// stub PointData out here, really not sure how I should be handnling this because we need a concrete
// type to construct our own glif
pub struct PointData;

impl<T> From<&Outline<T>> for Piecewise<Piecewise<Bezier>>
{
    fn from(outline: &Outline<T>) -> Self { 
        let mut ret = Piecewise {
            curves: Vec::new(),
        };
    
        for contour in outline
        {
            ret.curves.push(Piecewise::from(contour));
        }
    
        return ret;
    }
}

impl Piecewise<Piecewise<Bezier>> {
    pub fn to_outline(&self) -> Outline<Option<PointData>> {
        let mut output_outline: Outline<Option<PointData>> = Outline::new();

        for contour in &self.curves
        {
            output_outline.push(contour.to_contour());
        }

        return output_outline;
    }
}

impl<T> From<&Contour<T>> for Piecewise<Bezier>
{
    fn from(contour: &Contour<T>) -> Self {
        let mut ret = Piecewise {
            curves: Vec::new(),
        };

        let mut lastpoint: Option<&glifparser::Point<T>> = None;

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

        let firstpoint = contour.first().unwrap();
        if firstpoint.ptype != PointType::Move {
            ret.curves.push(Bezier::from(&lastpoint.unwrap(), firstpoint));
        }

        return ret
    }
}

impl Piecewise<Bezier> {
    pub fn to_contour(&self) -> Contour<Option<PointData>> {
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
}