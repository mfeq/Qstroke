use clap::{App, AppSettings, Arg, ArgMatches};
use glifparser::glif::{DashContour, DashCull};
use MFEKmath::skia_safe::{PaintCap, PaintJoin};

use std::fs;

pub fn clap_app() -> clap::App<'static> {
    App::new("DASH")
            .setting(AppSettings::DeriveDisplayOrder)
            .setting(AppSettings::AllowNegativeNumbers)
            .about("Applies a dash to a glyph.")
            .version("0.1.0")
            .author("Fredrick R. Brennan <copypasteⒶkittens.ph>; MFEK Authors; Skia/kurbo.rs authors")
            .arg(Arg::new("input")
                .long("input")
                .short('i')
                .takes_value(true)
                .required(true)
                .about("The path to the input glif file."))
            .arg(Arg::new("output")
                .long("output")
                .short('o')
                .required(true)
                .takes_value(true)
                .about("The path to the output glif file."))
            .arg(Arg::new("dash")
                .long("dash-description")
                .short('d')
                .multiple_values(true)
                .min_values(2)
                .validator(super::validators::arg_validator_positive_or_zero_f64)
                .validator_all(|vals| (vals.len() % 2 == 0).then(||Ok(())).unwrap_or_else(||Err("dash lengths not divisible by 2!")))
                .default_values(&["30", "30"])
                .about("Dash description"))
            .arg(Arg::new("cull")
                .short('c')
                .required(false)
                .long("cull")
                .about("Attempt to cull earlier dashes when later dashes cover them"))
            .arg(Arg::new("width")
                .short('w')
                .long("width")
                .takes_value(true)
                .validator(super::validators::arg_validator_positive_or_zero_f64)
                .default_value("30")
                .about("Stroke width (to leave an open contour, use 0)"))
            .arg(Arg::new("cull-width")
                .short('W')
                .long("cull-width")
                .takes_value(true)
                .validator(super::validators::arg_validator_positive_or_zero_f64)
                .default_value("40")
                .requires("cull")
                .about("Cull width"))
            .arg(Arg::new("area")
                .short('a')
                .short_alias('C')
                .long("min-area")
                .aliases(&["area-cutoff", "cull-cutoff"])
                .takes_value(true)
                .required(false)
                .validator(super::validators::arg_validator_positive_or_zero_f64)
                .requires("cull")
                .about("Paths with either a height or width below this number are culled. Do not set if unsure."))
            .arg(Arg::new("write-last-path")
                .short('l')
                .long("write-last")
                .alias("write-last-path")
                .requires("cull")
                .about("Write last path\n\n\n"))
            .arg(Arg::new("join-type")
                .long("join")
                .alias("jointype")
                .short('j')
                .takes_value(true)
                .case_insensitive(true)
                .possible_values(&["round", "miter", "bevel"])
                .about("How to join discontinuous splines")
                .default_value("round"))
            .arg(Arg::new("cap-type")
                .long("cap")
                .alias("captype")
                .short('J')
                .short_alias('A')
                .takes_value(true)
                .case_insensitive(true)
                .possible_values(&["round", "butt", "square"])
                .about("How to cap splines")
                .default_value("round"))
}

pub fn dash_cli(matches: &ArgMatches) {
    let path_string = matches.value_of("input").unwrap(); // required options shouldn't panic
    let out_string = matches.value_of("output").unwrap(); // required options shouldn't panic
    let stroke_width = matches.value_of("width").unwrap().parse::<f32>().unwrap();
    let cull = matches.is_present("cull");
    let cull_width = matches
        .value_of("cull-width")
        .unwrap()
        .parse::<f32>()
        .unwrap();
    let cull_cutoff = matches
        .value_of("area")
        .map(|v| v.parse::<f32>().unwrap())
        .unwrap_or((stroke_width * stroke_width) / 2.);
    let dash_desc = matches
        .values_of("dash")
        .unwrap()
        .map(|s| s.parse::<f32>().unwrap())
        .collect::<Vec<_>>();
    let include_last_path = matches.is_present("write-last-path");
    let paint_join = match matches
        .value_of("join-type")
        .unwrap()
        .to_ascii_lowercase()
        .as_str()
    {
        "round" => PaintJoin::Round,
        "bevel" => PaintJoin::Bevel,
        "miter" => PaintJoin::Miter,
        _ => unreachable!(),
    };
    let paint_cap = match matches
        .value_of("cap-type")
        .unwrap()
        .to_ascii_lowercase()
        .as_str()
    {
        "round" => PaintCap::Round,
        "butt" => PaintCap::Butt,
        "square" => PaintCap::Square,
        _ => unreachable!(),
    };

    let dash_settings = DashContour {
        stroke_width,
        dash_desc,
        include_last_path,
        paint_join: paint_join as u8,
        paint_cap: paint_cap as u8,
        cull: if cull {
            Some(DashCull {
                width: cull_width,
                area_cutoff: cull_cutoff,
            })
        } else {
            None
        },
    };

    // TODO: Handle errors properly!
    let mut path: glifparser::Glif<()> =
        glifparser::read(&fs::read_to_string(path_string).expect("Failed to read path file!"))
            .expect("glifparser couldn't parse input path glif. Invalid glif?");
    let out = MFEKmath::dash_along_glif(&path, &dash_settings);
    path = out;
    glifparser::write_to_filename(&path, out_string).unwrap();
}
