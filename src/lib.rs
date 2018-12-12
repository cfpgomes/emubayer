// Author: Cláudio Gomes (TofuLynx)
// Project: emubayer
// License: GNU GPL Version 3 (https://www.gnu.org/licenses/gpl-3.0.en.html)

extern crate png;
extern crate tiff_encoder;
extern crate byteorder;

use std::{fs::File, fmt};

use tiff_encoder::{*, tiff_type::*};
use byteorder::{WriteBytesExt, LittleEndian};

#[cfg(test)]
mod tests;

enum BitDepth {
    Eight,
    Sixteen,
}

enum ColorType {
    RGB,
    RGBA,
}

pub struct RgbImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    color_type: ColorType,
    bit_depth: BitDepth,
}

impl RgbImage {
    pub fn from_file(path: &str) -> Result<RgbImage, &str> {
        let png_file = File::open(path)
            .map_err(|_| "PNG image couldn't be opened.")?;

        let decoder = png::Decoder::new(png_file);
        let (info, mut reader) = decoder.read_info()
            .map_err(|_| "This PNG file appears to be corrupted.")?;

        let color_type = match info.color_type {
            png::ColorType::RGB => ColorType::RGB,
            png::ColorType::RGBA => ColorType::RGBA,
            _ => return Err("PNG image needs to be RGB or RGBA Color Type."),
        };

        let bit_depth = match info.bit_depth {
            png::BitDepth::Eight => BitDepth::Eight,
            png::BitDepth::Sixteen => BitDepth::Sixteen,
            _ => return Err("PNG image needs to have 8 or 16 Bit Depth."),
        };

        // Decode frame.
        let mut data = vec![0; info.buffer_size()];
        reader.next_frame(&mut data)
            .map_err(|_| "An error occurred interpreting this PNG image.")?;

        Ok(RgbImage {
            width: info.width,
            height: info.height,
            color_type: color_type,
            data: data,
            bit_depth: bit_depth,
        })
    }

    fn even_width(&self) -> u32 {
        if self.width % 2 == 0 {
            self.width
        } else {
            self.width - 1
        }
    }

    fn even_height(&self) -> u32 {
        if self.height % 2 == 0 {
            self.height
        } else {
            self.height - 1
        }
    }

    fn even_size(&self) -> u32 {
        self.even_width() * self.even_height()
    }

    pub fn to_raw(self, bayer_pattern: BayerPattern) -> RawImage {
        let width = self.width as usize;
        let is_even = width % 2 == 0;
        let color_offsets = bayer_pattern.color_offsets();

        let mut raw_data: Vec<u16> = vec![0; self.even_size() as usize];
        let mut raw_index;

        let multiplier = match self.color_type {
            ColorType::RGB => 3,
            ColorType::RGBA => 4,
        } as usize;

        for row in (0..self.even_height()).step_by(2) {
            for column in (0..self.even_width()).step_by(2) {
                let odd_offset = if is_even { 0 } else { row } as usize;

                // Top Left.
                raw_index = (row * self.even_width() + column) as usize;                
                raw_data[raw_index] = (self.data[((raw_index + odd_offset) * multiplier + color_offsets[0] as usize)] as u16) << 8;

                // Top Right.
                raw_index += 1;
                raw_data[raw_index] = (self.data[((raw_index + odd_offset) * multiplier + color_offsets[1] as usize)] as u16) << 8;

                // Bottom Right.
                raw_index += self.even_width() as usize;
                raw_data[raw_index] = (self.data[((raw_index + odd_offset) * multiplier + color_offsets[3] as usize)] as u16) << 8;

                // Bottom Left.
                raw_index -= 1;
                raw_data[raw_index] = (self.data[((raw_index + odd_offset) * multiplier + color_offsets[2] as usize)] as u16) << 8;
            }
        }

        RawImage {
            width: self.even_width(),
            height: self.even_height(),
            data: raw_data,
            bayer_pattern: bayer_pattern,
        }
    }
}

