use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, ToSocketAddrs},
    sync::broadcast,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub(crate) fn main() -> Result<()> {
    accept_loop("127.0.0.1:8080");
    Ok(())
}

#[tokio::main]
async fn accept_loop(addr: impl ToSocketAddrs) {
    let listener = TcpListener::bind(addr).await.unwrap();
    let (tx, _rx) = broadcast::channel(25);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();

        println!("Accepting from: {:?}", socket.peer_addr().unwrap());

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            // Split read & write
            let (read, mut writer) = socket.split();

            let mut reader = BufReader::new(read);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }

                        tx.send((line.clone(), addr)).unwrap();
                        line.clear()
                    }

                    result = rx.recv() => {
                        let (msg, other_addr) = result.unwrap();

                        if addr != other_addr {
                            // Writes an entire buffer to the ouput
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
