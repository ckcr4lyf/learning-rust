use tokio::{io::AsyncWriteExt, net::{TcpListener, TcpStream}};

struct Config {
    data: String,
}

#[tokio::main]
async fn main() {
    // Bind the listener to the address
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    // Assume config is initialized / read when program starts
    // e.g. from CLI argument or config file
    let cfg = Config {
        data: "HELLO".to_string()
    };

    loop {
        // The second item contains the IP and port of the new connection.
        let (mut socket, _) = listener.accept().await.unwrap();
        
        tokio::spawn(async move {
            // assume we need to spawn for some long running I/O task
            // but we need to use config (for READ ONLY) in this
            socket.write(cfg.data.as_bytes()).await;
        });
    }
}
