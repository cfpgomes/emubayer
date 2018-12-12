// Author: Cláudio Gomes (TofuLynx)
// Project: emubayer
// License: GNU GPL Version 3 (https://www.gnu.org/licenses/gpl-3.0.en.html)

extern crate emubayer;
extern crate clap;

use emubayer::*;
use clap::{Arg, App};

fn main() {
    let matches = App::new("emubayer")
                            .version("0.1")
                            .author("Cláudio Gomes (TofuLynx) <cfpgcp3@gmail.com>")
                            .about("Bayer CFA camera emulator that takes a \"picture\" of a provided PNG image and saves the result as a DNG file.")
                            .arg(Arg::with_name("INPUT_FILE")
                                .help("Sets the input PNG file to use")
                                .long_help("Sets the input PNG file to use. It must be a RGB image.")
                                .required(true)
                                .index(1)
                                )
                            .arg(Arg::with_name("BAYERPATTERN")
                                .help("Sets the Bayer Pattern to use")
                                .long_help("Sets the Bayter Pattern to use. Digital image sensors use a Color Filter Array with a specific pattern, usually called Bayer Filter Mosaic, which follows a pattern that is called Bayer Pattern here. There are 4 possible patterns: RGGB, BGGR, GRBG and GBRG; where R means Red, G means Green and B means Blue.")
                                .required(true)
                                .takes_value(true)
                                .possible_values(&["RGGB", "BGGR", "GRBG", "GBRG"])
                                .case_insensitive(true)
                                .index(2)
                                )
                            .arg(Arg::with_name("OUTPUT_FILE")
                                .help("Sets the filename of the output file.")
                                .long_help("Sets the filename of the output file. Emubayer automatically appends a .dng extension accordingly. If not specified, the output filename will be the same as the input file.")
                                .takes_value(true)
                                .index(3)
                                )
                            .get_matches();
    
    let input_path = matches.value_of("INPUT_FILE").unwrap();

    let output_path = matches.value_of("OUTPUT_FILE")
        .unwrap_or_else(|| matches.value_of("INPUT_FILE").unwrap())
        .trim_end_matches(".png")
        .trim_end_matches(".dng")
        .to_string() + ".dng";

    let bayer_pattern = BayerPattern::from_str(
        matches.value_of("BAYERPATTERN").unwrap()
    );

    println!("Using input file: {}", input_path);
    println!("Using Bayer Pattern: {}", bayer_pattern);

    let rgb_image = match RgbImage::from_file(input_path) {
        Ok(rgb_image) => rgb_image,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };

    let raw_image = rgb_image.to_raw(bayer_pattern);
    raw_image.save_as_dng(&output_path);

    println!("DNG file successfully saved as \"{}\".", output_path);
}
