use std::fs;

use MFEKmath::pattern_along_path::pattern_along_glif;
use MFEKmath::pattern_along_path::*;
use MFEKmath::vector::Vector;
use glifparser::glif::{MFEKPointData, PatternCopies, PatternSubdivide, PatternStretch};

use clap::{App, Arg};

pub fn clap_app() -> clap::App<'static, 'static> {
    App::new("PAP")
            .alias("patterned")
            .alias("pap")
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
                .help("<mode (spacing)> possible inputs are on, off, and spacing."))
            .arg(Arg::with_name("simplify")
                .long("simplify")
                .takes_value(true)
                .help("<boolean (false)> if we should run the result through a simplify routine."))
            .arg(Arg::with_name("overdraw")
                .long("overdraw")
                .takes_value(true)
                .help("<f64 (0.15)> any patterns that overlap more than arg * 100 percent are removed."))
            .arg(Arg::with_name("two-pass culling")
                .long("twopass")
                .takes_value(true)
                .help("<boolean (true)> whether we should reflow the path after culling during overdraw."))
            .arg(Arg::with_name("center_pattern")
                .long("center_pattern")
                .takes_value(true)
                .help("<boolean (true)> if you want to align a pattern manually you can change this to false."))
            .arg(Arg::with_name("contour")
                .long("contour")
                .takes_value(true)
                .help("<usize (-1)> if this is a positive number we stroke only that specific contour in the outline by index."))
}

pub fn pap_cli(matches: &clap::ArgMatches) {
    let path_string = matches.value_of("path").unwrap(); // required options shouldn't panic
    let pattern_string = matches.value_of("pattern").unwrap();
    let output_string = matches.value_of("output").unwrap();

    // TODO: Handle errors properly!
    let path: glifparser::Glif<Option<MFEKPointData>> = glifparser::read(
        &fs::read_to_string(path_string).expect("Failed to read path file!"),
    ).unwrap();

    let pattern: glifparser::Glif<Option<MFEKPointData>> = glifparser::read(
        &fs::read_to_string(pattern_string).expect("Failed to read pattern file!"),
    ).unwrap();

    let mut settings = PatternSettings {
        copies: PatternCopies::Single,
        subdivide: PatternSubdivide::Off,
        is_vertical: false,
        normal_offset: 0.,
        tangent_offset: 0.,
        center_pattern: true,
        pattern_scale: Vector { x: 1., y: 1. },
        spacing: 0.,
        stretch: PatternStretch::Spacing,
        simplify: false,
        cull_overlap: 1.,
        two_pass_culling: false,
    };

    if let Some(copies) = matches.value_of("mode") {
        match copies {
            "single" => settings.copies = PatternCopies::Single,
            "repeated" => settings.copies = PatternCopies::Repeated,
            _ => eprintln!("Invalid mode argument. Falling back to default. (Single)"),
        }
    }

    if let Some(sx_string) = matches.value_of("scale_x") {
        let sx = sx_string.parse::<f64>();

        match sx {
            Ok(n) => settings.pattern_scale.x = n,
            Err(_e) => eprintln!("Invalid scale x argument. Falling back to default. (1)"),
        }
    }

    if let Some(sy_string) = matches.value_of("scale_y") {
        let sy = sy_string.parse::<f64>();

        match sy {
            Ok(n) => settings.pattern_scale.y = n,
            Err(_e) => eprintln!("Invalid scale y argument. Falling back to default. (1)"),
        }
    }

    if let Some(sub_string) = matches.value_of("subdivide") {
        let subs = sub_string.parse::<usize>();

        match subs {
            Ok(n) => settings.subdivide = PatternSubdivide::Simple(n),
            Err(_e) => eprintln!("Invalid subdivision count. Falling back to default. (0)"),
        }
    }

    if let Some(spacing_string) = matches.value_of("spacing") {
        let spacing = spacing_string.parse::<f64>();

        match spacing {
            Ok(n) => settings.spacing = n,
            Err(_e) => eprintln!("Invalid spacing. Falling back to default. (0)"),
        }
    }

    if let Some(normal_string) = matches.value_of("normal_offset") {
        let spacing = normal_string.parse::<f64>();

        match spacing {
            Ok(n) => settings.normal_offset = n,
            Err(_e) => eprintln!("Invalid normal offset. Falling back to default. (0)"),
        }
    }

    if let Some(tangent_string) = matches.value_of("tangent_offset") {
        let spacing = tangent_string.parse::<f64>();

        match spacing {
            Ok(n) => settings.tangent_offset = n,
            Err(_e) => eprintln!("Invalid tangent offset. Falling back to default. (0)"),
        }
    }

    if let Some(center_string) = matches.value_of("center_pattern") {
        match center_string {
            "true" => settings.center_pattern = true,
            "false" => settings.center_pattern = false,
            _ => eprintln!("Invalid center pattern argument. Falling back to default. (true)"),
        }
    }

    if let Some(simplify_string) = matches.value_of("simplify") {
        match simplify_string {
            "true" => settings.simplify = true,
            "false" => settings.simplify = false,
            _ => eprintln!("Invalid center pattern argument. Falling back to default. (true)"),
        }
    }

    if let Some(stretch_string) = matches.value_of("stretch") {
        match stretch_string {
            "on" => settings.stretch = PatternStretch::On,
            "off" => settings.stretch = PatternStretch::Off,
            "spacing" => settings.stretch = PatternStretch::Spacing,
            _ => eprintln!("Invalid stretch argument. Falling back to default. (true)"),
        }
    }

    if let Some(simplify_string) = matches.value_of("two-pass") {
        match simplify_string {
            "true" => settings.two_pass_culling = true,
            "false" => settings.two_pass_culling = false,
            _ => eprintln!("Invalid center pattern argument. Falling back to default. (true)"),
        }
    }

    if let Some(overdraw_string) = matches.value_of("overdraw") {
        let overdraw = overdraw_string.parse::<f64>();

        match overdraw {
            Ok(n) => settings.cull_overlap = n,
            Err(_e) => eprintln!("Invalid overdraw percentage. Falling back to default. (0)"),
        }
    }

    let mut target_contour = None;
    if let Some(contour) = matches.value_of("contour") {
        let idx = contour.parse::<usize>();

        match idx {
            Ok(n) => if n >= 0 {target_contour = Some(n)},
            Err(_e) => eprintln!("Invalid contour argument. Falling back to default. (1)"),
        }
    }


    let output = pattern_along_glif(&path, &pattern, &settings, target_contour);
    let glifstring = glifparser::write(&output).unwrap(); // TODO: Proper error handling.
    fs::write(output_string, glifstring).expect("Unable to write file");
}
