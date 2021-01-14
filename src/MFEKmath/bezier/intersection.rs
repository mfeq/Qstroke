/*
use super::super::vector::Vector;
use super::super::bezier::Bezier;
use super::characteristics::*;

///
/// If a cubic curve contains a loop, finds the t values where the curve self-intersects
///
pub fn find_self_intersection_point(curve: &Bezier, accuracy: f64) -> Option<Vector> {
    let curve_type = curve.characteristics();

    if curve_type == CurveCategory::Loop {
        find_intersection_point_in_loop(curve, accuracy)
    } else {
        None
    }
}

///
/// Given a curve known to have a loop in it, subdivides it in order to determine where the intersection lies
///
fn find_intersection_point_in_loop(curve: &Bezier, accuracy: f64) -> Option<Vector> {
    use self::CurveCategory::*;

    // The algorithm here is to divide the curve into two. We'll either find a smaller curve with a loop or split the curve in the middle of the loop
    // If we split in the middle of the loop, we use the bezier clipping algorithm to find where the two sides intersect
    let (left, right)           = curve.subdivide(0.5);
    let (left_type, right_type) = (left.characteristics(), right.characteristics());

    match (left_type, right_type) {
        (Loop, Loop) => {
            // If both sides are loops then we've split the original curve at the intersection point
            unimplemented!("Need support for a loop where we hit the intersection point")
        }

        (Loop, _) => {
            // Loop is in the left side
            find_intersection_point_in_loop(&left, accuracy)
        },

        (_, Loop) => {
            // Loop is in the right side
            find_intersection_point_in_loop(&right, accuracy)
        }

        (_, _) => {
            // Can find the intersection by using the clipping algorithm
            let intersections = curve_intersects_curve_clip(&left, &right, accuracy);

            if intersections.len() == 0 && left.start_point().is_near_to(&right.end_point(), accuracy) {
                // Didn't find an intersection but the left and right curves start and end at the same position
                return Some((left.t_for_t(0.0), right.t_for_t(1.0)));
            }

            assert!(intersections.len() != 0);

            if intersections.len() == 1 {
                // Only found a single intersection
                intersections.into_iter().nth(0)
                    .map(|(t1, t2)| (left.t_for_t(t1), right.t_for_t(t2)))
            } else {
                // Intersection may include the point between the left and right curves (ignore any point that's at t=1 on the left or t=0 on the right)
                intersections.into_iter()
                    .filter(|(t1, t2)| *t1 < 1.0 && *t2 > 0.0)
                    .nth(0)
                    .map(|(t1, t2)| (left.t_for_t(t1), right.t_for_t(t2)))
            }
        }
    }
}


///
/// Determines the points at which two curves intersect using the Bezier clipping algorithm
/// 
fn curve_intersects_curve_clip_inner(curve1: Bezier, curve2: Bezier, accuracy_squared: f64) -> Vec<(f64, f64)> {
    // Overlapping curves should be treated separately (the clipping algorithm will just match all of the points)
    let overlaps = overlapping_region(&curve1, &curve2);
    if let Some(((c1_t1, c1_t2), (c2_t1, c2_t2))) = overlaps {
        // Convert the overlapping region back to t values for the original curve
        let c1_t1 = curve1.t_for_t(c1_t1);
        let c1_t2 = curve1.t_for_t(c1_t2);
        let c2_t1 = curve2.t_for_t(c2_t1);
        let c2_t2 = curve2.t_for_t(c2_t2);

        if c1_t1 == c1_t2 || c2_t1 == c2_t2 {
            // Overlapped at a single point, so only one intersection
            return smallvec![(c1_t1, c2_t1)];
        } else {
            // Overlapping curves cross at both points
            return smallvec![(c1_t1, c2_t1), (c1_t2, c2_t2)];
        }
    }

    // We'll iterate on the two curves
    let mut curve1 = curve1;
    let mut curve2 = curve2;

    // If a curve stops shrinking, we need to subdivide it to continue the match
    let mut curve1_last_len = curve_hull_length_sq(&curve1);
    let mut curve2_last_len = curve_hull_length_sq(&curve2);

    // Edge case: 0-length curves have no match
    if curve1_last_len == 0.0 { return smallvec![]; }
    if curve2_last_len == 0.0 { return smallvec![]; }

    // Iterate to refine the match
    loop {
        let curve2_len = if curve2_last_len > accuracy_squared {
            // Clip curve2 against curve1
            let clip_t  = clip(&curve2, &curve1);
            let clip_t  = match clip_t {
                ClipResult::None                    => { return smallvec![]; },
                ClipResult::Some(clip_t)            => clip_t,
                ClipResult::SecondCurveIsLinear     => { 
                    return intersections_with_linear_section(&curve1, &curve2)
                        .into_iter()
                        .map(|(t1, t2)| (curve1.t_for_t(t1), curve2.t_for_t(t2)))
                        .collect(); 
                }
            };

            curve2 = curve2.subsection(clip_t.0, clip_t.1);

            // Work out the length of the new curve
            curve_hull_length_sq(&curve2)
        } else { 
            curve2_last_len
        };

        let curve1_len = if curve1_last_len > accuracy_squared {
            // Clip curve1 against curve2
            let clip_t  = clip(&curve1, &curve2);
            let clip_t  = match clip_t {
                ClipResult::None                    => { return smallvec![]; },
                ClipResult::Some(clip_t)            => clip_t,
                ClipResult::SecondCurveIsLinear     => { 
                    return intersections_with_linear_section(&curve2, &curve1)
                        .into_iter()
                        .map(|(t2, t1)| (curve1.t_for_t(t1), curve2.t_for_t(t2)))
                        .collect(); 
                }
            };

            curve1 = curve1.subsection(clip_t.0, clip_t.1);

            // Work out the length of the new curve
            curve_hull_length_sq(&curve1)
        } else {
            curve1_last_len
        };

        if curve1_len <= accuracy_squared && curve2_len <= accuracy_squared {
            // Found a point to the required accuracy: return it, in coordinates relative to the original curve
            if curve1.fast_bounding_box::<Bounds<_>>().overlaps(&curve2.fast_bounding_box::<Bounds<_>>()) {
                let (t_min1, t_max1) = curve1.original_curve_t_values();
                let (t_min2, t_max2) = curve2.original_curve_t_values();

                return smallvec![((t_min1+t_max1)*0.5, (t_min2+t_max2)*0.5)];
            } else {
                // Clipping algorithm found a point, but the two curves do not actually overlap, so reject them
                return smallvec![];
            }
        }

        if (curve1_last_len*0.8) <= curve1_len && (curve2_last_len*0.8) <= curve2_len {
            // If neither curve shrunk by 20%, then subdivide the one that shrunk the least
            if curve1_len/curve1_last_len > curve2_len/curve2_last_len {
                // Curve1 shrunk less than curve2
                let (left, right)   = (curve1.subsection(0.0, 0.5), curve1.subsection(0.5, 1.0));
                let left            = curve_intersects_curve_clip_inner(left, curve2.clone(), accuracy_squared);
                let right           = curve_intersects_curve_clip_inner(right, curve2, accuracy_squared);

                return join_subsections(&curve1, left, right, accuracy_squared);
            } else {
                // Curve2 shrunk less than curve1
                let (left, right)   = (curve2.subsection(0.0, 0.5), curve2.subsection(0.5, 1.0));
                let left            = curve_intersects_curve_clip_inner(curve1.clone(), left, accuracy_squared);
                let right           = curve_intersects_curve_clip_inner(curve1.clone(), right, accuracy_squared);

                return join_subsections(&curve1, left, right, accuracy_squared);
            }
        }

        // Update the last lengths
        curve1_last_len = curve1_len;
        curve2_last_len = curve2_len;
    }
}

///
/// Determines the points at which two curves intersect using the Bezier clipping
/// algorihtm
/// 
pub fn curve_intersects_curve_clip(curve1: &Bezier, curve2: &Bezier, accuracy: f64) -> Vec<(f64, f64)> {
    // Start with the entire span of both curves
    let curve1 = curve1.section(0.0, 1.0);
    let curve2 = curve2.section(0.0, 1.0);

    // Perform the clipping algorithm on these curves
    curve_intersects_curve_clip_inner(curve1, curve2, accuracy*accuracy)
}
*/