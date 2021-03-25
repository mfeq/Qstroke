use fontforge_sys as fontforge;
use glifparser;
use std::ffi;
use std::fs;
use std::ops::Fn;
use std::os::raw;
use std::ptr;

#[derive(Default, Debug, PartialEq)]
struct SplinePointBitField {
    nonextcp: raw::c_uint,
    noprevcp: raw::c_uint,
    nextcpdef: raw::c_uint,
    prevcpdef: raw::c_uint,
    selected: raw::c_uint,
    nextcpselected: raw::c_uint,
    prevcpselected: raw::c_uint,
    pointtype: raw::c_uint,
    isintersection: raw::c_uint,
    flexy: raw::c_uint,
    flexx: raw::c_uint,
    roundx: raw::c_uint,
    roundy: raw::c_uint,
    dontinterpolate: raw::c_uint,
    ticked: raw::c_uint,
    watched: raw::c_uint,
}

#[rustfmt::skip]
impl SplinePointBitField {
    fn to_bitfield(self) -> (u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32, u32)
    {
        return (
            self.nonextcp,
            self.noprevcp,
            self.nextcpdef,
            self.prevcpdef,
            self.selected,
            self.nextcpselected,
            self.prevcpselected,
            self.pointtype,
            self.isintersection,
            self.flexy,
            self.flexx,
            self.roundx,
            self.roundy,
            self.dontinterpolate,
            self.ticked,
            self.watched,
        );
    }
}

fn ffbasepoint_to_handle(sp: Option<fontforge::BasePoint>) -> glifparser::Handle {
    match sp {
        Some(pt) => glifparser::Handle::At(pt.x as f32, pt.y as f32),
        None => glifparser::Handle::Colocated,
    }
}

fn ffbasepoint_to_point(
    me: fontforge::BasePoint,
    prevcp: Option<fontforge::BasePoint>,
    nextcp: Option<fontforge::BasePoint>,
    should_move: bool,
) -> glifparser::Point<()> {
    let mut p = glifparser::Point::new();
    p.x = me.x as f32;
    p.y = me.y as f32;
    p.b = ffbasepoint_to_handle(prevcp);
    p.a = ffbasepoint_to_handle(nextcp);
    p.ptype = if should_move {
        glifparser::PointType::Move
    } else {
        glifparser::PointType::Curve
    };
    p
}

// This function isn't close to done, don't bother reading it lol
fn ffsplineset_to_outline(ss_in: fontforge::SplineSet) -> glifparser::Outline<()> {
    let mut ret: glifparser::Outline<()> = glifparser::Outline::new();
    let mut splinesets: Vec<_>;
    unsafe {
        let mut ss = ss_in;
        splinesets = vec![ss];
        while ss.next != ptr::null_mut() {
            splinesets.push(*ss.next);
            ss = *ss.next
        }
        for ss2 in splinesets.iter() {
            let mut contour = vec![];
            let first = ss2.first;
            let mut pt = *(ss2.first);
            let mut i = 0;
            while pt.next != ptr::null_mut() {
                // `noprevcp`/`nonextcp` is a method because it's a bitfield in fontforge. It's a
                // bindgen artifact due to the fact Rust has no native support for bitfields.
                let prevcp = if pt.noprevcp() != 0 {
                    None
                } else {
                    Some(pt.prevcp)
                };
                let nextcp = if pt.nonextcp() != 0 {
                    None
                } else {
                    Some(pt.nextcp)
                };
                let should_move = pt.prev == ptr::null_mut() && i == 0;
                contour.push(ffbasepoint_to_point(pt.me, prevcp, nextcp, should_move));
                if (*pt.next).to == first {
                    break;
                }
                pt = *((*pt.next).to);
                i = i + 1;
            }
            ret.push(contour);
        }
    }
    ret
}

