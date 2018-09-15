// Author: Cláudio Gomes (TofuLynx)
// Project: emubayer
// License: GNU GPL Version 3 (https://www.gnu.org/licenses/gpl-3.0.en.html)

pub static INVALID_PATTERN: &str = "\
Could not parse the bayer pattern. Please enter one of the following:
    RGGB
    BGGR
    GRBG
    GBRG\
";

pub static INVALID_PNG: &str = "\
PNG image couldn't be opened.\
";

pub static DECODING_PNG: &str = "\
This PNG file appears to be corrupted.\
";

pub static INVALID_COLOR_TYPE: &str = "\
PNG image needs to be RGB Color Type.\
";

pub static INVALID_BIT_DEPTH: &str = "\
PNG image needs to have 8 or 16 Bit Depth.\
";

pub static CONVERTING_PNG: &str = "\
An error occurred interpreting this PNG image.\
";