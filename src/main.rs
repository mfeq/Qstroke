#![feature(clamp, fn_traits)]
#![allow(non_snake_case)] // for our name MFEKstroke

use MFEKmath;
use clap::{Arg, App};
use glifparser::{Glif, Outline};
use MFEKmath::Piecewise;
use MFEKmath::pattern_along_path::*;
use MFEKmath::vector::Vector;
use MFEKmath::variable_width_stroking::*;
use MFEKmath::piecewise::glif::PointData;
use std::fs;

#[cfg(feature = "fontforge")]
pub mod fontforge_nib;

fn main() {
    fn arg_validator_width(v: String) -> Result<(), String> {
        match v.parse::<f64>() {
            Ok(i) => {
                if i <= 0.0 + f64::EPSILON {
                    Err(String::from("Value too small"))
                } else {
                    Ok(())
                }
            },
            Err(_) => Err(String::from("Value must be a float"))
        }
    }

    #[allow(unused_mut)] // we actually use it if cfg(feature=fontforge)
    let mut argparser = App::new("MFEKstroke")
        .version("0.1.0")
        .author("Matthew Blanchard <matthewrblanchard@gmail.com>; Fredrick R. Brennan <copypasteⒶkittens⊙ph>")
        .about("A utility for applying stroking techniques to UFO contours.")
        .subcommand(App::new("PAP")
            .alias("patterned")
            .about("Maps a pattern glyph along a path glyph.")
            .version("0.1")
            .author("Matthew Blanchard <matthewrblanchard@gmail.com>")
            .arg(Arg::with_name("path")
                .long("path")
                .takes_value(true)
                .help("The path to the input path file.")
                .required(true))
            .arg(Arg::with_name("pattern")
                .long("pattern")
                .takes_value(true)
                .help("The path to the input pattern file.")
                .required(true))
            .arg(Arg::with_name("output")
                .long("out")
                .takes_value(true)
                .help("The path where the output will be saved.")
                .required(true))
            .arg(Arg::with_name("mode")
                .short("m")
                .long("mode")
                .takes_value(true)
                .help("<[single|repeated] (single)> set our repeat mode."))
            .arg(Arg::with_name("scale_x")
                .long("sx")
                .takes_value(true)
                .help("<f64 (1)> how much we scale our input pattern on the x-axis."))
            .arg(Arg::with_name("scale_y")
                .long("sy")
                .takes_value(true)
                .help("<f64 (1)> how much we scale our input pattern on the y-axis."))
            .arg(Arg::with_name("subdivide")
                .short("sub")
                .long("subdivide")
                .takes_value(true)
                .help("<f64 (0)> how many times to subdivide the patterns at their midpoint."))
            .arg(Arg::with_name("spacing")
                .long("spacing")
                .takes_value(true)
                .help("<f64 (0)> how much padding to trail each copy with."))
            .arg(Arg::with_name("normal_offset")
                .long("noffset")
                .takes_value(true)
                .help("<f64 (0)> how much to offset the pattern along the normal of the path."))
            .arg(Arg::with_name("tangent_offset")
                .long("toffset")
                .takes_value(true)
                .help("<f64 (0)> how much to offset the pattern along the tangent of the path."))
            .arg(Arg::with_name("stretch")
                .long("stretch")
                .takes_value(true)
                .help("<boolean (false)> whether to stretch the input pattern or not."))
            .arg(Arg::with_name("simplify")
                .long("simplify")
                .takes_value(true)
                .help("<boolean (false)> if we should run the result through a simplify routine."))
            .arg(Arg::with_name("center_pattern")
                .long("center_pattern")
                .takes_value(true)
                .help("<boolean (true)> if you want to align a pattern manually you can change this to false."))
        )
        .subcommand(App::new("VWS")
            .alias("variable")
            .about("Takes a .glif file and strokes it with variable width.")
            .version("0.0")
            .author("Matthew Blanchard <matthewrblanchard@gmail.com>")
            .arg(Arg::with_name("input")
                .short("in")
                .takes_value(true)
                .help("The path to the input file.")
                .required(true))
            .arg(Arg::with_name("output")
                .short("out")
                .takes_value(true)
                .help("The path where the output will be saved.")
                .required(true))
        )
        .subcommand(App::new("CWS")
            .alias("constant")
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
                .validator(arg_validator_width)
                .required(true))
        );

    #[cfg(feature="fontforge")]
    {
    argparser = argparser.subcommand(App::new("NIB")
         .alias("nib")
         .about("Takes a nib and a path, both in .glif format, and emulates a pen, with the chosen nib, stroking the path.\n\nImportant note: FontForge is used for this, so it may be more unstable than other modes as FontForge is implemented in C and not memory safe. To prevent bugs, we turn off simplification and overlap removal. Use MFEK for that.")
         .version("0.1.0")
         .author("Fredrick R. Brennan <copypasteⒶkittens⊙ph>; Skef Iterum (FontForge)")
         .arg(Arg::with_name("nib")
             .display_order(1)
             .short("nib")
             .long("nib")
             .takes_value(true)
             .help("The path to the nib file. FontForge is quite strict about these. The .glif must contain a single closed spline, running clockwise, which represents a convex shape.")
             .required(true))
         .arg(Arg::with_name("input")
             .display_order(2)
             .short("in")
             .long("input")
             .takes_value(true)
             .help("The path to the input path file.")
             .required(true))
        .arg(Arg::with_name("output")
             .display_order(3)
             .short("out")
             .long("output")
             .takes_value(true)
             .help("The path where the output .glif will be saved.")
             .required(true))
        );
    }

    let matches = argparser.get_matches();

    match matches.subcommand_name() {
        Some("PAP") => pap_cli(&matches.subcommand_matches("PAP").unwrap()),
        Some("VWS") => vws_cli(&matches.subcommand_matches("VWS").unwrap()),
        Some("CWS") => cws_cli(&matches.subcommand_matches("CWS").unwrap()),
        Some("NIB") => nib_cli(&matches.subcommand_matches("NIB").unwrap()),
        _ => {}
    }
}

