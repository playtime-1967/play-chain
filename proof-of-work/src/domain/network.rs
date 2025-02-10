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
        Self {
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
                if let Ok((stream, addr)) = listener.accept().await {
                    //the lock is held only for the duration of inserting the new peer into the peers set, and then the lock is released.
                    let mut peers = {
                        let locked_peers = peers.lock().await;
                        locked_peers.clone()
                    };
                    peers.insert(addr.to_string());
                    println!("New connection from {}", addr);

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

                    //TODO: replace this with custom handling logic.
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
        self.peers.lock().await.insert(addr);
    }

    pub async fn add_peers(&mut self, addrs: Vec<String>) {
        let mut peers = self.peers.lock().await;
        for addr in addrs {
            peers.insert(addr);
        }
    }

    pub async fn get_peers(&self) -> Vec<String> {
        self.peers.lock().await.iter().cloned().collect()
    }
}
