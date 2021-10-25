use std::fs;

use glifparser::glif::{MFEKPointData, PatternCopies, PatternStretch, PatternSubdivide};
use MFEKmath::pattern_along_path::pattern_along_glif;
use MFEKmath::pattern_along_path::*;
use MFEKmath::vec2;
use MFEKmath::vector::Vector;
use MFEKmath::EvalScale;
use MFEKmath::Piecewise;

use clap::{App, Arg};

pub fn clap_app() -> clap::App<'static, 'static> {
    App::new("PAP")
            .alias("patterned")
            .alias("pap")
            .alias("PaP")
            .about("Maps a pattern glyph along a path glyph.")
            .version("0.2")
            .author("Matthew Blanchard <matthewrblanchard@gmail.com>; Fredrick R. Brennan <copypasteⒶkittens.ph>; MFEK Authors")
            .arg(Arg::with_name("pattern")
                .display_order(1)
                .long("pattern")
                .short("p")
                .takes_value(true)
                .required_unless_one(&["dot-pattern", "dash-pattern"])
                .help("The path to the input pattern file. You may also provide either --dot-pattern or --dash-pattern to use built-in patterns."))
            .arg(Arg::with_name("path")
                .display_order(2)
                .long("path")
                .short("P")
                .takes_value(true)
                .help("The path to the input path file.")
                .required(true))
            .arg(Arg::with_name("output")
                .display_order(3)
                .long("output")
                .alias("out")
                .short("o")
                .takes_value(true)
                .help("The path where the output will be saved. If omitted, or `-`, stdout."))
            .arg(Arg::with_name("dash-pattern")
                .long("dash_pattern")
                .conflicts_with("dot-pattern")
                .hidden(true)
                .help("Use a standardized dash pattern"))
            .arg(Arg::with_name("dot-pattern")
                .long("dot_pattern")
                .conflicts_with("dash-pattern")
                .hidden(true)
                .help("Use a standardized dot pattern"))
            .arg(Arg::with_name("contour")
                .display_order(4)
                .long("contour")
                .short("c")
                .takes_value(true)
                .validator(super::arg_validator_isize)
                .default_value("-1")
                .help("<isize> if this is a positive number we stroke only that specific contour in the outline by index."))
            .arg(Arg::with_name("mode")
                .display_order(4)
                .short("m")
                .long("mode")
                .takes_value(true)
                .default_value("single")
                .possible_values(&["single", "repeated"])
                .help("Repeat mode."))
            .arg(Arg::with_name("subdivide")
                .display_order(4)
                .short("sub")
                .long("subdivide")
                .takes_value(true)
                .default_value("0")
                .validator(super::arg_validator_usize)
                .help("<usize> how many times to subdivide the patterns at their midpoint."))
            .arg(Arg::with_name("sx")
                .display_order(4)
                .long("sx")
                .short("X")
                .takes_value(true)
                .default_value("1")
                .validator(super::arg_validator_positive_f64)
                .help("<f64> how much we scale our input pattern on the x-axis."))
            .arg(Arg::with_name("sy")
                .display_order(4)
                .long("sy")
                .short("Y")
                .takes_value(true)
                .default_value("1")
                .validator(super::arg_validator_positive_f64)
                .help("<f64> how much we scale our input pattern on the y-axis."))
            .arg(Arg::with_name("normal_offset")
                .display_order(5)
                .long("noffset")
                .short("n")
                .takes_value(true)
                .default_value("0")
                .validator(super::arg_validator_positive_or_zero_f64)
                .help("<f64> how much to offset the pattern along the normal of the path."))
            .arg(Arg::with_name("tangent_offset")
                .display_order(5)
                .long("toffset")
                .short("t")
                .takes_value(true)
                .default_value("0")
                .validator(super::arg_validator_positive_or_zero_f64)
                .help("<f64> how much to offset the pattern along the tangent of the path."))
            .arg(Arg::with_name("spacing")
                .long("spacing")
                .short("0")
                .takes_value(true)
                .default_value("0")
                .validator(super::arg_validator_positive_or_zero_f64)
                .help("<f64> how much padding to trail each copy with."))
            .arg(Arg::with_name("stretch")
                .long("stretch")
                .short("!")
                .empty_values(true)
                .possible_values(&["spacing"])
                .help("<stretch> false if not given, true if given, spacing mode if value of spacing given"))
            .arg(Arg::with_name("simplify")
                .short("S")
                .long("simplify")
                .help("<boolean> if we should run the result through a simplify routine."))
            .arg(Arg::with_name("overdraw")
                .long("overdraw")
                .short("O")
                .takes_value(true)
                .default_value("0.15")
                .validator(super::arg_validator_positive_f64)
                .help("<f64> any patterns that overlap more than arg * 100 percent are removed."))
            .arg(Arg::with_name("one-pass")
                .long("onepass")
                .short("1")
                .takes_value(true)
                .empty_values(true)
                .help("<boolean> whether we should not reflow the path after culling during overdraw (faster but worse)."))
            .arg(Arg::with_name("no-center-pattern")
                .long("no_center_pattern")
                .short("C")
                .help("<boolean> supply if you wish to center the pattern"))
            .arg(Arg::with_name("reverse")
                .long("reverse")
                .short("r")
                .help("<boolean> true will reverse the path."))
            .arg(Arg::with_name("reverse-culling")
                .long("reverse_culling")
                .short("R")
                .help("<boolean> true will reverse the order we check for overlaps during overlap culling."))
}

