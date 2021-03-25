mod fontforge;

use std::fs;

use clap::{App, Arg};

pub fn clap_app() -> clap::App<'static, 'static>
{
    App::new("NIB")
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
}

pub fn nib_cli(matches: &clap::ArgMatches)
{
    let nib_file = matches.value_of("nib").unwrap();
    let input_file = matches.value_of("input").unwrap();
    let output_file = matches.value_of("output").unwrap();

    let settings = fontforge::NibSettings {
        nib: nib_file.to_string(),
        path: input_file.to_string(),
        quiet: true
    };

    let converted = fontforge::convert_glif(&settings);
    match converted {
        Some(glifstring) => fs::write(output_file, glifstring).expect("Unable to write file"),
        None => eprintln!("Failed to nib stroke")
    }
}
