use std::net::TcpListener;

use newsletter::startup::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind to localhost.");
    run(listener)?.await
}
