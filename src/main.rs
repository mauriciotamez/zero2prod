use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();

    println!("Server started on port {}", port);

    run(listener)?.await?;

    Ok(())
}
