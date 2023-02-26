use std::net::TcpListener;

use email_newsletter::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let port = 3000;

    let listener = TcpListener::bind(("127.0.0.1", port))
        .expect(format!("The port {} is not avalible", port).as_ref());

    run(listener)?.await
}
