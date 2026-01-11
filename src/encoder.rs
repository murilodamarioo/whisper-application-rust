use crate::utils::char_to_bits;
use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};

fn prepare_bits(message: &str) -> Vec<u8> {
    let mut bits: Vec<u8> = Vec::new();
    let message_with_null = format!("{}\0", message);

    for c in message_with_null.chars() {
        let char_bits = char_to_bits(c);
        bits.extend_from_slice(&char_bits);
    }

    return bits;
}

pub fn encode(img: &DynamicImage, message: &str) -> DynamicImage {
    let bits: Vec<u8> = prepare_bits(message);
    let mut bit_index: usize = 0;

    let (width, height) = img.dimensions();
    let mut output_img: ImageBuffer<Rgba<u8>, Vec<u8>> = img.to_rgba8();

    'outer: for y in 0..height {
        for x in 0..width {
            let mut pixel: Rgba<u8> = *output_img.get_pixel(x, y);

            for i in 0..3 {
                if bit_index < bits.len() {
                    let bit: u8 = bits[bit_index];

                    pixel.0[i] = (pixel.0[i] & 0xFE) | bit;

                    bit_index += 1;
                } else {
                    output_img.put_pixel(x, y, pixel);
                    break 'outer;
                }
            }
            output_img.put_pixel(x, y, pixel);
        }
    }
    return DynamicImage::ImageRgba8(output_img);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepare_bits_adds_null_terminator() {
        let message = "A";
        let bits = prepare_bits(message);

        assert_eq!(bits.len(), 16);

        assert_eq!(&bits[0..8], &[0, 1, 0, 0, 0, 0, 0, 1]);

        assert_eq!(&bits[8..16], &[0, 0, 0, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn test_encode_single_character() {
        let img = DynamicImage::new_rgba8(2, 2);

        let encoded_img = encode(&img, "A");

        let res_img = encoded_img.to_rgba8();

        let p0 = res_img.get_pixel(0, 0);
        assert_eq!(p0.0[0] % 2, 0);
        assert_eq!(p0.0[1] % 2, 1);
        assert_eq!(p0.0[2] % 2, 0);
    }
}
