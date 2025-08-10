use axum::{
    response::Html, routing::get, Router
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler)); // http://127.0.0.1:3000

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener,app).await.unwrap();

}
// 处理器
async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

