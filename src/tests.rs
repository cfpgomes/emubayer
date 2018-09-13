use super::*;

fn get_input_vector() -> Vec<u8> {
    // R->1 G->2 B->3
    vec![
        1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3,
        1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3,
        1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3,
        1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3,
        1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3,
        1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3,
        1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3,
        1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3, 1, 2, 3,
    ]
}

#[test]
fn test_image_extract_rggb() {
    let rgb_image = RgbImage {
        width: 8,
        height: 8,
        data: get_input_vector(),
        bit_depth: BitDepth::Eight,
    };

    let raw_image = rgb_image.to_raw(BayerPattern::RGGB);

    assert_eq!(raw_image.data, vec![
        1, 2, 1, 2, 1, 2, 1, 2,
        2, 3, 2, 3, 2, 3, 2, 3,
        1, 2, 1, 2, 1, 2, 1, 2,
        2, 3, 2, 3, 2, 3, 2, 3,
        1, 2, 1, 2, 1, 2, 1, 2,
        2, 3, 2, 3, 2, 3, 2, 3,
        1, 2, 1, 2, 1, 2, 1, 2,
        2, 3, 2, 3, 2, 3, 2, 3,
    ]);
}

#[test]
fn test_image_extract_bggr() {
    let rgb_image = RgbImage {
        width: 8,
        height: 8,
        data: get_input_vector(),
        bit_depth: BitDepth::Eight,
    };

    let raw_image = rgb_image.to_raw(BayerPattern::BGGR);

    assert_eq!(raw_image.data, vec![
        3, 2, 3, 2, 3, 2, 3, 2,
        2, 1, 2, 1, 2, 1, 2, 1,
        3, 2, 3, 2, 3, 2, 3, 2,
        2, 1, 2, 1, 2, 1, 2, 1,
        3, 2, 3, 2, 3, 2, 3, 2,
        2, 1, 2, 1, 2, 1, 2, 1,
        3, 2, 3, 2, 3, 2, 3, 2,
        2, 1, 2, 1, 2, 1, 2, 1,
    ]);
}

#[test]
fn test_image_extract_grbg() {
    let rgb_image = RgbImage {
        width: 8,
        height: 8,
        data: get_input_vector(),
        bit_depth: BitDepth::Eight,
    };

    let raw_image = rgb_image.to_raw(BayerPattern::GRBG);

    assert_eq!(raw_image.data, vec![
        2, 1, 2, 1, 2, 1, 2, 1,
        3, 2, 3, 2, 3, 2, 3, 2,
        2, 1, 2, 1, 2, 1, 2, 1,
        3, 2, 3, 2, 3, 2, 3, 2,
        2, 1, 2, 1, 2, 1, 2, 1,
        3, 2, 3, 2, 3, 2, 3, 2,
        2, 1, 2, 1, 2, 1, 2, 1,
        3, 2, 3, 2, 3, 2, 3, 2,
    ]);
}

#[test]
fn test_image_extract_gbrg() {
    let rgb_image = RgbImage {
        width: 8,
        height: 8,
        data: get_input_vector(),
        bit_depth: BitDepth::Eight,
    };

    let raw_image = rgb_image.to_raw(BayerPattern::GBRG);

    assert_eq!(raw_image.data, vec![
        2, 3, 2, 3, 2, 3, 2, 3,
        1, 2, 1, 2, 1, 2, 1, 2,
        2, 3, 2, 3, 2, 3, 2, 3,
        1, 2, 1, 2, 1, 2, 1, 2,
        2, 3, 2, 3, 2, 3, 2, 3,
        1, 2, 1, 2, 1, 2, 1, 2,
        2, 3, 2, 3, 2, 3, 2, 3,
        1, 2, 1, 2, 1, 2, 1, 2,
    ]);
}