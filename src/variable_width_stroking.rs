use crate::MFEKmath::{Bezier, Evaluate, Piecewise, Vector, consts::SMALL_DISTANCE};
use crate::MFEKmath::piecewise::glif::PointData;

use glifparser::{Glif, Outline};

// need to mvoe this stuff to it's own struct or use flo_curves PathBuilder
fn line_to(path: &mut Vec<Bezier>, to: Vector)
{
    let from = path.last().unwrap().end_point();
    let line = Bezier::from_points(from, from, to, to);

    path.push(line);
}

fn is_closed(path: &Vec<Bezier>) -> bool
{
    let first_point = path.first().unwrap().to_control_points()[0];
    let last_point = path.last().unwrap().to_control_points()[3];

    return first_point.is_near(last_point, SMALL_DISTANCE)
}

// takes a vector of beziers and fills in discontinuities with joins
fn fix_path(in_path: Vec<Bezier>) -> Vec<Bezier>
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

                //TODO: implement more complicated joins
                out.push(bezier.clone());
                line_to(&mut out, next_start);
            }
            else
            {
                out.push(bezier.clone());
            }
        }
        else if is_closed(&in_path)
        {
            // our path is closed and if there's not a next point we need to make sure that our current
            // and last curve matches up with the first one

            let first_point = in_path.first().unwrap().start_point();
            let last_end = bezier.end_point();

            if !last_end.is_near(first_point, SMALL_DISTANCE)
            {
                //TODO: implement more complicated joins
                out.push(bezier.clone());
                line_to(&mut out, first_point);
            }
        }
        else
        {
            out.push(bezier.clone());
        }
    }

    return out;
}

pub fn variable_width_stroke(path: &Piecewise<Bezier>, start_width: f64, end_width: f64) -> Piecewise<Piecewise<Bezier>> {
 
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
    
    }
     
    right_line.reverse();
    let mut final_right_line: Vec<Bezier> = right_line.iter()
        .map(|bez| bez.clone().reverse())
        .collect();

    
    left_line = fix_path(left_line);
    final_right_line = fix_path(final_right_line);

    if path.is_closed() {
        let mut out = Vec::new();

        let left_pw = Piecewise::new(left_line, None);
        let right_pw = Piecewise::new(final_right_line, None);

        out.push(left_pw);
        out.push(right_pw);
        
        return Piecewise::new(out, None);
    }
    else
    {
        let mut out_vec = left_line;

        // path is not closed we need to cap the ends, for now that means a bevel
        let to = final_right_line.last().unwrap().to_control_points();
        line_to(&mut out_vec, to[0]);

        // append the right line to the left now that we've connected them
        out_vec.append(&mut final_right_line);

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
        let mut results = variable_width_stroke(&pwpath_contour, 0., 100.);
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