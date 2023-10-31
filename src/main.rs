use tokio::{io::BufStream, net::TcpStream, net::TcpListener, io::AsyncBufReadExt};
mod req;

static DEFAULT_PORT: &str = "8080";

#[tokio::main]
// async fn main() -> anyhow::Result<()> {
    async fn main() -> anyhow::Result<()> {
    let port: u16 = std::env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_PORT.to_string())
        .parse()?;

    let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await.unwrap();

    println!("listening on {}", listener.local_addr()?);

    loop {
        let (stream, addr) = listener.accept().await?;
        let mut stream = BufStream::new(stream);
        tokio::spawn(async move {
            println!("new connection {}", addr);
            match handle_connection(&mut stream).await {
                Ok(()) => println!("all good"),
                Err(e) => println!("all bad {}", e),
            };
        });
    }
}

pub async fn handle_connection(stream: &mut BufStream<TcpStream>) -> anyhow::Result<()> {
    let mut line_buffer = String::new();
    stream.read_until(byte, buf)
    stream.read_line(&mut line_buffer).await?;
    println!("got this: {}", line_buffer);
    Ok(())
}