#![feature(clamp)]
#![macro_use]

extern crate MFEKMath;
use clap::{Arg, App};
use MFEKMath::pattern_along_path::*;
use MFEKMath::vector::Vector;
use MFEKMath::variable_width_stroking::*;
use std::fs;

fn main() {
    let matches = App::new("MFEKStroke")
        .version("0.0.1")
        .author("Matthew Blanchard <matthewrblanchard@gmail.com")
        .about("A utility for applying stroking techniques to UFO contours.")
        .subcommand(App::new("PAP")
            .about("Maps a pattern glyph along a path glyph.")
            .version("0.1")
            .author("Matthew Blanchard <matthewrblanchard@gmail.com")
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
            .about("Takes a UFO file and strokes it with variable width.")
            .version("0.0")
            .author("Matthew Blanchard <matthewrblanchard@gmail.com")
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
            .arg(Arg::with_name("join_type")
                .short("jt")
                .long("join_type")
                .takes_value(true)
                .help("<bevel/miter/round> which type of join to use."))
            .arg(Arg::with_name("cap_type_start")
                .long("cts")
                .takes_value(true)
                .help("<round/square/path to custom cap> which type of cap to use for the starting cap."))
            .arg(Arg::with_name("cap_type_end")
                .long("noffset")
                .takes_value(true)
                .help("<round/square/path to custom cap> which type of cap to use for the ending cap."))
        )
        .get_matches();

    if let Some(ref matches) = matches.subcommand_matches("PAP") {
        pap_cli(matches);
    }
 
    if let Some(ref matches) = matches.subcommand_matches("VWS") {
        vws_cli(matches);
    }
    
}

fn vws_cli(matches: &clap::ArgMatches)
{
    let input_string = matches.value_of("input").unwrap();
    let output_string = matches.value_of("output").unwrap();

    let input: glifparser::Glif<Option<MFEKMath::piecewise::glif::PointData>> = glifparser::read_ufo_glif(&fs::read_to_string(input_string)
    .expect("Failed to read path file!"));

    let mut settings = VWSSettings {
        join_type: JoinType::Bevel,
        cap_type_start: CapType::Square,
        cap_type_end: CapType::Square,
        cap_custom_end: None,
        cap_custom_start: None
    };

    if let Some(joins) = matches.value_of("join_type") { 
        match joins {
            "bevel" => settings.join_type = JoinType::Bevel,
            "round" => settings.join_type = JoinType::Round,
            "miter" => settings.join_type = JoinType::Miter,
            _ => eprintln!("Invalid join type argument. Falling back to default. (Bevel)")
        }
    }

    if let Some(capstart) = matches.value_of("cap_type_start") { 
        match capstart {
            "round" => settings.cap_type_start = CapType::Round,
            "square" => settings.cap_type_start = CapType::Square,
            _ => {
                // if our input isn't round or square it should be a path to a glif
                let cap: glifparser::Glif<Option<MFEKMath::piecewise::glif::PointData>> = glifparser::read_ufo_glif(&fs::read_to_string(capstart)
                    .expect("Failed to read cap start file!"));
                
                settings.cap_type_start = CapType::Custom;
                settings.cap_custom_start = Some(cap);
            }
        }
    }

    if let Some(capend) = matches.value_of("cap_type_end") { 
        match capend {
            "round" => settings.cap_type_end = CapType::Round,
            "square" => settings.cap_type_end = CapType::Square,
            _ => {
                // if our input isn't round or square it should be a path to a glif
                let cap: glifparser::Glif<Option<MFEKMath::piecewise::glif::PointData>> = glifparser::read_ufo_glif(&fs::read_to_string(capend)
                    .expect("Failed to read cap start file!"));
                
                settings.cap_type_end = CapType::Custom;
                settings.cap_custom_end = Some(cap);
            }
        }
    }

    let out = variable_width_stroke_glif(&input, settings);
    let glifstring = glifparser::write_ufo_glif(&out);
    fs::write(output_string, glifstring).expect("Unable to write file");
}

fn pap_cli(matches: &clap::ArgMatches)
{
    let path_string = matches.value_of("path").unwrap(); // required options shouldn't panic?
    let pattern_string = matches.value_of("pattern").unwrap();
    let output_string = matches.value_of("output").unwrap();

    let path: glifparser::Glif<Option<MFEKMath::piecewise::glif::PointData>> = glifparser::read_ufo_glif(&fs::read_to_string(path_string)
    .expect("Failed to read path file!"));
 
    let pattern: glifparser::Glif<Option<MFEKMath::piecewise::glif::PointData>> = glifparser::read_ufo_glif(&fs::read_to_string(pattern_string)
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