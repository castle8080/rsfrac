use image::ImageBuffer;

pub type Color = image::Rgb<u8>;

pub struct Colors {
}

impl Colors {
    pub fn white() -> Color {
        image::Rgb([255, 255, 255])
    }

    pub fn black() -> Color {
        image::Rgb([0, 0, 0])
    }

    pub fn between(start_color: Color, end_color: Color, ratio: f64) -> Color {
        image::Rgb([
            Colors::_between(start_color[0], end_color[0], ratio),
            Colors::_between(start_color[1], end_color[1], ratio),
            Colors::_between(start_color[2], end_color[2], ratio),
        ])
    }

    fn _between(start: u8, end: u8, ratio: f64) -> u8 {
        (((end as i16 - start as i16) as f64 * ratio) + start as f64) as u8
    }

    pub fn parse(rgbstr: &String) -> Option<Color> {
        if rgbstr.len() == 0 || !rgbstr.starts_with("#") {
            None
        }
        else {
            match i64::from_str_radix(&rgbstr[1..rgbstr.len()], 16) {
                Err(_) => None,
                Ok(n) => {
                    Some(image::Rgb([
                        (n >> 16 & 0xff) as u8,
                        (n >> 8 & 0xff) as u8,
                        (n & 0xff) as u8,
                    ]))
                }
            }
        }
    }
} 

pub struct ImageRequest {
    pub position: (f64, f64),
    pub width: f64,
    pub height: f64,
    pub pixel_width: u32,
    pub pixel_height: u32,
    pub max_iterations: u16,
    pub start_color: Color,
    pub end_color: Color,
}

impl ImageRequest {
    fn get_pixel_position(&self, x: u32, y: u32) -> (f64, f64) {
        let img_x_left = self.position.0 - self.width / 2.0;
        let img_y_top = self.position.1 + self.height / 2.0;

        let img_x = img_x_left + (x as f64 / self.pixel_width as f64) * self.width;
        let img_y = img_y_top - (y as f64 / self.pixel_height as f64) * self.height;

        (img_x, img_y)
    }
}

fn get_escape(x0: f64, y0: f64, max_iterations: u16) -> f64 {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut iteration: u16 = 0;

    loop {
        if (x * x + y * y) > 4.0 {
            return iteration as f64 / max_iterations as f64;
        } else if iteration >= max_iterations {
            return 1.0;
        } else {
            let x_temp = x * x - y * y + x0;
            y = 2.0 * x * y + y0;
            x = x_temp;
            iteration += 1;
        }
    }
}

// A simple image generation used to test end to end.
pub fn generate_image(request: &ImageRequest) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    ImageBuffer::from_fn(request.pixel_width, request.pixel_height, |x, y| {
        let (x, y) = request.get_pixel_position(x, y);
        
        let escape_ratio = get_escape(x, y, request.max_iterations);
        Colors::between(request.start_color, request.end_color, escape_ratio)
    })
}

pub fn generate_png(request: &ImageRequest) -> Vec<u8> {
    let img = generate_image(request);

    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(
        &mut std::io::Cursor::new(&mut bytes),
        image::ImageOutputFormat::Png,
    )
    .unwrap();

    bytes
}
