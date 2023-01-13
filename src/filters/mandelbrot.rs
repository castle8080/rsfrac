use std::collections::HashMap;
use warp::http::Response;
use warp::Filter;

use crate::imagegen::mandelbrot::{generate_png, ImageRequest, Colors};

/// Handles a request to generate a mandelbrot image.
/// A response is given in PNG format.
pub fn mandelbrot() -> impl Filter<Extract = (Response<Vec<u8>>,), Error = warp::Rejection> + Copy {
    warp::path!("images" / "mandelbrot")
        .and(warp::query::<HashMap<String, String>>())
        .map(|params: HashMap<String, String>| {
            println!("Query: {:?}", params);

            // TODO: if parameters aren't specified it would be nice to issue a redirect.

            let mut req = ImageRequest {
                position: (-1.0, 0.0),
                width: 4.0,
                height: 4.0,
                pixel_width: 800,
                pixel_height: 800,
                max_iterations: 250,
                start_color: Colors::white(),
                end_color: Colors::black(),
            };

            req.position.0 = params.get("x").map_or(-1.0, |s| s.parse::<f64>().unwrap());
            req.position.1 = params.get("y").map_or(0.0, |s| s.parse::<f64>().unwrap());
            req.width = params.get("width").map_or(4.0, |s| s.parse::<f64>().unwrap());
            req.height = params.get("height").map_or(4.0, |s| s.parse::<f64>().unwrap());
            req.max_iterations = params.get("max_iterations").map_or(250, |s| s.parse::<u16>().unwrap());
            req.pixel_width = params.get("pixel_width").map_or(600, |s| s.parse::<u32>().unwrap());
            req.pixel_height = params.get("pixel_height").map_or(600, |s| s.parse::<u32>().unwrap());

            req.start_color = params.get("start_color").map_or(Colors::white(), |s| Colors::parse(s).unwrap());
            req.end_color = params.get("end_color").map_or(Colors::black(), |s| Colors::parse(s).unwrap());

            let img = generate_png(&req);

            Response::builder()
                .header("Content-Type", "image/png")
                .body(img)
                .unwrap()
        })
}
