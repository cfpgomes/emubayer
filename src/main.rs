// Author: Cláudio Gomes (TofuLynx)
// Project: emubayer
// License: GNU GPL Version 3 (https://www.gnu.org/licenses/gpl-3.0.en.html)

extern crate png;
extern crate byteorder;


use std::fs::File;
use std::io::prelude::*;
use byteorder::{WriteBytesExt, BigEndian, LittleEndian};

mod info;
#[cfg(test)]
mod tests;

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
        let png_file = File::open(path)
            .map_err(|_| info::error::INVALID_PNG)?;

        let decoder = png::Decoder::new(png_file);
        let (info, mut reader) = decoder.read_info()
            .map_err(|_| info::error::DECODING_PNG)?;

        // TODO (TofuLynx): Evaluate whether or not to extend support to RGBA Color Type.
        if info.color_type != png::ColorType::RGB {
            return Err(info::error::INVALID_COLOR_TYPE);
        }

        let bit_depth = match info.bit_depth {
            png::BitDepth::Eight => BitDepth::Eight,
            png::BitDepth::Sixteen => BitDepth::Sixteen,
            _ => {
                return Err(info::error::INVALID_BIT_DEPTH);
            }
        };

        // Decode frame.
        let mut data = vec![0; info.buffer_size()];
        reader.next_frame(&mut data)
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
        let width = self.width as usize;
        let height = self.height as usize;
        let color_offsets = bayer_pattern.color_offsets();

        let mut raw_data: Vec<u16> = vec![0; self.size() as usize];
        let mut raw_index;

        for row in (0..height).step_by(2) {
            for column in (0..width).step_by(2) {
                // Top Left.
                raw_index = row * width + column;
                raw_data[raw_index] = (self.data[(raw_index * 3 + color_offsets[0])] as u16) << 8;

                // Top Right.
                raw_index += 1;
                raw_data[raw_index] = (self.data[(raw_index * 3 + color_offsets[1])] as u16) << 8;

                // Bottom Right.
                raw_index += width;
                raw_data[raw_index] = (self.data[(raw_index * 3 + color_offsets[3])] as u16) << 8;

                // Bottom Left.
                raw_index -= 1;
                raw_data[raw_index] = (self.data[(raw_index * 3 + color_offsets[2])] as u16) << 8;
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
    fn from_str(bayer_pattern: &str) -> Result<BayerPattern, &str> {
        match bayer_pattern.to_uppercase().trim() {
            "RGGB" => Ok(BayerPattern::RGGB),
            "BGGR" => Ok(BayerPattern::BGGR),
            "GRBG" => Ok(BayerPattern::GRBG),
            "GBRG" => Ok(BayerPattern::GBRG),
            _ => Err(info::error::INVALID_PATTERN),
        }
    }

    fn color_offsets(&self) -> Vec<usize> {
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
    pub fn save_as_dng(&self, file_path: &str) {
        // TODO (TofuLynx): RawImage.to_dng
        // TODO (TofuLynx): Handle errors.
        let mut dng_file = File::create(file_path).unwrap();

        // Write header.
        let mut header = Vec::new();
        header.write_u16::<LittleEndian>(0x4949).unwrap(); // Byte order: II
        header.write_u16::<LittleEndian>(42).unwrap(); // Magic number: 42
        header.write_u32::<LittleEndian>(8).unwrap(); // Offset to IFD0: 8

        dng_file.write_all(&header).unwrap();

        let ifd0_entries = 1;
        let raw_ifd_entries = 14;

        // Write IFD0.
        let mut ifd0 = Vec::new();
        ifd0.write_u16::<LittleEndian>(ifd0_entries).unwrap(); // Number of Entries

        // Entry 0
        ifd0.write_u16::<LittleEndian>(0x014A).unwrap(); // SubIFDs Offset
        ifd0.write_u16::<LittleEndian>(4).unwrap(); // Type: LONG
        ifd0.write_u32::<LittleEndian>(1).unwrap(); // Count: 1 value
        ifd0.write_u32::<LittleEndian>(8 + 6 + 12 * (ifd0_entries as u32)).unwrap(); // Value: Offset to raw subIFD

        

        ifd0.write_u32::<LittleEndian>(0).unwrap(); // OFFSET TO NEXT IFD (what is it?)

        dng_file.write_all(&ifd0).unwrap();

        // Write rawIFD.
        let mut raw_ifd = Vec::new();
        raw_ifd.write_u16::<LittleEndian>(raw_ifd_entries).unwrap(); // Number of Entries

        // Entry 0
        raw_ifd.write_u16::<LittleEndian>(0xFE).unwrap(); // NewSubFileType
        raw_ifd.write_u16::<LittleEndian>(4).unwrap(); // Type: LONG
        raw_ifd.write_u32::<LittleEndian>(1).unwrap(); // Count: 1 value
        raw_ifd.write_u32::<LittleEndian>(0).unwrap(); 

        // Entry 1
        raw_ifd.write_u16::<LittleEndian>(0x100).unwrap(); // ImageWidth
        raw_ifd.write_u16::<LittleEndian>(4).unwrap(); // Type: LONG
        raw_ifd.write_u32::<LittleEndian>(1).unwrap(); // Count: 1 value
        raw_ifd.write_u32::<LittleEndian>(self.width).unwrap(); 

        // Entry 2
        raw_ifd.write_u16::<LittleEndian>(0x101).unwrap(); // ImageLength
        raw_ifd.write_u16::<LittleEndian>(4).unwrap(); // Type: LONG
        raw_ifd.write_u32::<LittleEndian>(1).unwrap(); // Count: 1 value
        raw_ifd.write_u32::<LittleEndian>(self.height).unwrap(); 

        // Entry 3
        raw_ifd.write_u16::<LittleEndian>(0x102).unwrap(); // BitDepth
        raw_ifd.write_u16::<LittleEndian>(3).unwrap(); // Type: SHORT
        raw_ifd.write_u32::<LittleEndian>(1).unwrap(); // Count: 1 value
        raw_ifd.write_u16::<LittleEndian>(16).unwrap();
        raw_ifd.write_u16::<LittleEndian>(0).unwrap();

        // Entry 4
        raw_ifd.write_u16::<LittleEndian>(0x103).unwrap(); // Compression
        raw_ifd.write_u16::<LittleEndian>(3).unwrap(); // Type: SHORT
        raw_ifd.write_u32::<LittleEndian>(1).unwrap(); // Count: 1 value
        raw_ifd.write_u16::<LittleEndian>(1).unwrap(); // Value: Uncompressed
        raw_ifd.write_u16::<LittleEndian>(0).unwrap();

        // Entry 5
        raw_ifd.write_u16::<LittleEndian>(0x106).unwrap(); // PhotometricInterpretation
        raw_ifd.write_u16::<LittleEndian>(3).unwrap(); // Type: SHORT
        raw_ifd.write_u32::<LittleEndian>(1).unwrap(); // Count: 1 value
        raw_ifd.write_u16::<LittleEndian>(32803).unwrap(); // Value: Color Filter Array
        raw_ifd.write_u16::<LittleEndian>(0).unwrap();

        // Entry 6
        raw_ifd.write_u16::<LittleEndian>(0x111).unwrap(); // StripOffsets
        raw_ifd.write_u16::<LittleEndian>(4).unwrap(); // Type: LONG
        raw_ifd.write_u32::<LittleEndian>(1).unwrap(); // Count: 1 value
        raw_ifd.write_u32::<LittleEndian>(8 + 2*6 + 12 * (ifd0_entries as u32 + raw_ifd_entries as u32)).unwrap(); // Offset to image bytes

        // Entry 7
        raw_ifd.write_u16::<LittleEndian>(0x0112).unwrap(); // Orientation
        raw_ifd.write_u16::<LittleEndian>(3).unwrap(); // Type: SHORT
        raw_ifd.write_u32::<LittleEndian>(1).unwrap(); // Count: 1 value
        raw_ifd.write_u16::<LittleEndian>(1).unwrap(); // Value: Horizontal
        raw_ifd.write_u16::<LittleEndian>(0).unwrap();

        // Entry 8
        raw_ifd.write_u16::<LittleEndian>(0x115).unwrap(); // SamplesPerPixel
        raw_ifd.write_u16::<LittleEndian>(3).unwrap(); // Type: SHORT
        raw_ifd.write_u32::<LittleEndian>(1).unwrap(); // Count: 1 value
        raw_ifd.write_u16::<LittleEndian>(1).unwrap(); // Value: Number of color channels, 1 for monochrome
        raw_ifd.write_u16::<LittleEndian>(0).unwrap();

        // Entry 9
        raw_ifd.write_u16::<LittleEndian>(0x0116).unwrap(); // RowsPerStrip
        raw_ifd.write_u16::<LittleEndian>(4).unwrap(); // Type: LONG
        raw_ifd.write_u32::<LittleEndian>(1).unwrap(); // Count: 1 value
        raw_ifd.write_u32::<LittleEndian>(self.height).unwrap(); 

        // Entry 10
        raw_ifd.write_u16::<LittleEndian>(0x0117).unwrap(); // StripByteCounts
        raw_ifd.write_u16::<LittleEndian>(4).unwrap(); // Type: LONG
        raw_ifd.write_u32::<LittleEndian>(1).unwrap(); // Count: 1 value
        raw_ifd.write_u32::<LittleEndian>(self.width * self.height * 2).unwrap(); 


        // Entry 11
        raw_ifd.write_u16::<LittleEndian>(0x828D).unwrap(); // CFARepeatPatternDim
        raw_ifd.write_u16::<LittleEndian>(3).unwrap(); // Type: SHORT
        raw_ifd.write_u32::<LittleEndian>(2).unwrap(); // Count: 2 values
        raw_ifd.write_u16::<LittleEndian>(2).unwrap();
        raw_ifd.write_u16::<LittleEndian>(2).unwrap();

        // Entry 12
        raw_ifd.write_u16::<LittleEndian>(0x828E).unwrap(); // CFAPattern2
        raw_ifd.write_u16::<LittleEndian>(1).unwrap(); // Type: BYTE
        raw_ifd.write_u32::<LittleEndian>(4).unwrap(); // Count: 4 values
        raw_ifd.write_u8(0).unwrap(); // TODO: Support different patterns.
        raw_ifd.write_u8(1).unwrap();
        raw_ifd.write_u8(1).unwrap();
        raw_ifd.write_u8(2).unwrap();

        // Entry 13
        raw_ifd.write_u16::<LittleEndian>(0xC612).unwrap(); // DNGVersion
        raw_ifd.write_u16::<LittleEndian>(1).unwrap(); // Type: BYTE
        raw_ifd.write_u32::<LittleEndian>(4).unwrap(); // Count: 4 values
        raw_ifd.write_u8(0x01).unwrap(); 
        raw_ifd.write_u8(0x04).unwrap();
        raw_ifd.write_u8(0x00).unwrap();
        raw_ifd.write_u8(0x00).unwrap();

        raw_ifd.write_u32::<LittleEndian>(0).unwrap(); // OFFSET TO NEXT IFD (what is it?)

        dng_file.write_all(&raw_ifd).unwrap();

        // Image bytes.
        let mut image_bytes = Vec::new();

        for index in 0..self.data.len() {
            image_bytes.write_u16::<LittleEndian>(self.data[index]).unwrap();
        }

        dng_file.write_all(&image_bytes).unwrap();
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
    raw_image.save_as_dng("test.dng");

    println!("Hello, world!");
}
