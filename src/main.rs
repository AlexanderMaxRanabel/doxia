mod client;
mod server;

use std::env;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let ip = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
        let conf = std::fs::read_to_string(std::env::args().nth(2).unwrap()).unwrap();
        tokio::spawn(async {
            let _ = client::client_runtime(ip, conf);
        });
    } else {
        // Build router
        let app = Router::new().route("/", get(handler));

        // Run server
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3133")
            .await
            .unwrap();
        println!("Listening on http://127.0.0.1:3133");
        axum::serve(listener, app).await.unwrap();
    }
    Ok(())
}

async fn handler() -> &'static str {
    "This is: DOXIA"
}
