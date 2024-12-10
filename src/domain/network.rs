use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone)]
pub struct Network {
    peers: Arc<Mutex<HashSet<String>>>,
}

impl Network {
    pub fn new() -> Self {
        Network {
            peers: Arc::new(Mutex::new(HashSet::new())),
        }
    }

    pub async fn start_listening(&self, bind_addr: &str) -> tokio::task::JoinHandle<()> {
        let listener: TcpListener = TcpListener::bind(bind_addr)
            .await
            .expect("Failed to bind to port");

        println!("Listening for peers on {}", bind_addr);

        let peers = Arc::clone(&self.peers);
        tokio::spawn(async move {
            loop {
                let mut peers = {
                    // Acquire lock briefly
                    let locked_peers = peers.lock().await;
                    locked_peers.clone()
                };

                if let Ok((stream, addr)) = listener.accept().await {
                    println!("New connection from {}", addr);
                    peers.insert(addr.to_string());
                    Network::handle_connection(stream).await;
                }
            }
        })
    }

    pub async fn handle_connection(mut stream: TcpStream) {
        let mut buffer = vec![0; 1024];
        loop {
            buffer.fill(0);
            match stream.read(&mut buffer).await {
                Ok(0) => {
                    println!("Connection closed by client");
                    break;
                }
                Ok(n) => {
                    let message = String::from_utf8_lossy(&buffer[..n]);
                    println!("Received: {}", message);

                    // Echo the message back (you can replace this with custom handling logic)
                    if let Err(err) = stream.write_all(message.as_bytes()).await {
                        eprintln!("Failed to write to stream: {}", err);
                        break;
                    }
                }
                Err(err) => {
                    eprintln!("Connection error: {}", err);
                    break;
                }
            }
        }
    }

    pub async fn send_message(&self, addr: &str, message: &str) -> tokio::io::Result<()> {
        let mut stream: TcpStream = TcpStream::connect(addr).await?;
        stream.write_all(message.as_bytes()).await?;

        let mut buffer = vec![0; 1024];
        let n = stream.read(&mut buffer).await?;
        println!(
            "Acknowledgment received: {}",
            String::from_utf8_lossy(&buffer[..n])
        );

        Ok(())
    }

    pub async fn add_peer(&self, addr: String) {
        self.peers.lock().await.insert(addr.to_string());
    }

    pub async fn add_peers(&mut self, addrs: Vec<&str>) {
        for addr in addrs {
            self.peers.lock().await.insert(addr.to_string());
        }
    }

    pub async fn get_peers(&self) -> Vec<String> {
        self.peers.lock().await.iter().cloned().collect()
    }
}
