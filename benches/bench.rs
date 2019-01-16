// Author: Cláudio Gomes (TofuLynx)
// Project: emubayer
// License: GNU GPL Version 3 (https://www.gnu.org/licenses/gpl-3.0.en.html)

#![feature(test)]

extern crate emubayer;
extern crate test;

use emubayer::*;
use test::Bencher;

static SAMPLE_EVEN_SIZE: &'static str = "samples/input/even_size.png";
static SAMPLE_ODD_SIZE: &'static str = "samples/input/odd_size.png";

#[bench]
fn bench_even_sample(b: &mut Bencher) {
    b.iter(|| {
        let rgb_image = RgbImage::from_file(SAMPLE_EVEN_SIZE).unwrap();

        let raw_image = rgb_image.to_raw(BayerPattern::RGGB);
        raw_image.save_as_dng("samples/output/even_size.dng");
    });
}

#[bench]
fn bench_odd_sample(b: &mut Bencher) {
    b.iter(|| {
        let rgb_image = RgbImage::from_file(SAMPLE_ODD_SIZE).unwrap();

        let raw_image = rgb_image.to_raw(BayerPattern::RGGB);
        raw_image.save_as_dng("samples/output/odd_size.dng");
    });
}
