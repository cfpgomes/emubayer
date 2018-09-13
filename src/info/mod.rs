// Author: Cláudio Gomes (TofuLynx)
// Project: emubayer
// License: GNU GPL Version 3 (https://www.gnu.org/licenses/gpl-3.0.en.html)

pub mod error;

pub static HELP: &str = "\
usage:
emubayer <png_path> <bayer_cfa_pattern>
    Creates a DNG image sample based on an emulated Bayer CFA Camera taking a picture at the given PNG image.\
";
