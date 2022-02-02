use std::fs;

use clap::{App, Arg};
use MFEKmath::{variable_width_stroke_glif, VWSSettings};

pub fn clap_app() -> clap::App<'static> {
    App::new("VWS")
        .alias("variable")
        .alias("vws")
        .about("Takes a .glif file and strokes it with variable width.")
        .version("0.1")
        .author("Matthew Blanchard <matthewrblanchard@gmail.com>")
        .arg(
            Arg::new("input")
                .short('i')
                .takes_value(true)
                .help("The path to the input file.")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .takes_value(true)
                .help("The path where the output will be saved.")
                .required(true),
        )
}

pub fn vws_cli(matches: &clap::ArgMatches) {
    let input_string = matches.value_of("input").unwrap();
    let output_string = matches.value_of("output").unwrap();

    let input: glifparser::Glif<()> = glifparser::read(&fs::read_to_string(input_string).expect("Failed to read path file!")).unwrap(); // TODO: Proper error handling!

    // TODO: Copy logic from CWS here
    let settings = VWSSettings::<()> {
        cap_custom_end: None,
        cap_custom_start: None,
    };

    let out = variable_width_stroke_glif(&input, settings);
    let glifstring = glifparser::write(&out).unwrap(); // TODO: Proper error handling!
    fs::write(output_string, glifstring).expect("Unable to write file");
}
