use std::collections::HashMap;
use warp::http::Response;
use warp::Filter;

use crate::imagegen::mandelbrot::generate_png;

pub fn mandelbrot() -> impl Filter<Extract = (Response<Vec<u8>>,), Error = warp::Rejection> + Copy {
    warp::path!("images" / "mandelbrot")
        .and(warp::query::<HashMap<String, String>>())
        .map(|params: HashMap<String, String>| {
            println!("Query: {:?}", params);

            let img = generate_png();

            Response::builder()
                .header("Content-Type", "image/png")
                .body(img)
                .unwrap()
        })
}
