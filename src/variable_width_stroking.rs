use crate::MFEKmath::{Bezier, Evaluate, Piecewise, Vector, consts::SMALL_DISTANCE};
use crate::MFEKmath::piecewise::glif::PointData;
use flo_curves::line::{line_intersects_line};
use flo_curves::bezier::{characterize_curve, CurveCategory};

use glifparser::{Glif, Outline};

enum Winding {
    Clockwise,
    Counter
}

fn find_discontinuity_intersection(from: Vector, to: Vector, tangent1: Vector, tangent2: Vector) -> Option<Vector>
{
    // create rays starting at from and to and pointing in the direction of the respective tangent
    let ray1 = (from, from + tangent1*200.);
    let ray2 = (to, to + tangent2*200.);

    return line_intersects_line(&ray1, &ray2);
}

// need to mvoe this stuff to it's own struct or use flo_curves PathBuilder
fn line_to(path: &mut Vec<Bezier>, to: Vector)
{
    let from = path.last().unwrap().end_point();
    let line = Bezier::from_points(from, from, to, to);

    path.push(line);
}

fn miter_to(path: &mut Vec<Bezier>, to: Vector, tangent1: Vector, tangent2: Vector)
{
    let from = path.last().unwrap().end_point();
    let _intersection = find_discontinuity_intersection(from, to, tangent1, tangent2);

    if let Some(intersection) = _intersection {
        // found an intersection so we draw a line to it
        line_to(path, intersection);
        line_to(path, to);
    }
    else
    {
        // if no intersection can be found we default to a bevel
        line_to(path, to);
    }
}

// https://www.stat.auckland.ac.nz/~paul/Reports/VWline/line-styles/line-styles.html
fn arc_to(path: &mut Vec<Bezier>, to: Vector, tangent1: Vector, tangent2: Vector)
{
    let from = path.last().unwrap().end_point();
    let _intersection = find_discontinuity_intersection(from, to, tangent1, tangent2);
    
    if let Some(intersection) = _intersection {
        let radius = f64::min(from.distance(intersection), to.distance(intersection));
        let angle = f64::acos(from.dot(to) / (from.magnitude() * to.magnitude()));
        let dist_along_tangents = radius*(4./(3.*(1./f64::cos(angle/2.) + 1.)));

        let arc = Bezier::from_points(from, from + tangent1 * dist_along_tangents, to + tangent2 * dist_along_tangents, to);
        path.push(arc);
    }
    else
    {
        let radius = from.distance(to) * (2./3.);
        let angle = f64::acos(from.dot(to) / (from.magnitude() * to.magnitude()));
        let dist_along_tangents = radius*(4./(3.*(1./f64::cos(angle/2.) + 1.)));

        let arc = Bezier::from_points(
            from,
            from + tangent1 * dist_along_tangents,
            to + tangent2 * dist_along_tangents,
            to
        );
        path.push(arc);
    }
}

