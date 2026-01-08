use image::{DynamicImage, GenericImageView, Rgba};

fn main() {
    let img: DynamicImage =
        image::open("src/images/Albert_Einstein.png").expect("Failed to open image");

    let (width, height) = img.dimensions();
    println!("Width: {}px, Height: {}px", width, height);

    let pixel: Rgba<u8> = img.get_pixel(0, 0);

    let r: u8 = pixel.0[0];
    let g: u8 = pixel.0[1];
    let b: u8 = pixel.0[2];
    let a: u8 = pixel.0[3];

    println!("Pixel at (0, 0): R={}, G={}, B={}, A={}", r, g, b, a);

    let total_pixels: u64 = (width as u64) * (height as u64);

    let total_bits_capacity: u64 = total_pixels * 4;

    let total_bytes_capacity: u64 = total_bits_capacity / 8;

    println!("\n--- Statistics ---");
    println!("Total Pixels  : {}", total_pixels);
    println!("Bit Capacity  : {} bits", total_bits_capacity);
    println!(
        "Byte Capacity : {} bytes (~{} KB)",
        total_bytes_capacity,
        total_bytes_capacity / 1024
    );

    println!("Max Message   : {} characters", total_bytes_capacity);
}
