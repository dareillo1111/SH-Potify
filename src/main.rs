use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
        let router = Router::new().route("/hi", get(say_hi));
        let address = "0.0.0.0:6570";
        let listener = tokio::net::TcpListener::bind(&address).await.unwrap();

        axum::serve(listener, router).await.unwrap();
}

async fn say_hi() -> String {
        "world".to_string()
}

async fn recive_hi(hi: String) {
        println!("recived {0}", hi)
}
