extern crate png;

use std::fs::File;

mod info;

enum BitDepth {
    Eight,
    Sixteen,
}



struct RgbImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    bit_depth: BitDepth,
}

impl RgbImage {
    pub fn from_file(path: &str) -> Result<RgbImage, &str> {
        // Initialize decoder with provided file.
        let decoder = png::Decoder::new(File::open(path).map_err(|_| info::error::INVALID_PNG)?);

        // Start decoding and get info from image.
        let (info, mut reader) = decoder.read_info().map_err(|_| info::error::DECODING_PNG)?;

        // Check if valid Color Type (RGB only).
        // TODO (TofuLynx): Evaluate wether or not to extend support to RGBA Color Type.
        if info.color_type != png::ColorType::RGB {
            return Err(info::error::INVALID_COLOR_TYPE);
        }

        // Initialize vector to store image data.
        let mut data = vec![0; info.buffer_size()];

        // Check if valid Bit Depth (8 or 16 only) and matches it.
        let bit_depth = match info.bit_depth {
            png::BitDepth::Eight => BitDepth::Eight,
            png::BitDepth::Sixteen => BitDepth::Sixteen,
            _ => {
                return Err(info::error::INVALID_BIT_DEPTH);
            }
        };

        // Decode frame.
        reader
            .next_frame(&mut data)
            .map_err(|_| info::error::CONVERTING_PNG)?;

        Ok(RgbImage {
            width: info.width,
            height: info.height,
            data: data,
            bit_depth: bit_depth,
        })
    }

    fn size(&self) -> u32 {
        self.width * self.height
    }

    pub fn to_raw(self, bayer_pattern: BayerPattern) -> RawImage {
        // TODO (TofuLynx): RgbImage.to_raw
        let width = self.width as usize;
        let height = self.height as usize;
        let color_offsets = bayer_pattern.color_offsets();

        let mut raw_data: Vec<u16> = vec![0; self.size() as usize];
        let mut raw_index;

        for row in (0..height).step_by(2) {
            for column in (0..width).step_by(2) {
                // Top Left.
                raw_index = row * width + column;
                raw_data[raw_index] = (self.data[(raw_index * 3 + color_offsets[0])] as u16) << 0;

                // Top Right.
                raw_index += 1;
                raw_data[raw_index] = (self.data[(raw_index * 3 + color_offsets[1])] as u16) << 0;

                // Bottom Right.
                raw_index += width;
                raw_data[raw_index] = (self.data[(raw_index * 3 + color_offsets[3])] as u16) << 0;

                // Bottom Left.
                raw_index -= 1;
                raw_data[raw_index] = (self.data[(raw_index * 3 + color_offsets[2])] as u16) << 0;
            }
        }

        RawImage {
            width: self.width,
            height: self.height,
            data: raw_data,
            bayer_pattern: bayer_pattern,
        }
    }
}

enum BayerPattern {
    RGGB,
    BGGR,
    GRBG,
    GBRG,
}

impl BayerPattern {
    pub fn from_str(bayer_pattern: &str) -> Result<BayerPattern, &str> {
        match bayer_pattern.to_uppercase().trim() {
            "RGGB" => Ok(BayerPattern::RGGB),
            "BGGR" => Ok(BayerPattern::BGGR),
            "GRBG" => Ok(BayerPattern::GRBG),
            "GBRG" => Ok(BayerPattern::GBRG),
            _ => Err(info::error::INVALID_PATTERN),
        }
    }

    pub fn color_offsets(&self) -> Vec<usize> {
        match self {
            BayerPattern::RGGB => vec![0, 1, 1, 2],
            BayerPattern::BGGR => vec![2, 1, 1, 0],
            BayerPattern::GRBG => vec![1, 0, 2, 1],
            BayerPattern::GBRG => vec![1, 2, 0, 1],
        }
    }
}

struct RawImage {
    width: u32,
    height: u32,
    data: Vec<u16>,
    bayer_pattern: BayerPattern,
}

impl RawImage {
    pub fn save_as_dng() {
        // TODO (TofuLynx): RawImage.to_dng
    }
}

fn main() {
    let rgb_image = match RgbImage::from_file("sample.png") {
        Ok(rgb_image) => rgb_image,
        Err(err) => {
            eprintln!("Error: {}", err);
            std::process::exit(1);
        }
    };

    let raw_image = rgb_image.to_raw(BayerPattern::RGGB);
    

    println!("Hello, world!");
}
