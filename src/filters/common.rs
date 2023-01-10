use std::convert::Infallible;

use warp::http::StatusCode;
use warp::{Rejection, Reply};

/// Handles errors and sends back appropriate HTTP status codes.
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    if err.is_not_found() {
        Ok(warp::reply::with_status(
            String::from("Not found."),
            StatusCode::NOT_FOUND,
        ))
    } else {
        println!("Error: {:?}", err);
        Ok(warp::reply::with_status(
            format!("Error: {:?}", err),
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
