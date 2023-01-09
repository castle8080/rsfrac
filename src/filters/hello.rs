use warp::Filter;

// Just a basic handler to test out sending text content.
pub fn hello() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Copy {
    warp::path!("hello" / String)
        .and(warp::get())
        .map(|name| format!("Hello, {}!", name))
}