fn nib_cli(matches: &clap::ArgMatches)
{
    let nib_file = matches.value_of("nib").unwrap();
    let input_file = matches.value_of("input").unwrap();
    let output_file = matches.value_of("output").unwrap();

    let settings = fontforge_nib::NibSettings {
        nib: nib_file.to_string(),
        path: input_file.to_string(),
        quiet: true
    };

    let converted = fontforge_nib::convert_glif(&settings);
    match converted {
        Some(glifstring) => fs::write(output_file, glifstring).expect("Unable to write file"),
        None => eprintln!("Failed to nib stroke")
    }
}

// Constant width stroking is really just a special case of variable width stroking. So, we take
// the width, divide by two to make handles from it, and use those to stroke at a tangent of 0.
//
// Some of this was copied from MFEK/math.rlib file src/variable_width_stroking.rs fn variable_width_stroke_glif
fn cws_cli(matches: &clap::ArgMatches)
{
    fn str_to_jointype(s: &str) -> JoinType {
        match s {
            "bevel" => JoinType::Bevel,
            "miter" => JoinType::Miter,
            "round" => JoinType::Round,
            _ => unimplemented!()
        }
    }

    fn str_to_cap(s: &str) -> CapType {
        match s {
            "round" => CapType::Round,
            "square" => CapType::Square,
            _ => CapType::Custom
        }
    }

    fn custom_cap_if_requested(ct: CapType, input_file: &str) -> Option<Glif<Option<PointData>>> {
        if ct == CapType::Custom {
            let path: glifparser::Glif<Option<PointData>> = glifparser::read_ufo_glif(&fs::read_to_string(input_file)
            .expect("Failed to read file!"));
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
    let path: glifparser::Glif<Option<PointData>> = glifparser::read_ufo_glif(&fs::read_to_string(input_file)
    .expect("Failed to read file!"));

    let vws_contour = VWSContour {
        id: 0,
        join_type: jointype,
        cap_start_type: startcap,
        cap_end_type: endcap,
        handles: vec![] // to be populated based on number of points
    };

    let mut settings = VWSSettings {
        cap_custom_end: custom_cap_if_requested(endcap, matches.value_of("endcap").unwrap()),
        cap_custom_start: custom_cap_if_requested(startcap, matches.value_of("startcap").unwrap())
    };

    // convert our path and pattern to piecewise collections of beziers
    let piece_path = Piecewise::from(path.outline.as_ref().unwrap());
    let mut output_outline: Outline<Option<PointData>> = Vec::new();

    let mut vws_contours = vec![vws_contour; path.outline.as_ref().unwrap().len()];

    let vws_handle = VWSHandle {
        left_offset: width / 2.0,
        right_offset: width / 2.0,
        tangent_offset: 0.0,
        interpolation: InterpolationType::Linear
    };

    for outline in path.outline.as_ref() {
        for (cidx, contour) in outline.iter().enumerate() {
            let pointiter = contour.iter().enumerate();

            for (pidx, point) in pointiter {
                vws_contours[cidx].handles.push(vws_handle);
            }
        }
    }

    let iter = piece_path.segs.iter().enumerate();
    for (i, pwpath_contour) in iter {
        let mut vws_contour = &vws_contours[i];

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
        name: path.name.clone(),
        format: 2,
        lib: None
    };
    let glifstring = glifparser::write_ufo_glif(&out);
    fs::write(output_file, glifstring).expect("Unable to write file");
}

fn vws_cli(matches: &clap::ArgMatches)
{
    let input_string = matches.value_of("input").unwrap();
    let output_string = matches.value_of("output").unwrap();

    let input: glifparser::Glif<Option<PointData>> = glifparser::read_ufo_glif(&fs::read_to_string(input_string)
    .expect("Failed to read path file!"));

    let mut settings = VWSSettings {
        cap_custom_end: None,
        cap_custom_start: None
    };

    let out = variable_width_stroke_glif(&input, settings);
    let glifstring = glifparser::write_ufo_glif(&out);
    fs::write(output_string, glifstring).expect("Unable to write file");
}

fn pap_cli(matches: &clap::ArgMatches)
{
    let path_string = matches.value_of("path").unwrap(); // required options shouldn't panic
    let pattern_string = matches.value_of("pattern").unwrap();
    let output_string = matches.value_of("output").unwrap();

    let path: glifparser::Glif<Option<PointData>> = glifparser::read_ufo_glif(&fs::read_to_string(path_string)
    .expect("Failed to read path file!"));

    let pattern: glifparser::Glif<Option<PointData>> = glifparser::read_ufo_glif(&fs::read_to_string(pattern_string)
        .expect("Failed to read pattern file!"));


    let mut settings = PatternSettings{
        copies: PatternCopies::Single,
        subdivide: PatternSubdivide::Off,
        is_vertical: false,
        normal_offset: 0.,
        tangent_offset: 0.,
        center_pattern: true,
        pattern_scale: Vector{x:1., y: 1.},
        spacing: 0.,
        stretch: false,
        simplify: false
    };

    if let Some(copies) = matches.value_of("mode") { 
        match copies {
            "single" => settings.copies = PatternCopies::Single,
            "repeated" => settings.copies = PatternCopies::Repeated,
            _ => eprintln!("Invalid mode argument. Falling back to default. (Single)")
        }
    }

    if let Some(sx_string) = matches.value_of("scale_x") {
        let sx = sx_string.parse::<f64>();

        match sx {
            Ok(n) => settings.pattern_scale.x = n,
            Err(_e) => eprintln!("Invalid scale x argument. Falling back to default. (1)")
        }
    }

    if let Some(sy_string) = matches.value_of("scale_y") {
        let sy = sy_string.parse::<f64>();

        match sy {
            Ok(n) => settings.pattern_scale.y = n,
            Err(_e) => eprintln!("Invalid scale y argument. Falling back to default. (1)")
        }
    }

    if let Some(sub_string) = matches.value_of("subdivide") {
        let subs = sub_string.parse::<usize>();

        match subs {
            Ok(n) => settings.subdivide = PatternSubdivide::Simple(n),
            Err(_e) => eprintln!("Invalid subdivision count. Falling back to default. (0)")
        }
    }

    if let Some(spacing_string) = matches.value_of("spacing") {
        let spacing = spacing_string.parse::<f64>();

        match spacing {
            Ok(n) => settings.spacing = n,
            Err(_e) => eprintln!("Invalid spacing. Falling back to default. (0)")
        }
    }

    if let Some(normal_string) = matches.value_of("normal_offset") {
        let spacing = normal_string.parse::<f64>();

        match spacing {
            Ok(n) => settings.normal_offset = n,
            Err(_e) => eprintln!("Invalid normal offset. Falling back to default. (0)")
        }
    }

    if let Some(tangent_string) = matches.value_of("tangent_offset") {
        let spacing = tangent_string.parse::<f64>();

        match spacing {
            Ok(n) => settings.tangent_offset = n,
            Err(_e) => eprintln!("Invalid tangent offset. Falling back to default. (0)")
        }
    }

    if let Some(center_string) = matches.value_of("center_pattern") {
        match center_string {
            "true" => settings.center_pattern = true,
            "false" => settings.center_pattern = false,
            _ => eprintln!("Invalid center pattern argument. Falling back to default. (true)")
        }
    }

    if let Some(simplify_string) = matches.value_of("simplify") {
        match simplify_string {
            "true" => settings.simplify = true,
            "false" => settings.simplify = false,
            _ => eprintln!("Invalid center pattern argument. Falling back to default. (true)")
        }
    }

    if let Some(stretch_string) = matches.value_of("stretch") {
        match stretch_string {
            "true" => settings.stretch = true,
            "false" => settings.stretch = false,
            _ => eprintln!("Invalid center pattern argument. Falling back to default. (true)")
        }
    }


    let output = pattern_along_glif(&path, &pattern, &settings);
    let glifstring = glifparser::write_ufo_glif(&output);
    fs::write(output_string, glifstring).expect("Unable to write file");
}
