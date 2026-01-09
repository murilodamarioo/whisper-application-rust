pub fn flip_last_bit(byte: u8) -> u8 {
    return byte ^ 1;
}

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
}
