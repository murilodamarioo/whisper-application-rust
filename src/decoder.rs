use crate::utils::bits_to_char;
use image::{DynamicImage, GenericImageView};

pub fn decode(img: &DynamicImage) -> String {
    let mut message = String::new();
    let mut current_bits: [u8; 8] = [0u8; 8];
    let mut bit_index = 0;

    let (width, height) = img.dimensions();

    'outer: for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);

            for i in 0..3 {
                let extracted_bit = pixel.0[i] & 1;

                current_bits[bit_index] = extracted_bit;
                bit_index += 1;

                if bit_index == 8 {
                    let character = bits_to_char(current_bits);

                    if character == '\0' {
                        break 'outer;
                    }

                    message.push(character);
                    bit_index = 0;
                    current_bits = [0u8; 8];
                }
            }
        }
    }

    return message;
}
