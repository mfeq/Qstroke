use crate::MFEKmath::{Bezier, Evaluate, Piecewise, Vector, consts::SMALL_DISTANCE, Interpolator, InterpolationType, EvaluateTransform, ArcLengthParameterization, Parameterization};
use crate::MFEKmath::piecewise::glif::PointData;
use crate::MFEKmath::piecewise::SegmentIterator;
use crate::vec2;
use std::time::Instant;
use flo_curves::line::{line_intersects_line};
use glifparser::{Glif, Outline};

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

fn prepare_in_pw(in_pw: &Piecewise<Bezier>, right_pw: &Piecewise<Interpolator>, left_pw: &Piecewise<Interpolator>, arclen: &ArcLengthParameterization) -> Piecewise<Bezier> {
    let mut out_pw = in_pw.translate(vec2!(0., 0.));

    for cut in &right_pw.cuts {
        let _t = arclen.parameterize(*cut);
        out_pw = out_pw.cut_at_t(_t);
    }

    for cut in &left_pw.cuts {
        let _t = arclen.parameterize(*cut);
        out_pw = out_pw.cut_at_t(_t);
    }

    return out_pw;
}

// takes a vector of beziers and fills in discontinuities with joins
fn fix_path(in_path: Vec<Bezier>, closed: bool) -> Vec<Bezier>
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

                let on_outside = Vector::dot(tangent2, discontinuity_vec) >= 0.;

                
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
            else
            {
                out.push(bezier.clone());
            }
        }
        else
        {
            out.push(bezier.clone());
        }
    }

    return out;
}


pub fn variable_width_stroke(in_pw: &Piecewise<Bezier>, right_pw: &Piecewise<Interpolator>, left_pw: Option<&Piecewise<Interpolator>>) -> Piecewise<Piecewise<Bezier>> {
    let left_pw = left_pw.unwrap_or(right_pw);

 
    let closed = in_pw.is_closed();

    // check if our input path is closed
    // We're gonna keep track of a left line and a right line.
    let mut left_line: Vec<Bezier> = Vec::new();
    let mut right_line: Vec<Bezier> = Vec::new();

    let copy_pw = in_pw.translate(vec2!(0., 0.));

    let arclen = ArcLengthParameterization::from(&copy_pw);
    let in_pw = prepare_in_pw(in_pw, right_pw, left_pw, &arclen);

    let iter = SegmentIterator::new(copy_pw);
    for (bezier, su, eu) in iter {
        let (st, et) = (arclen.parameterize(su) + SMALL_DISTANCE, arclen.parameterize(eu) + SMALL_DISTANCE);
        let mut left_offset = flo_curves::bezier::offset(&bezier, -left_pw.at(st), -left_pw.at(et));
        left_line.append(&mut left_offset);

        let mut right_offset = flo_curves::bezier::offset(&bezier, right_pw.at(st), right_pw.at(et));
        right_line.append(&mut right_offset);
    }
     
    right_line.reverse();
    right_line = right_line.iter()
        .map(|bez| bez.clone().reverse())
        .collect();

    right_line = fix_path(right_line, closed);
    left_line = fix_path(left_line, closed);

    if in_pw.is_closed() {
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

struct VWSHandle {
    t: f64,
    left_offset: Option<f64>,
    right_offset: Option<f64>,
    interpolation: InterpolationType
}

struct InternalVWSHandle {
    t: f64,
    offset: f64,
    interpolation: InterpolationType
}

fn split_stroke_handles(handles: Vec<VWSHandle>) -> [Vec<InternalVWSHandle>; 2] {
    let mut left_internal = Vec::new();
    let mut right_internal = Vec::new();

    for handle in handles {
        if let Some(left_offset) = handle.left_offset {
            left_internal.push(InternalVWSHandle {
                t: handle.t,
                offset: left_offset,
                interpolation: handle.interpolation
            })
        }

        if let Some(right_offset) = handle.right_offset {
            right_internal.push(InternalVWSHandle {
                t: handle.t,
                offset: right_offset,
                interpolation: handle.interpolation
            })
        }
    }

    if left_internal.is_empty() || right_internal.is_empty() {
        panic!("There must be at least one handle on each side of the line. If you want to have one side not be offset use a single 0.0 offset handle.")
    }

    return [left_internal, right_internal];
}

fn create_pw_from_handles(handles: Vec<VWSHandle>) -> [Piecewise<Interpolator>; 2] {
    let split_handles = split_stroke_handles(handles);
    let mut output_pws = Vec::new();

    let iter = split_handles.iter().enumerate();
    for (i, handle_vec) in iter {
        let mut out_pw_segs: Vec<Interpolator> = Vec::new();
        let mut out_pw_cuts: Vec<f64> = Vec::new();

        let mut first = true;
        let mut handle_iter = handle_vec.iter().peekable();
        while let Some(handle) = handle_iter.next() {
            if let Some(peek_handle) = handle_iter.peek() {
                let start_offset = handle.offset;
                let end_offset = peek_handle.offset;

                if first && handle.t < SMALL_DISTANCE {
                    first = false;        
                    out_pw_cuts.push(0.);
                    out_pw_segs.push(Interpolator::new(start_offset, end_offset, handle.interpolation));
                }
                else {
    
                    out_pw_cuts.push(handle.t);
                    out_pw_segs.push(Interpolator::new(start_offset, end_offset, handle.interpolation));
                }
            } else if handle.t < 1. - SMALL_DISTANCE {
                let start_offset = handle.offset;

                out_pw_cuts.push(handle.t);
                out_pw_segs.push(Interpolator::new(start_offset, start_offset, handle.interpolation));
                out_pw_cuts.push(1.);
            } else {
                out_pw_cuts.push(1.);
            }
        }

        output_pws.push(Piecewise::new(out_pw_segs, Some(out_pw_cuts)));
    }


    let right = output_pws.pop().unwrap();
    let left = output_pws.pop().unwrap();
    return [left, right];
}

pub fn variable_width_stroke_glif<U>(path: &Glif<U>) -> Glif<Option<PointData>>
{
    let start = Instant::now();

    // convert our path and pattern to piecewise collections of beziers
    let piece_path = Piecewise::from(path.outline.as_ref().unwrap());
    let mut output_outline: Outline<Option<PointData>> = Vec::new();

    let handles = vec![
        VWSHandle {
            t: 0.0,
            left_offset: Some(1.),
            right_offset: Some(1.),
            interpolation: InterpolationType::Linear
        },
        VWSHandle {
            t: 0.5,
            left_offset: Some(10.),
            right_offset: Some(10.),
            interpolation: InterpolationType::Linear
        },
        VWSHandle {
            t: 1.0,
            left_offset: Some(1.),
            right_offset: Some(1.),
            interpolation: InterpolationType::Linear
        }
    ];

    let pws = create_pw_from_handles(handles);

    for pwpath_contour in piece_path.segs {
        let results = variable_width_stroke(&pwpath_contour, &pws[0], Some(&pws[1]));
        for result_contour in results.segs {
            output_outline.push(result_contour.to_contour());
        }
    }

    let elapsed = start.elapsed();
    print!("{0}", elapsed.as_millis());
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