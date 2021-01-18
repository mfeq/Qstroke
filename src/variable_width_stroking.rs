use crate::MFEKmath::{Bezier, Piecewise, Vector};
use crate::MFEKmath::piecewise::glif::PointData;
use flo_curves::BezierCurve;
use super::consts::SMALL_DISTANCE;

use glifparser::{Glif, Outline};

// need to mvoe this stuff to it's own struct or use flo_curves PathBuilder
fn line_to(path: &mut Vec<Bezier>, to: Vector)
{
    let from = path.last().unwrap().end_point();
    let line = Bezier::from_points(from, from, to, to);

    path.push(line);
}

pub fn variable_width_stroke(path: &Piecewise<Bezier>, start_width: f64, end_width: f64) -> Piecewise<Bezier> {

    // We're gonna keep track of a left line and a right line.
    let mut left_line: Vec<Bezier> = Vec::new();
    let mut right_line: Vec<Bezier> = Vec::new();

   for bezier in &path.curves {
        println!("{:?}", bezier);
        let mut left_offset = flo_curves::bezier::offset(bezier, 10., 10.);

        if let Some(next_point) = left_offset.first() {
            let next_start = next_point.start_point();

            if let Some(last_point) = left_line.last() {
                let last_end = last_point.end_point();
        
                if !last_end.is_near(next_start, SMALL_DISTANCE)
                {
                    // okay we've found a discontinuity for now we're gonna hit it with a bevel
                    //line_to(&mut right_line, next_start)
                }
            }
        }

        left_line.append(&mut left_offset);

        let mut right_offset = flo_curves::bezier::offset(bezier, 0., 0.);

        if let Some(last_point) = right_offset.first() {
            let next_start = last_point.start_point();

            if let Some(next_point) = right_line.last() {
                let last_end = next_point.end_point();
        
                if !last_end.is_near(next_start, SMALL_DISTANCE)
                {
                    // okay we've found a discontinuity for now we're gonna hit it with a bevel
                    //line_to(&mut right_line, next_start)
                }
            }
        }

        right_line.append(&mut right_offset);
    
    }

    
    right_line.reverse();
    let final_right_line: Vec<Bezier> = right_line.iter()
        .map(|bez| bez.clone().reverse())
        .collect();

    if let Some(first_right) = right_line.first() {
       let to = first_right.start_point();

        //line_to(&mut left_line, to);
    }

    left_line.append(&mut right_line);

    if let Some(first_left) = left_line.first() {
        let to = first_left.start_point();
        //line_to(&mut left_line, to);
    }
    

    return Piecewise {
        curves: left_line
    }
}

pub fn variable_width_stroke_glif<U>(path: &Glif<U>) -> Glif<Option<PointData>>
{
    // convert our path and pattern to piecewise collections of beziers
    let piece_path = Piecewise::from(path.outline.as_ref().unwrap());
    let mut output_outline: Outline<Option<PointData>> = Vec::new();


    for contour in piece_path.curves {
        let mut temp_pattern = variable_width_stroke(&contour, 0., 100.);


        let temp_contour = temp_pattern.to_contour();
        output_outline.push(temp_contour);
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