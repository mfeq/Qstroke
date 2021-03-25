use std::fs;

use MFEKmath::piecewise::glif::PointData;
use MFEKmath::VWSHandle;
use MFEKmath::variable_width_stroke;
use MFEKmath::variable_width_stroking::*;

use clap::{App, Arg};

pub fn clap_app() -> clap::App<'static, 'static>
{
    App::new("VWS")
        .alias("variable")
        .alias("vws")
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

}

pub fn vws_cli(matches: &clap::ArgMatches)
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
