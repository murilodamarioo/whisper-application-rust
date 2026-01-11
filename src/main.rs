use image::DynamicImage;

fn main() {
    let img: DynamicImage =
        image::open("src/images/Albert_Einstein.png").expect("Failed to open image");

    let encoded_img = whisper::encoder::encode(
        &img,
        "We cannot solve our problems with the same thinking we used when we created them",
    );

    encoded_img
        .save("encoded_output.png")
        .expect("Failed to save image");
    println!("Message hidden successfully in encoded_output.png");
}
