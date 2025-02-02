use std::fs;

use clap::{App, Arg};
use MFEKmath;

pub fn clap_app() -> clap::App<'static> {
    App::new("NIB")
        .alias("nib")
        .about("Takes a nib and a path, both in .glif format, and emulates a pen, with the chosen nib, stroking the path.\n\nImportant note: FontForge is used for this, so it may be more unstable than other modes as FontForge is implemented in C and not memory safe. To prevent bugs, we turn off simplification and overlap removal. Use MFEK for that.")
        .version("0.1.0")
        .author("Fredrick R. Brennan <copypasteⒶkittens⊙ph>; Skef Iterum (FontForge C code)")
        .arg(Arg::new("nib")
            .display_order(1)
            .short('n')
            .long("nib")
            .takes_value(true)
            .allow_invalid_utf8(true) // OS may use non-utf8 encoding for paths
            .help("The path to the nib file. FontForge is quite strict about these. The .glif must contain a single closed spline, running clockwise, which represents a convex shape.")
            .required(true))
        .arg(Arg::new("input")
            .display_order(2)
            .short('i')
            .long("input")
            .takes_value(true)
            .allow_invalid_utf8(true) // OS may use non-utf8 encoding for paths
            .help("The path to the input path file.")
            .required(true))
       .arg(Arg::new("output")
            .display_order(3)
            .short('o')
            .long("output")
            .takes_value(true)
            .allow_invalid_utf8(true) // OS may use non-utf8 encoding for paths
            .help("The path where the output .glif will be saved.")
            .required(true))
       .arg(Arg::new("accuracy")
            .display_order(4)
            .short('a')
            .long("accuracy")
            .takes_value(true)
            .default_value("0.25")
            .help("<f64> Accuracy target")
            .validator(super::arg_validator_positive_f64)
            .required(false))
}

pub fn nib_cli(matches: &clap::ArgMatches) {
    let nib_file = matches.value_of_os("nib").unwrap();
    let input_file = matches.value_of_os("input").unwrap();
    let output_file = matches.value_of_os("output").unwrap();
    let accuracy = matches.value_of("accuracy").unwrap();

    let settings = MFEKmath::nib_stroking::NibSettings {
        nib: nib_file.into(),
        path: input_file.into(),
        accuracy: accuracy.parse().unwrap(), //validated by super::arg_validator_positive_f64
        quiet: true,
    };

    let converted = MFEKmath::nib_stroking::convert_glif(&settings);
    match converted {
        Some(glifstring) => fs::write(&output_file, glifstring).expect("Unable to write file"),
        None => eprintln!("Failed to nib stroke"),
    }
}
