use super::models::Post;

// Get request at /posts/{id}
pub async fn get_post(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    //return static post.
    let post = Post {
        id,
        title: String::from("Hello, Warp!"),
        body: String::from("This is a post about warp."),
    };
    Ok(warp::reply::json(&post))
}
