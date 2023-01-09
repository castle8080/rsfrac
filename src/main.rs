mod crypto;
mod filters;
mod imagegen;

use warp::Filter;

#[tokio::main]
async fn main() {
    let server_port = 3030;
    let cert_path = "etc/tls/cert.pem";
    let key_path = "etc/tls/key.rsa";

    println!("Checking need for self signed certificates:");
    crypto::cert::generate_self_signed_certs_if_not_exists(cert_path, key_path).unwrap();

    println!("Configuring server route handling:");

    let routes =
        filters::hello::hello()
            .or(filters::mandelbrot::mandelbrot())
            .or(warp::get().and(warp::fs::dir("./static")))
            .recover(filters::common::handle_rejection);

    println!("Starting server: https://localhost:{}", server_port);
    warp::serve(routes)
        .tls()
        .cert_path(cert_path)
        .key_path(key_path)
        .run(([0, 0, 0, 0], server_port))
        .await;
}