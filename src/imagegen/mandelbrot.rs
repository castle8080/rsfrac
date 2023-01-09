use image::{ImageBuffer, RgbImage};

// A simple image generation used to test end to end.
pub fn generate_test_image() -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let img: RgbImage = ImageBuffer::from_fn(1024, 1024, |x, y| {
        let r = ((x / 5) % 255) as u8;
        let b = ((y / 5) % 255) as u8;
        image::Rgb([r, 0, b])
    });
    img
}

pub fn generate_png() -> Vec<u8> {
    let img = generate_test_image();

    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut bytes),
        image::ImageOutputFormat::Png,
    )
    .unwrap();

    bytes
}
