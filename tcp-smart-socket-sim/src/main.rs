use clap::Parser;

use connection_lib::tcp_server::SimpleServer;
use std::sync::Arc;
use tcp_smart_socket_sim::{process_connection, smart_socket::SmartSocket};
use tokio::sync::RwLock;

#[derive(Parser, Debug)]
#[command(name = "Smart Socket Server")]
#[command(about = "Иммитатор умной розетки", long_about = None)]
struct Args {
    /// Адрес для прослушивания (например, 127.0.0.1:8080)
    #[arg(short, long, default_value = "127.0.0.1:0")]
    listen: String,
}

/// Имитатор умной розетки:
/// Читает адрес для приёма TCP-соединений из аргументов командной строки.
/// Реализован с использованием неблокирующего сетевого взаимодействия.
/// Хранит состояние розетки.
/// Позволяет управлять розеткой множеству клиентов одновременно.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let server = SimpleServer::bind(&args.listen).await?;
    println!("Сервер слушает: {}", args.listen);

    // Создаём умную розетку.
    let smart_socket = Arc::new(RwLock::new(SmartSocket::new()));

    // Обрабатываем подключения клиентов.
    loop {
        let Ok(connection) = server.accept().await else {
            continue;
        };

        tokio::spawn(process_connection(connection, smart_socket.clone()));
    }
}
