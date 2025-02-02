use std::fs;

use float_cmp::ApproxEq as _;
use float_cmp::F64Margin;
use MFEKmath::pattern_along_glif;
use glifparser::glif::contour_operations::pap::{PatternCopies, PatternStretch, PatternSubdivide};
use MFEKmath::vec2;
use MFEKmath::vector::Vector;
use MFEKmath::EvalScale;
use MFEKmath::Piecewise;
use MFEKmath::PatternSettings;

use clap::{App, AppSettings, Arg};

pub fn clap_app() -> clap::App<'static> {
    App::new("PAP")
            .setting(AppSettings::DeriveDisplayOrder)
            .setting(AppSettings::AllowNegativeNumbers)
            .alias("patterned")
            .alias("pap")
            .alias("PaP")
            .about("Maps a pattern glyph along a path glyph.")
            .version("0.2.1")
            .author("Matthew Blanchard <matthewrblanchard@gmail.com>; Fredrick R. Brennan <copypasteⒶkittens.ph>; MFEK Authors")
            .arg(Arg::new("pattern")
                .long("pattern")
                .short('p')
                .takes_value(true)
                //.allow_invalid_utf8(true)
                .required_unless_present_any(&["dot-pattern", "dash-pattern"])
                .conflicts_with_all(&["dot-pattern", "dash-pattern"])
                .help("The path to the input pattern file. You may also provide either --dot-pattern or --dash-pattern to use built-in patterns."))
            .arg(Arg::new("dash-pattern")
                .long("dash-pattern")
                .short('=')
                .conflicts_with("dot-pattern")
                .help("Use a simple dash pattern"))
            .arg(Arg::new("warp")
                .long("warp")
                .short('w')
                .help("Warp the pattern to fit the path."))
            .arg(Arg::new("dot-pattern")
                .long("dot-pattern")
                .short('.')
                .conflicts_with("dash-pattern")
                .help("Use a simple dot pattern"))
            .arg(Arg::new("path")
                .long("path")
                .short('P')
                .takes_value(true)
                //.allow_invalid_utf8(true)
                .help("The path to the input path file.")
                .required(true))
            .arg(Arg::new("output")
                .long("output")
                .alias("out")
                .short('o')
                .takes_value(true)
                //.allow_invalid_utf8(true)
                .help("The path where the output will be saved. If omitted, or `-`, stdout.\n\n\n"))
            .arg(Arg::new("contour")
                .long("contour")
                .short('c')
                .takes_value(true)
                .validator(super::arg_validator_isize)
                .default_value("-1")
                .help("<isize> if this is a positive number we stroke only that specific contour in the outline by index."))
            .arg(Arg::new("mode")
                .short('m')
                .long("mode")
                .takes_value(true)
                .default_value("single")
                .possible_values(&["single", "repeated"])
                .help("Repeat mode."))
            .arg(Arg::new("subdivide")
                .short('s')
                .long("subdivide")
                .takes_value(true)
                .default_value("0")
                .hide_default_value(true)
                .validator(super::arg_validator_usize)
                .help("<usize> how many times to subdivide the patterns at their midpoint. [default: 0]\n\n\n"))
            .arg(Arg::new("subdivide_angle")
                .short('°')
                .long("subdivide-angle")
                .takes_value(true)
                .default_value("1")
                .hide_default_value(true)
                .validator(super::arg_validator_positive_f64)
                .help("<f64> how many degrees of change in direction to subdivide the patterns at. [default: 0]\n\n\n"))
            .arg(Arg::new("sx")
                .long("sx")
                .short('X')
                .takes_value(true)
                .default_value("1")
                .validator(super::arg_validator_positive_f64)
                .help("<f64> how much we scale our input pattern on the x-axis."))
            .arg(Arg::new("sy")
                .long("sy")
                .short('Y')
                .takes_value(true)
                .default_value("1")
                .validator(super::arg_validator_positive_f64)
                .help("<f64> how much we scale our input pattern on the y-axis."))
            .arg(Arg::new("split_at_discontinuity")
                .short('|')
                .long("split-at-discontinuity")
                .help("Handle discontinuities by splitting the path.")
            )
            .arg(Arg::new("normal-offset")
                .long("noffset")
                .short('n')
                .takes_value(true)
                .default_value("0")
                .validator(super::arg_validator_f64)
                .help("<f64> how much to offset the pattern along the normal of the path."))
            .arg(Arg::new("tangent-offset")
                .long("toffset")
                .short('t')
                .takes_value(true)
                .default_value("0")
                .hide_default_value(true)
                .validator(super::arg_validator_f64)
                .help("<f64> how much to offset the pattern along the tangent of the path. [default: 0]\n\n\n"))
            .arg(Arg::new("spacing")
                .long("spacing")
                .short('W')
                .takes_value(true)
                .default_value("0")
                .validator(super::arg_validator_positive_or_zero_f64)
                .help("<f64> how much padding to trail each copy with."))
            .arg(Arg::new("stretch")
                .long("stretch")
                .short('!')
                .takes_value(true)
                .possible_values(&["spacing"])
                .help("<stretch> false if not given, true if given, spacing mode if value of spacing given"))
            .arg(Arg::new("simplify")
                .short('S')
                .long("simplify")
                .help("<boolean> if we should run the result through Skia's (buggy) simplify routine."))
            .arg(Arg::new("remove_overlapping")
                .long("remove-overlapping")
                .short('O')
                .conflicts_with_all(&["erase_overlapping_stroke_width", "erase_overlapping", "erase_overlapping_area_percent"])
                .help("Remove patterns that would overlap."))
            .arg(Arg::new("erase_overlapping")
                .long("erase-overlapping")
                .short('Z')
                .help("Erase the area underneath patterns that would overlap."))   
            .arg(Arg::new("erase_overlapping_stroke_width")
                .long("erase-overlapping-stroke")
                .short('z')
                .takes_value(true)
                .default_value("5")
                .hide_default_value(true)
                .validator(super::arg_validator_f64) 
                .help("<float> how much we should expand the pattern when erasing overlapping patterns."))   
            .arg(Arg::new("erase_overlapping_area_percent")
                .short('%')
                .long("erase-overlapping-area-percent")
                .takes_value(true)
                .default_value("5")
                .hide_default_value(true)
                .validator(super::arg_validator_f64) 
                .help("<float> how much we should expand the pattern when erasing overlapping patterns."))              
            .arg(Arg::new("one-pass")
                .long("one-pass")
                .short('Q')
                .help("<boolean> whether we should not reflow the path after culling during overdraw (faster but worse)."))
            .arg(Arg::new("no-center-pattern")
                .long("no-center-pattern")
                .short('C')
                .conflicts_with_all(&["sx", "sy"])
                .help("<boolean> supply if you wish to center the pattern"))
            .arg(Arg::new("reverse")
                .long("reverse")
                .short('r')
                .help("<boolean> true will reverse the path."))
            .arg(Arg::new("reverse-culling")
                .long("reverse-culling")
                .short('R')
                .help("<boolean> true will reverse the order we check for overlaps during overlap culling.\n\n\n"))
}