// The FontForge `Spline` type shouldn't be made by us.
// Cf. (GitHub) fontforge/fontforge#4676, fontforge/fontforge#4673.
fn make_spline(
    from: *mut fontforge::SplinePoint,
    to: *mut fontforge::SplinePoint,
    order2: bool,
) -> *mut fontforge::Spline {
    unsafe {
        let s = fontforge::SplineMake(from, to, order2 as raw::c_int);
        s
    }
}

type RustSplineSet = Vec<Vec<fontforge::SplinePoint>>;

fn glif_to_ffsplineset<T>(glif: glifparser::Glif<T>) -> (Vec<fontforge::SplineSet>, RustSplineSet) {
    let mut ffsps = vec![];
    for c in glif.outline.unwrap().iter() {
        let mut cffsps = vec![];
        for (idx, p) in c.iter().enumerate() {
            let bp0_1 = fontforge::BasePoint {
                x: p.x as f64,
                y: p.y as f64,
            };
            let (ax, ay) = p.handle_or_colocated(glifparser::WhichHandle::A, |f| f, |f| f);
            let bp0_2 = fontforge::BasePoint {
                x: ax as f64,
                y: ay as f64,
            };
            let (bx, by) = p.handle_or_colocated(glifparser::WhichHandle::B, |f| f, |f| f);
            let bp0_3 = fontforge::BasePoint {
                x: bx as f64,
                y: by as f64,
            };
            let mut spbf = SplinePointBitField {
                ..Default::default()
            };

            let nonextcp = p.a == glifparser::Handle::Colocated;
            let noprevcp = p.b == glifparser::Handle::Colocated;

            spbf.nonextcp = nonextcp as u32;
            spbf.noprevcp = noprevcp as u32;
            spbf.pointtype = fontforge::pointtype_pt_corner;

            let bf = Fn::call(&fontforge::splinepoint::new_bitfield_1, spbf.to_bitfield());

            let sp = fontforge::SplinePoint {
                me: bp0_1,
                prevcp: bp0_3,
                nextcp: bp0_2,
                _bitfield_align_1: [],
                _bitfield_1: bf,
                ptindex: idx as u16,
                // These are for TrueType fonts and don't matter to us.
                ttfindex: 0,
                nextcpindex: 0,
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
                hintmask: ptr::null_mut(),
                name: ptr::null_mut(),
            };
            cffsps.push(sp);
        }
        // Calculating the len here prevents immutable borrows inside mutable borrows
        let cffsps_len = cffsps.len();
        // First, we treat all SplinePoint's as if they form a loop.
        #[rustfmt::skip]
        for idx in 0..cffsps_len {
            if idx == 0 {
                cffsps[idx].prev = make_spline(&mut cffsps[idx] as *mut _, &mut cffsps[cffsps_len - 1] as *mut _, false);
                cffsps[idx].next = make_spline(&mut cffsps[idx] as *mut _, &mut cffsps[idx + 1] as *mut _, false);
            } else if idx == cffsps_len - 1 {
                cffsps[idx].prev = make_spline(&mut cffsps[idx] as *mut _, &mut cffsps[idx - 1] as *mut _, false);
                cffsps[idx].next = make_spline(&mut cffsps[idx] as *mut _, &mut cffsps[0] as *mut _, false);
            } else {
                cffsps[idx].prev = make_spline(&mut cffsps[idx] as *mut _, &mut cffsps[idx - 1] as *mut _, false);
                cffsps[idx].next = make_spline(&mut cffsps[idx] as *mut _, &mut cffsps[idx + 1] as *mut _, false);
            }
            //eprintln!("{} {:?} {:?}", idx, cffsps[idx].prev, cffsps[idx].next);
        }
        // Then, if we know that the contour this SplineSet will refer to is open, we null the
        // appropriate point fields.
        if c[0].ptype == glifparser::PointType::Move {
            cffsps[0].prev = ptr::null_mut();
            cffsps[cffsps_len - 1].next = ptr::null_mut();
        }
        ffsps.push(cffsps);
    }
    let mut ffsss = vec![];
    for spl in ffsps.iter_mut() {
        //eprintln!("SPL: {:?}", spl);
        ffsss.push(fontforge::SplineSet {
            first: spl.first_mut().unwrap(),
            last: spl.last_mut().unwrap(),
            next: ptr::null_mut(),
            spiros: ptr::null_mut(),
            spiro_cnt: 0,
            spiro_max: 0,
            ticked: 0,
            beziers_need_optimizer: 0,
            is_clip_path: 0,
            start_offset: 0,
            contour_name: ptr::null_mut(),
        });
    }
    /*
    for idx in 0..ffsss.len() {
        if idx + 1 > ffsss.len() - 1 {
            break;
        } else {
            ffsss[idx].next = Box::new(ffsss[idx + 1]).as_mut()
        }
    }
    */
    // We return ffsps so its valuable data doesn't go out of scope.
    (ffsss, ffsps)
}

