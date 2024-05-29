mod handlers;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let routes = routes::routes();

    println!("Server started at https://localhost:8000");
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}
