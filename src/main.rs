use std::net::TcpListener;

use email_newsletter::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let port = 3000;

    let listener = TcpListener::bind(("127.0.0.1", port))
        .unwrap_or_else(|_| panic!("The port {} is not avalible", port));

    run(listener)?.await
}
