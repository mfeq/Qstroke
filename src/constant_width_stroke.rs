use std::fs;

use MFEKmath::piecewise::glif::PointData;
use MFEKmath::variable_width_stroke;
use MFEKmath::variable_width_stroking::*;
use MFEKmath::Piecewise;
use MFEKmath::VWSHandle;

use glifparser::{Glif, Outline};

use clap::{App, Arg};

pub fn clap_app() -> clap::App<'static, 'static> {
    App::new("CWS")
        .alias("constant")
        .alias("cws")
        .about("Takes a .glif file and strokes it at a constant width.")
        .version("0.0")
        .author("Fredrick R. Brennan <copypasteⒶkittens⊙ph>; Matthew Blanchard <matthewrblanchard@gmail.com>")
        .arg(Arg::with_name("input")
            .display_order(1)
            .short("in")
            .long("input")
            .takes_value(true)
            .help("The path to the input file.")
            .required(true))
        .arg(Arg::with_name("output")
            .display_order(2)
            .short("out")
            .long("output")
            .takes_value(true)
            .help("The path where the output will be saved.")
            .required(true))
        .arg(Arg::with_name("startcap")
            .long("startcap")
            .short("s")
            .takes_value(true)
            .help(r#"Either the constant strings "round" or "square", or a .glif file."#)
            .default_value("round"))
        .arg(Arg::with_name("endcap")
            .long("endcap")
            .short("e")
            .takes_value(true)
            .help(r#"Either the constant strings "round" or "square", or a .glif file."#)
            .default_value("round"))
        .arg(Arg::with_name("jointype")
            .long("jointype")
            .short("j")
            .takes_value(true)
            .help(r#"Either of the constant strings "round", "miter", or "bevel"."#)
            .default_value("round"))
        .arg(Arg::with_name("width")
            .long("width")
            .short("w")
            .takes_value(true)
            .help(r#"<f64> Constant stroke width."#)
            .validator(super::arg_validator_positive_f64)
            .required(true))
}

// Constant width stroking is really just a special case of variable width stroking. So, we take
// the width, divide by two to make handles from it, and use those to stroke at a tangent of 0.
//
// Some of this was copied from MFEK/math.rlib file src/variable_width_stroking.rs fn variable_width_stroke_glif
pub fn cws_cli(matches: &clap::ArgMatches) {
    fn str_to_jointype(s: &str) -> JoinType {
        match s {
            "bevel" => JoinType::Bevel,
            "miter" => JoinType::Miter,
            "round" => JoinType::Round,
            _ => unimplemented!(),
        }
    }

    fn str_to_cap(s: &str) -> CapType {
        match s {
            "round" => CapType::Round,
            "square" => CapType::Square,
            _ => CapType::Custom,
        }
    }

    fn custom_cap_if_requested(ct: CapType, input_file: &str) -> Option<Glif<Option<PointData>>> {
        if ct == CapType::Custom {
            let path: glifparser::Glif<Option<PointData>> = glifparser::read_ufo_glif(
                &fs::read_to_string(input_file).expect("Failed to read file!"),
            );
            Some(path)
        } else {
            None
        }
    }

    let input_file = matches.value_of("input").unwrap();
    let output_file = matches.value_of("output").unwrap();
    let startcap = str_to_cap(matches.value_of("startcap").unwrap());
    let endcap = str_to_cap(matches.value_of("endcap").unwrap());
    let jointype = str_to_jointype(matches.value_of("jointype").unwrap());

    let width: f64 = matches.value_of("width").unwrap().parse().unwrap();
    let path: glifparser::Glif<Option<PointData>> =
        glifparser::read_ufo_glif(&fs::read_to_string(input_file).expect("Failed to read file!"));

    let vws_contour = VWSContour {
        id: 0,
        join_type: jointype,
        cap_start_type: startcap,
        cap_end_type: endcap,
        handles: vec![], // to be populated based on number of points
    };

    let settings = VWSSettings {
        cap_custom_end: custom_cap_if_requested(endcap, matches.value_of("endcap").unwrap()),
        cap_custom_start: custom_cap_if_requested(startcap, matches.value_of("startcap").unwrap()),
    };

    // convert our path and pattern to piecewise collections of beziers
    let piece_path = Piecewise::from(path.outline.as_ref().unwrap());
    let mut output_outline: Outline<Option<PointData>> = Vec::new();

    let mut vws_contours = vec![vws_contour; path.outline.as_ref().unwrap().len()];

    let vws_handle = VWSHandle {
        left_offset: width / 2.0,
        right_offset: width / 2.0,
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

    let iter = piece_path.segs.iter().enumerate();
    for (i, pwpath_contour) in iter {
        let vws_contour = &vws_contours[i];

        let results = variable_width_stroke(&pwpath_contour, &vws_contour, &settings);
        for result_contour in results.segs {
            output_outline.push(result_contour.to_contour());
        }
    }

    let out = Glif {
        outline: Some(output_outline),
        order: path.order,
        anchors: path.anchors.clone(),
        width: path.width,
        unicode: path.unicode,
        name: path.name,
        format: 2,
        lib: None,
    };
    let glifstring = glifparser::write_ufo_glif(&out);
    fs::write(output_file, glifstring).expect("Unable to write file");
}
