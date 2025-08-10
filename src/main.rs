use axum::{
    response::Html, routing::{get, post},  Router
};

mod vehicle;

use vehicle::{vehice_post, vehicle_get, vehice_post_query};


#[tokio::main]
async fn main() {
    // Create the axum router 
    let app = Router::new()
        .route("/", get(handler))
        .route("/vehice", 
        get(vehicle_get)
                      .post(vehice_post))
        .route("/vehiceQuery", post(vehice_post_query)); 

    // 2. Define the IP and port listener (TCP)
    let address = "0.0.0.0:4000";
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    // 3. axum serve to launch the web server 
    axum::serve(listener,app).await.unwrap();

}


// 处理器
async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

