#![feature(clamp, fn_traits)]
#![allow(non_snake_case)] // for our name MFEKstroke

use MFEKmath;
use clap::{Arg, App, AppSettings};
use glifparser::{Glif, Outline};
use MFEKmath::Piecewise;
use MFEKmath::vector::Vector;
use MFEKmath::piecewise::glif::PointData;
use std::fs;

mod pattern_along_path;
mod variable_width_stroke;
mod constant_width_stroke;
#[cfg(feature="fontforge")]
mod nib_stroke;

fn main() {
    #[allow(unused_mut)] // we actually use it if cfg(feature=fontforge)
    let mut argparser = App::new("MFEKstroke")
        .setting(AppSettings::ArgRequiredElseHelp)
        .version("0.1.0")
        .author("Matthew Blanchard <matthewrblanchard@gmail.com>; Fredrick R. Brennan <copypasteⒶkittens⊙ph>; MFEK Authors")
        .about("A utility for applying stroking techniques to UFO contours.")
        .subcommand(pattern_along_path::clap_app())
        .subcommand(variable_width_stroke::clap_app())
        .subcommand(constant_width_stroke::clap_app());

    #[cfg(feature="fontforge")]
    {
        argparser = argparser.subcommand(nib_stroke::clap_app());
    }

    let matches = argparser.get_matches();

    match matches.subcommand_name() {
        Some("PAP") => pattern_along_path::pap_cli(&matches.subcommand_matches("PAP").unwrap()),
        Some("VWS") => variable_width_stroke::vws_cli(&matches.subcommand_matches("VWS").unwrap()),
        Some("CWS") => constant_width_stroke::cws_cli(&matches.subcommand_matches("CWS").unwrap()),
        #[cfg(feature="fontforge")]
        Some("NIB") => nib_stroke::nib_cli(&matches.subcommand_matches("NIB").unwrap()),
        _ => {}
    }
}
