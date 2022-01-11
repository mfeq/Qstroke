use std::ffi;
use std::fs;
use std::path::PathBuf as FsPathBuf;

use glifparser::glif::{CapType, InterpolationType, JoinType, VWSContour, VWSHandle};
use MFEKmath::{variable_width_stroke, Piecewise, VWSSettings};

use glifparser::{Glif, Outline, PointData};
use glifparser::glif::mfek::{ContourOperations, MFEKGlif};

use clap::{App, AppSettings, Arg};

pub fn clap_app() -> clap::App<'static> {
    App::new("CWS")
        .setting(AppSettings::DeriveDisplayOrder)
        .alias("constant")
        .alias("cws")
        .about("Takes a .glif file and strokes it at a constant width.")
        .version("0.1")
        .author("Fredrick R. Brennan <copypasteⒶkittens⊙ph>; Matthew Blanchard <matthewrblanchard@gmail.com>")
        .arg(Arg::new("input")
            .display_order(1)
            .short('i')
            .long("input")
            .takes_value(true)
            .help("The path to the input file.")
            .required(true))
        .arg(Arg::new("output")
            .display_order(2)
            .short('o')
            .long("output")
            .takes_value(true)
            .help("The path where the output will be saved.")
            .required(true))
        .arg(Arg::new("startcap")
            .long("startcap")
            .short('s')
            .takes_value(true)
            .help(r#"Either the constant strings "circle", "round" or "square", or a .glif file."#)
            .default_value("circle"))
        .arg(Arg::new("endcap")
            .long("endcap")
            .short('e')
            .takes_value(true)
            .help(r#"Either the constant strings "circle", "round" or "square", or a .glif file."#)
            .default_value("circle"))
        .arg(Arg::new("jointype")
            .long("jointype")
            .short('j')
            .takes_value(true)
            .possible_values(&["round", "circle", "miter", "bevel"])
            .help("How to join discontinuous splines")
            .default_value("round"))
        .arg(Arg::new("width")
            .long("width")
            .short('w')
            .takes_value(true)
            .help(r#"<f64> Constant stroke width."#)
            .validator(super::arg_validator_positive_f64)
            .conflicts_with("left")
            .conflicts_with("right")
            .required_unless_present_all(&["left", "right"]))
        .arg(Arg::new("left")
            .long("left")
            .short('l')
            .takes_value(true)
            .help(r#"<f64> Constant stroke width (left)."#)
            .validator(super::arg_validator_positive_f64)
            .requires("right"))
        .arg(Arg::new("right")
            .long("right")
            .short('r')
            .takes_value(true)
            .help(r#"<f64> Constant stroke width (right)."#)
            .validator(super::arg_validator_positive_f64)
            .requires("left"))
        .arg(Arg::new("remove-internal")
            .long("remove-internal")
            .short('I')
            .takes_value(false)
            .help(r#"Remove internal contour"#))
        .arg(Arg::new("remove-external")
            .long("remove-external")
            .short('E')
            .takes_value(false)
            .help(r#"Remove external contour"#))
        .arg(Arg::new("segmentwise")
            .long("segmentwise")
            .short('S')
            .takes_value(false)
            .help(r#"Join all segments with caps (stroke all Bézier segments one by one)"#))
}

#[derive(Debug)]
struct CWSSettings<PD: PointData> {
    vws_settings: VWSSettings<PD>,
    left: f64,
    right: f64,
    jointype: JoinType,
    startcap: CapType,
    endcap: CapType,
    remove_internal: bool,
    remove_external: bool,
    segmentwise: bool,
}

fn make_vws_contours(path: &Glif<()>, settings: &CWSSettings<()>) -> Vec<VWSContour> {
    let vws_contour = VWSContour {
        join_type: settings.jointype,
        cap_start_type: settings.startcap,
        cap_end_type: settings.endcap,
        handles: vec![], // to be populated based on number of points
        remove_internal: settings.remove_internal,
        remove_external: settings.remove_external,
    };

    let mut vws_contours = vec![vws_contour; path.outline.as_ref().unwrap().len()];

    let vws_handle = VWSHandle {
        left_offset: settings.left,
        right_offset: settings.right,
        tangent_offset: 0.0,
        interpolation: InterpolationType::Linear,
    };

    for outline in path.outline.as_ref() {
        for (cidx, contour) in outline.iter().enumerate() {
            let pointiter = contour.iter().enumerate();

            for (_, _) in pointiter {
                vws_contours[cidx].handles.push(vws_handle);
            }
            vws_contours[cidx].handles.push(vws_handle);
        }
    }

    vws_contours
}

fn constant_width_stroke_glifjson(path: Glif<()>, settings: &CWSSettings<()>) -> MFEKGlif<()> {
    let vws_contours = make_vws_contours(&path, settings);
    let mut ret: MFEKGlif<()> = path.into();
    for (i, contour) in ret.layers[0].outline.iter_mut().enumerate() {
        contour.operation = Some(ContourOperations::VariableWidthStroke { data: vws_contours[i].clone() });
    }
    ret
}

fn constant_width_stroke(path: &glifparser::Glif<()>, settings: &CWSSettings<()>) -> Outline<()> {
    // convert our path and pattern to piecewise collections of beziers
    let piece_path = Piecewise::from(path.outline.as_ref().unwrap());
    let mut output_outline: Outline<()> = Vec::new();
    let vws_contours = make_vws_contours(path, settings);

    let iter = piece_path.segs.iter().enumerate();
    for (i, pwpath_contour) in iter {
        let vws_contour = &vws_contours[i];

        let results = if settings.segmentwise {
            pwpath_contour
                .segs
                .iter()
                .map(|p| {
                    variable_width_stroke(
                        &Piecewise::new(vec![p.clone()], None),
                        &vws_contour,
                        &settings.vws_settings,
                    )
                })
                .collect()
        } else {
            vec![variable_width_stroke(
                &pwpath_contour,
                &vws_contour,
                &settings.vws_settings,
            )]
        };

        for result_outline in results {
            for result_contour in result_outline.segs.iter() {
                output_outline.push(result_contour.to_contour());
            }
        }
    }
    output_outline
}

// Constant width stroking is really just a special case of variable width stroking. So, we take
// the width, divide by two to make handles from it, and use those to stroke at a tangent of 0.
//
// Some of this was copied from MFEK/math.rlib file src/variable_width_stroking.rs fn variable_width_stroke_glif
pub fn cws_cli(matches: &clap::ArgMatches) {
    fn custom_cap_if_requested(ct: CapType, input_file: &str) -> Option<Glif<()>> {
        if ct == CapType::Custom {
            let path: glifparser::Glif<()> =
                glifparser::read(&fs::read_to_string(input_file).expect("Failed to read file!"))
                    .unwrap(); // TODO: Proper error handling!
            Some(path)
        } else {
            None
        }
    }

    let input_file = matches.value_of_os("input").unwrap();
    let output_file = matches.value_of_os("output").unwrap();
    let startcap: CapType = (matches.value_of("startcap").unwrap())
        .parse()
        .expect("Invalid cap/join");
    let endcap: CapType = (matches.value_of("endcap").unwrap())
        .parse()
        .expect("Invalid cap/join");
    let jointype: JoinType = (matches.value_of("jointype").unwrap())
        .parse()
        .expect("Invalid cap/join");
    let remove_internal = matches.is_present("remove-internal");
    let remove_external = matches.is_present("remove-external");
    let segmentwise = matches.is_present("segmentwise");
    let left: f64;
    let right: f64;

    if matches.is_present("left") {
        left = matches.value_of("left").unwrap().parse().unwrap();
        right = matches.value_of("right").unwrap().parse().unwrap();
    } else {
        let width: f64 = matches.value_of("width").unwrap().parse().unwrap();
        left = width / 2.0;
        right = width / 2.0;
    }

    // TODO: Proper error handling!
    let path: glifparser::Glif<()> =
        glifparser::read(&fs::read_to_string(input_file).expect("Failed to read file!")).unwrap();

    let vws_settings = VWSSettings {
        cap_custom_end: custom_cap_if_requested(endcap, matches.value_of("endcap").unwrap()),
        cap_custom_start: custom_cap_if_requested(startcap, matches.value_of("startcap").unwrap()),
    };

    let cws_settings = CWSSettings {
        vws_settings,
        left,
        right,
        startcap,
        endcap,
        jointype,
        remove_internal,
        remove_external,
        segmentwise,
    };

    let oss = match FsPathBuf::from(output_file).extension() {
        Some(oss) => {
            oss.to_ascii_lowercase()
        },
        None => ffi::OsString::from("glif")
    };
        
    if &oss == &ffi::OsString::from("glifjson") {
        let out = constant_width_stroke_glifjson(path, &cws_settings);
        fs::write(output_file, serde_json::to_vec_pretty(&out).unwrap()).expect("Write failed");
    } else if &oss == &ffi::OsString::from("glif") {
        let output_outline = path
            .outline
            .as_ref()
            .map(|_| Some(constant_width_stroke(&path, &cws_settings)))
            .unwrap_or_else(|| None);

        let out = Glif {
            outline: output_outline,
            order: path.order,
            anchors: path.anchors.clone(),
            width: path.width,
            unicode: path.unicode,
            name: path.name,
            lib: path.lib,
            components: path.components,
            guidelines: path.guidelines,
            images: path.images,
            note: path.note,
            filename: path.filename,
        };

        let glifstring = glifparser::write(&out).unwrap(); // TODO: Proper error handling!
        fs::write(output_file, glifstring).expect("Unable to write file");
    }
}