pub fn pap_cli(matches: &clap::ArgMatches) {
    let path_string = matches.value_of("path").unwrap(); // required options shouldn't panic
    let pattern_string = matches.value_of("pattern");
    let output_string = matches.value_of("output");

    // TODO: Handle errors properly!
    let path: glifparser::Glif<()> = glifparser::read(&fs::read_to_string(path_string).expect("Failed to read path file!"))
        .expect("glifparser couldn't parse input path glif. Invalid glif?");

    let pattern: glifparser::Glif<()> = match pattern_string {
        None => {
            if matches.is_present("dot-pattern") {
                let mut dot = glifparser::read(include_str!("../assets/dot.glif")).unwrap();
                let piece_pattern = Piecewise::from(dot.outline.as_ref().unwrap());
                let normalized_pattern = piece_pattern.scale(vec2!(1. / 20., 1. / 20.));

                dot.outline = Some(normalized_pattern.to_outline());
                dot
            } else if matches.is_present("dash-pattern") {
                let mut dash = glifparser::read(include_str!("../assets/dash.glif")).unwrap();

                let piece_pattern = Piecewise::from(dash.outline.as_ref().unwrap());
                let normalized_pattern = piece_pattern.scale(vec2!(1. / 20., 1. / 20.));

                dash.outline = Some(normalized_pattern.to_outline());
                dash
            } else {
                unreachable!()
            }
        }
        Some(pattern) => glifparser::read(&fs::read_to_string(pattern).expect("Failed to read pattern file!"))
            .expect("glifparser couldn't parse input pattern glif. Invalid glif?"),
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
        cull_overlap: glifparser::glif::contour_operations::pap::PatternCulling::Off,
        two_pass_culling: false,
        reverse_path: false,
        reverse_culling: false,
        split_path: false,
        warp_pattern: false,
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
        let n = sub_string.parse::<usize>().unwrap();
        settings.subdivide = match n {
            0 => PatternSubdivide::Off,
            _ => PatternSubdivide::Simple(n),
        };
    }

    if let Some(sub_angle_string) = matches.value_of("subdivide_angle") {
        let n = sub_angle_string.parse::<f64>().unwrap();
        if 0.0f64.approx_eq(n, F64Margin { ulps: 2, epsilon: 0.01 }) {
            settings.subdivide = PatternSubdivide::Off;
        } else {
            settings.subdivide = PatternSubdivide::Angle(n);
        }
    }

    if let Some(spacing_string) = matches.value_of("spacing") {
        settings.spacing = spacing_string.parse::<f64>().unwrap();
    }

    if let Some(normal_string) = matches.value_of("normal-offset") {
        settings.normal_offset = normal_string.parse::<f64>().unwrap();
    }

    if let Some(tangent_string) = matches.value_of("tangent-offset") {
        settings.tangent_offset = tangent_string.parse::<f64>().unwrap();
    }

    settings.warp_pattern = matches.is_present("warp");
    settings.center_pattern = !matches.is_present("no-center-pattern");
    settings.simplify = matches.is_present("simplify");

    if matches.value_of("remove_overlapping").is_some() {
        settings.cull_overlap = glifparser::glif::contour_operations::pap::PatternCulling::RemoveOverlapping;
    }

    if matches.is_present("erase_overlapping") {
        settings.cull_overlap = glifparser::glif::contour_operations::pap::PatternCulling::EraseOverlapping(
            matches
                .value_of("erase_overlapping_stroke_width")
                .unwrap_or("5")
                .parse::<f64>()
                .unwrap(),
            matches
                .value_of("erase_overlapping_area_percent")
                .unwrap_or("25")
                .parse::<f64>()
                .unwrap(),
        );
    }

    settings.split_path = matches.is_present("split_at_discontinuity");

    // We know the string must be "spacing" as that's the only .possible_value to clap::Arg
    if let Some(s) = matches.value_of("stretch") {
        debug_assert_eq!(s, "spacing");
        settings.stretch = PatternStretch::Spacing;
    } else if matches.is_present("stretch") {
        settings.stretch = PatternStretch::On;
    } else {
        settings.stretch = PatternStretch::Off;
    }

    let mut target_contour = None;
    if let Some(contour) = matches.value_of("contour") {
        let idx = contour.parse::<isize>().unwrap();

        match (idx, path.outline.as_ref().map(|o| o.len() as isize >= idx), idx == -1) {
            (n, Some(false), false) => target_contour = Some(n as usize),
            (_, _, true) => {} // -1 ⇒ do nothing, target_contour already None
            _ => eprintln!("Invalid contour argument. Falling back to default. (-1)"),
        }
    }

    settings.two_pass_culling = !matches.is_present("one-pass");
    settings.reverse_path = matches.is_present("reverse");
    settings.reverse_culling = matches.is_present("reverse-culling");

    let output = pattern_along_glif(&path, &pattern, &settings, target_contour);
    let glifstring = glifparser::write(&output).expect("glifparser failed to understand output of PaP?"); // TODO: Proper error handling.
    if let Some(output_file) = output_string {
        if output_file != "-" {
            // common stand-in for stdout on *nix
            fs::write(output_file, &glifstring).expect("Unable to write output file");
            return;
        }
    }

    print!("{}", glifstring);
}