pub enum BayerPattern {
    RGGB,
    BGGR,
    GRBG,
    GBRG,
}
impl fmt::Display for BayerPattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",
            match self {
                BayerPattern::RGGB => "RGGB",
                BayerPattern::BGGR => "BGGR",
                BayerPattern::GRBG => "GRBG",
                BayerPattern::GBRG => "GBRG",
            }
        )
    }
}
impl BayerPattern {
    pub fn from_str(bayer_pattern: &str) -> BayerPattern {
        match bayer_pattern.to_uppercase().trim() {
            "RGGB" => BayerPattern::RGGB,
            "BGGR" => BayerPattern::BGGR,
            "GRBG" => BayerPattern::GRBG,
            "GBRG" => BayerPattern::GBRG,
            _ => panic!("Could not parse Bayer pattern from str: Unexpected value given."),
        }
    }

    fn color_offsets(&self) -> Vec<u8> {
        match self {
            BayerPattern::RGGB => vec![0, 1, 1, 2],
            BayerPattern::BGGR => vec![2, 1, 1, 0],
            BayerPattern::GRBG => vec![1, 0, 2, 1],
            BayerPattern::GBRG => vec![1, 2, 0, 1],
        }
    }
}

pub struct RawImage {
    width: u32,
    height: u32,
    data: Vec<u16>,
    bayer_pattern: BayerPattern,
}

impl RawImage {
    pub fn save_as_dng(&self, file_path: &str) {

        // Image bytes
        let mut image_bytes = Vec::new();

        for &val in self.data.iter() {
            image_bytes.write_u16::<LittleEndian>(val).unwrap();
        }

        const TAG_CFAREPEARPATTERNDIM:  u16 = 0x828D;
        const TAG_CFAPATTERN2:          u16 = 0x828E;
        const TAG_DNGVERSION:           u16 = 0xC612;
        const TAG_COLORMATRIX1:         u16 = 0xC621;
        const TAG_ASSHOTNEUTRAL:        u16 = 0xC628;
        const TAG_ASSHOTWHITEXY:        u16 = 0xC629;

        TiffFile::new(
            Ifd::new()
                .with_entry(tag::PhotometricInterpretation, SHORT::single(32803))
                .with_entry(tag::NewSubfileType,            LONG::single(0))
                .with_entry(tag::ImageWidth,                LONG::single(self.width))
                .with_entry(tag::ImageLength,               LONG::single(self.height))
                .with_entry(tag::BitsPerSample,             SHORT::single(16))
                .with_entry(tag::Compression,               SHORT::single(1))
                .with_entry(tag::Orientation,               SHORT::single(1))
                .with_entry(tag::SamplesPerPixel,           SHORT::single(1))
                .with_entry(tag::RowsPerStrip,              LONG::single(self.height))
                .with_entry(tag::StripByteCounts,           LONG::single(self.width * self.height * 2))
                .with_entry(TAG_CFAREPEARPATTERNDIM,        SHORT::values(vec![2,2]))
                .with_entry(TAG_CFAPATTERN2,                BYTE::values(self.bayer_pattern.color_offsets()))
                .with_entry(TAG_DNGVERSION,                 BYTE::values(vec![1, 4, 0, 0]))
                .with_entry(TAG_COLORMATRIX1,               SRATIONAL::values(vec![
                                                                (4124564,10000000), (3575761,10000000), (1804375,10000000),
                                                                (2126729,10000000), (7151522,10000000), (0721750,10000000),
                                                                (0193339,10000000), (1191920,10000000), (9503041,10000000)
                                                            ]))
                .with_entry(TAG_ASSHOTNEUTRAL,              SRATIONAL::values(vec![(1,1),(1,1),(1,1)]))
                .with_entry(TAG_ASSHOTWHITEXY,              SRATIONAL::values(vec![(1,1),(1,1)]))
                .with_entry(tag::StripOffsets,              ByteBlock::single(image_bytes))
                .single()
        ).write_to(file_path).unwrap();
    }
}