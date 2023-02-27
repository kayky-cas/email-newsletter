#[tokio::main]
async fn main() -> std::io::Result<()> {
    let port = 3000;

    let listener = std::net::TcpListener::bind(("127.0.0.1", port))
        .unwrap_or_else(|_| panic!("The port {} is not avalible", port));

    email_newsletter::run(listener)?.await
}
