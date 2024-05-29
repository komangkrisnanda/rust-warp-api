mod handlers;
mod models;
mod routes;

use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    //Load env.

    dotenv().ok();

    //Validate env.
    let app_port: u16 = env::var("APP_PORT")
        .expect("APP_PORT must be set.")
        .parse()
        .expect("APP_PORT must be a valid u16 number.");

    //Map routes.
    let routes = routes::routes();

    //Serve app.
    println!("Server started at http://localhost:{}", app_port);
    warp::serve(routes).run(([127, 0, 0, 1], app_port)).await;
}
