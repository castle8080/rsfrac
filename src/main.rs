mod crypto;
mod filters;
mod imagegen;

use warp::Filter;

/// Start and run an http sever with a few handlers.
#[tokio::main]
async fn main() {
    let server_port = 3030;
    let cert_path = "etc/tls/cert.pem";
    let key_path = "etc/tls/key.rsa";

    // Generate self-signed certificates if the certs don't already exist.
    println!("Checking need for self signed certificates:");
    crypto::cert::generate_self_signed_certs_if_not_exists(cert_path, key_path).unwrap();

    // Sets up the different handlers.
    // TODO: Is there a way to hook a panic and recover from that?
    println!("Configuring server route handling:");
    let routes =
        // A hello world style handler
        filters::hello::hello()
            // Dynamic generation of mandel brot images.
            .or(filters::mandelbrot::mandelbrot())
            // Setups up serving of static services.
            .or(warp::get().and(warp::fs::dir("./static")))
            // Handles errors such as not found.
            .recover(filters::common::handle_rejection);

    println!("Starting server: https://localhost:{}", server_port);
    warp::serve(routes)
        .tls()
        .cert_path(cert_path)
        .key_path(key_path)
        .run(([0, 0, 0, 0], server_port))
        .await;
}