#[derive(Clone, Debug)]
pub struct NibSettings {
    pub nib: String,
    pub path: String,
    pub quiet: bool,
}

pub fn convert_glif(settings: &NibSettings) -> Option<String> {
    if !settings.quiet {
        eprintln!("Reading nib...");
    }
    let nibglif: glifparser::Glif<()> = glifparser::read_ufo_glif(
        &fs::read_to_string(&settings.nib).expect("Nib .glif inaccessible"),
    );
    if !settings.quiet {
        eprintln!("Reading path...");
    }
    let ssglif: glifparser::Glif<()> = glifparser::read_ufo_glif(
        &fs::read_to_string(&settings.path).expect("Path to stroke .glif inaccessible"),
    );
    let mut outglif = ssglif.clone();
    // The "raw"'s are fontforge::SplineSet's that are having their memory managed by Rust.
    let (nibss_raw, _nibss_ffsps) = glif_to_ffsplineset(nibglif);
    let (mut ss_raw, _ss_ffsps) = glif_to_ffsplineset(ssglif);
    // These are Rust Box<_> types for holding types that will be transferred to C
    let mut nibss_boxed = Box::new(nibss_raw[0]);
    let mut ss_vec: Vec<Box<_>> = ss_raw.iter_mut().map(|v| Box::new(v)).collect();
    // These are integer null pointers passable to C
    let nibss = nibss_boxed.as_mut();
    let mut out_ss = vec![];
    unsafe {
        let shape = fontforge::NibIsValid(nibss);
        if shape != 0 {
            let shapetype = fontforge::NibShapeTypeMsg(shape);
            eprintln!(
                "Shape: {}\nCannot stroke!",
                ffi::CStr::from_ptr(shapetype).to_str().unwrap()
            );
            return None;
        }
        let si = fontforge::InitializeStrokeInfo(ptr::null_mut());
        (*si).stroke_type = fontforge::si_type_si_nib;
        (*si).nib = nibss;
        (*si).width = 10.;
        (*si).simplify = -1;
        (*si).rmov = fontforge::stroke_rmov_srmov_none;
        // Do the stroke for each contour. We do it this way to avoid constructing linked lists of
        // SplineSet's. It seems more reliable:
        for ss in ss_vec.iter_mut() {
            let newss = fontforge::SplineSetStroke(*(ss.as_mut()), si, 0);
            if newss == ptr::null_mut() {
                eprintln!("SplineSetStroke returned NULL. Try to recreate the bug in FontForge. If it happens there, report it upstream to FontForge. Otherwise, report it to MFEKstroke bug tracker.");
            }
            //eprintln!("{:?}", *newss);
            out_ss.push(*newss);
        }
    }
    let mut outlines = vec![];
    for oss in out_ss.iter() {
        outlines.push(ffsplineset_to_outline(*oss));
    }
    let mut outline = vec![];
    for o in outlines {
        outline.extend(o);
    }
    outglif.outline = Some(outline);
    Some(glifparser::write_ufo_glif(&outglif))
}
