pub fn flip_last_bit(byte: u8) -> u8 {
    return byte ^ 1;
}

pub fn char_to_bits(c: char) -> [u8; 8] {
    let byte: u8 = c as u8;
    let mut bits: [u8; 8] = [0u8; 8];

    for i in 0..8 {
        bits[i] = (byte >> (7 - i)) & 1;
    }

    return bits;
}

pub fn bits_to_char(bits: [u8; 8]) -> char {
    let mut byte: u8 = 0u8;

    for i in 0..8 {
      byte = byte | bits[i] << (7 - i);
    }

    return byte as char;
}


pub fn print_whisper_art() {
    let cyan = "\x1b[36m";
    let reset = "\x1b[0m";
    
    let whisper_art = r#"
  ██╗    ██╗██╗  ██╗██╗███████╗██████╗ ███████╗██████╗ 
  ██║    ██║██║  ██║██║██╔════╝██╔══██╗██╔════╝██╔══██╗
  ██║ █╗ ██║███████║██║███████╗██████╔╝█████╗  ██████╔╝
  ██║███╗██║██╔══██║██║╚════██║██╔═══╝ ██╔══╝  ██╔══██╗
  ╚███╔███╔╝██║  ██║██║███████║██║     ███████╗██║  ██║
   ╚══╝╚══╝ ╚═╝  ╚═╝╚═╝╚══════╝╚═╝     ╚══════╝╚═╝  ╚═╝
    "#;
    
    println!("{}{}{}", cyan, whisper_art, reset);
}

// Uso:
// print_whisper_art(cyan, reset);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_flip_the_byte() {
        assert_eq!(flip_last_bit(0b00000000), 0b00000001);
        assert_eq!(flip_last_bit(0b00000001), 0b00000000);
        assert_eq!(flip_last_bit(254), 255);
        assert_eq!(flip_last_bit(255), 254);
    }

    #[test]
    fn should_convert_char_to_bits() {
        assert_eq!(char_to_bits('A'), [0, 1, 0, 0, 0, 0, 0, 1]);
        assert_eq!(char_to_bits('B'), [0, 1, 0, 0, 0, 0, 1, 0]);
        assert_eq!(char_to_bits('C'), [0, 1, 0, 0, 0, 0, 1, 1]);
    }

    #[test]
    fn should_convert_bits_to_char() {
        assert_eq!(bits_to_char([0, 1, 0, 0, 0, 0, 0, 1]), 'A');
        assert_eq!(bits_to_char([0, 1, 0, 0, 0, 0, 1, 0]), 'B');
        assert_eq!(bits_to_char([0, 1, 0, 0, 0, 0, 1, 1]), 'C');
    }
}