// takes a vector of beziers and fills in discontinuities with joins
fn fix_path(in_path: Vec<Bezier>, winding: Winding, closed: bool) -> Vec<Bezier>
{
    let mut out: Vec<Bezier> = Vec::new();

    let mut path_iter = in_path.iter().peekable();
    
    while let Some(bezier) = path_iter.next() {
        if let Some(next_bezier) = path_iter.peek()
        {
            let next_start = next_bezier.start_point();
            let last_end = bezier.end_point();
            if !last_end.is_near(next_start, SMALL_DISTANCE)
            {
                // the end of our last curve doesn't match up with the start of our next so we need to
                // deal with the discontinuity be creating a join
                
                let tangent1 = bezier.tangent_at(1.).normalize(); 
                let tangent2 = -next_bezier.tangent_at(0.).normalize();
                let discontinuity_vec = next_start - last_end;

                let on_outside = match winding {
                    Winding::Clockwise => Vector::dot(tangent2, discontinuity_vec) >= 0.,
                    Winding::Counter => Vector::dot(tangent2, -discontinuity_vec) >= 0.
                };
                
                if !on_outside {
                    //TODO: implement more complicated joins
                    out.push(bezier.clone());
                    arc_to(&mut out, next_start, tangent1, tangent2);
                }
                else
                {
                    // we're inside so we default to a bevel
                    out.push(bezier.clone());
                    line_to(&mut out, next_start);
                }
            }
            else
            {
                out.push(bezier.clone());
            }
        }
        else if closed
        {
            // our path is closed and if there's not a next point we need to make sure that our current
            // and last curve matches up with the first one

            let first_bez = in_path.first().unwrap();
            let first_point = first_bez.start_point();
            let last_end = bezier.end_point();

            if !last_end.is_near(first_point, SMALL_DISTANCE)
            {
                let tangent1 = bezier.tangent_at(1.).normalize(); 
                let tangent2 = -first_bez.tangent_at(0.).normalize();

                //TODO: implement more complicated joins
                out.push(bezier.clone());
                arc_to(&mut out, first_point, tangent1, tangent2);
            }
        }
        else
        {
            out.push(bezier.clone());
        }
    }

    return out;
}

pub fn variable_width_stroke(path: &Piecewise<Bezier>) -> Piecewise<Piecewise<Bezier>> {
 
    // check if our input path is closed
    // We're gonna keep track of a left line and a right line.
    let mut left_line: Vec<Bezier> = Vec::new();
    let mut right_line: Vec<Bezier> = Vec::new();


    let iter = path.segs.iter().enumerate();
    for bezier in &path.segs {
        let mut left_offset = flo_curves::bezier::offset(bezier, 10., 10.);
        left_line.append(&mut left_offset);

        let mut right_offset = flo_curves::bezier::offset(bezier, -10., -10.);
        right_line.append(&mut right_offset);

        if characterize_curve(bezier) == CurveCategory::Linear {
            assert!(characterize_curve(right_line.last().unwrap()) == CurveCategory::Linear);
        }
    
    }
     
    right_line.reverse();
    right_line = right_line.iter()
        .map(|bez| bez.clone().reverse())
        .collect();

    let closed = path.is_closed();
    right_line = fix_path(right_line, Winding::Clockwise, closed);
    left_line = fix_path(left_line, Winding::Counter, closed);

    if path.is_closed() {
        let mut out = Vec::new();

        let left_pw = Piecewise::new(left_line, None);
        let right_pw = Piecewise::new(right_line, None);

        out.push(left_pw);
        out.push(right_pw);
        
        return Piecewise::new(out, None);
    }
    else
    {
        let mut out_vec = left_line;

        // path is not closed we need to cap the ends, for now that means a bevel
        let to = right_line.last().unwrap().to_control_points();
        line_to(&mut out_vec, to[0]);

        // append the right line to the left now that we've connected them
        out_vec.append(&mut right_line);

        // we need to close the beginning now 
        let to = out_vec.first().unwrap().to_control_points();
        line_to(&mut out_vec, to[0]);

        let inner = Piecewise::new(out_vec, None);
        return Piecewise::new(vec![inner], None);
    }
}

pub fn variable_width_stroke_glif<U>(path: &Glif<U>) -> Glif<Option<PointData>>
{
    // convert our path and pattern to piecewise collections of beziers
    let piece_path = Piecewise::from(path.outline.as_ref().unwrap());
    let mut output_outline: Outline<Option<PointData>> = Vec::new();


    for pwpath_contour in piece_path.segs {
        let results = variable_width_stroke(&pwpath_contour);
        for result_contour in results.segs {
            output_outline.push(result_contour.to_contour());
        }
    }

    return Glif {
        outline: Some(output_outline),
        order: path.order, // default when only corners
        anchors: path.anchors.clone(),
        width: path.width,
        unicode: path.unicode,
        name: path.name.clone(),
        format: 2,
    };
}