pub fn pap_cli(matches: &clap::ArgMatches) {
    let path_string = matches.value_of("path").unwrap(); // required options shouldn't panic
    let pattern_string = matches.value_of("pattern");
    let output_string = matches.value_of("output");

    // TODO: Handle errors properly!
    let path: glifparser::Glif<MFEKPointData> =
        glifparser::read(&fs::read_to_string(path_string).expect("Failed to read path file!"))
            .unwrap();

    let pattern: glifparser::Glif<MFEKPointData> = match pattern_string {
        None => {
            if matches.is_present("dot-pattern") {
                let mut dot = glifparser::read(include_str!("dot.glif")).unwrap();
                let piece_pattern = Piecewise::from(dot.outline.as_ref().unwrap());
                let normalized_pattern = piece_pattern.scale(vec2!(1. / 20., 1. / 20.));

                dot.outline = Some(normalized_pattern.to_outline());
                dot
            } else if matches.is_present("dash-pattern") {
                let mut dash = glifparser::read(include_str!("dash.glif")).unwrap();

                let piece_pattern = Piecewise::from(dash.outline.as_ref().unwrap());
                let normalized_pattern = piece_pattern.scale(vec2!(1. / 20., 1. / 20.));

                dash.outline = Some(normalized_pattern.to_outline());
                dash
            } else {
                unreachable!()
            }
        }
        Some(pattern) => {
            glifparser::read(&fs::read_to_string(pattern).expect("Failed to read pattern file!"))
                .unwrap()
        }
    };

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
        reverse_path: false,
        reverse_culling: false,
    };

    if let Some(copies) = matches.value_of("mode") {
        match copies {
            "single" => settings.copies = PatternCopies::Single,
            "repeated" => settings.copies = PatternCopies::Repeated,
            _ => eprintln!("Invalid mode argument. Falling back to default. (Single)"),
        }
    }

    if let Some(sx_string) = matches.value_of("sx") {
        settings.pattern_scale.x = sx_string.parse::<f64>().unwrap();
    }

    if let Some(sy_string) = matches.value_of("sy") {
        settings.pattern_scale.y = sy_string.parse::<f64>().unwrap();
    }

    if let Some(sub_string) = matches.value_of("subdivide") {
        settings.subdivide = PatternSubdivide::Simple(sub_string.parse::<usize>().unwrap());
    }

    if let Some(spacing_string) = matches.value_of("spacing") {
        settings.spacing = spacing_string.parse::<f64>().unwrap();
    }

    if let Some(normal_string) = matches.value_of("normal_offset") {
        settings.normal_offset = normal_string.parse::<f64>().unwrap();
    }

    if let Some(tangent_string) = matches.value_of("tangent_offset") {
        settings.tangent_offset = tangent_string.parse::<f64>().unwrap();
    }

    settings.center_pattern = matches.is_present("center_pattern");
    settings.simplify = matches.is_present("simplify");

    // We know the string must be "spacing" as that's the only .possible_value to clap::Arg
    if let Some(_) = matches.value_of("stretch") {
        settings.stretch = PatternStretch::Spacing;
    } else if matches.is_present("stretch") {
        settings.stretch = PatternStretch::On;
    } else {
        settings.stretch = PatternStretch::Off;
    }

    settings.two_pass_culling = !matches.is_present("one-pass");

    if let Some(overdraw_string) = matches.value_of("overdraw") {
        settings.cull_overlap = overdraw_string.parse::<f64>().unwrap();
    }

    let mut target_contour = None;
    if let Some(contour) = matches.value_of("contour") {
        let idx = contour.parse::<isize>().unwrap();

        match (
            idx,
            path.outline.as_ref().map(|o| o.len() as isize >= idx),
            idx == -1,
        ) {
            (n, Some(false), false) => target_contour = Some(n as usize),
            (_, _, true) => {} // -1 ⇒ do nothing, target_contour already None
            _ => eprintln!("Invalid contour argument. Falling back to default. (-1)"),
        }
    }

    settings.two_pass_culling = !matches.is_present("one-pass");
    settings.reverse_path = matches.is_present("reverse");
    settings.reverse_culling = matches.is_present("reverse-culling");

    let output = pattern_along_glif(&path, &pattern, &settings, target_contour);
    let glifstring = glifparser::write(&output).unwrap(); // TODO: Proper error handling.
    if let Some(output_file) = output_string {
        if output_file != "-" {
            // common stand-in for stdout on *nix
            fs::write(output_file, &glifstring).expect("Unable to write file");
            return;
        }
    }

    print!("{}", glifstring);
}
