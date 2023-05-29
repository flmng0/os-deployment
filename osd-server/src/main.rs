use anyhow::Result;

use async_std::net::{TcpListener, TcpStream};
use async_std::prelude::*;
use async_std::task;
use osd_core::{ClientMessage, SERVER_IP, SERVER_PORT};
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::{Pool, Sqlite};

struct Server {
    db: Pool<Sqlite>,
    listener: TcpListener,
}

impl Server {
    pub async fn new() -> Result<Self> {
        let db = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite::memory:")
            .await?;

        let listener = TcpListener::bind((SERVER_IP, SERVER_PORT)).await?;

        Ok(Self { db, listener })
    }

    pub async fn handle_client(&self, stream: &TcpStream) -> Result<()> {
        let mut bytes = Vec::new();
        stream.read_to_end(&mut bytes).await?;

        let message: ClientMessage = bson::from_reader(&mut bytes)?;

        dbg!(message);

        Ok(())
    }

    pub async fn start_listening(mut self) -> Result<()> {
        let mut incoming = self.listener.incoming();

        while let Some(stream) = incoming.next().await {
            let stream = stream?;

            let _client = task::spawn(self.handle_client(&stream));
        }

        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
