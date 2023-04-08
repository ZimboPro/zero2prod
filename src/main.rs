use std::net::TcpListener;

use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let port = 8000;
    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))
        .expect(format!("Port {port} already is use").as_str());
    run(listener)?.await
}
