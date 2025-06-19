pub mod smart_socket;

use std::sync::Arc;
use tokio::sync::RwLock;

use tcp_connection_utils_lib::server::Connection;

pub async fn process_connection(
    mut connection: Connection,
    socket: Arc<RwLock<smart_socket::SmartSocket>>,
) {
    println!(
        "🚦 process_connection стартовал {}",
        connection.peer_addr().unwrap()
    );
    loop {
        let socket = socket.clone();

        // Обрабатываем запрос.
        let processing_result = connection
            .process_request_async(|req| async move {
                println!("📩 Получен запрос: {req}");

                if req == "status" {
                    return socket.read().await.status();
                }

                if req == "power" {
                    return socket.read().await.power().to_string();
                }

                if req == "on" {
                    let result = socket.write().await.on();
                    if result.is_err() {
                        return "Failed to turn on".to_string();
                    }
                    return socket.read().await.status();
                }

                if req == "off" {
                    let result = socket.write().await.off();
                    if result.is_err() {
                        return "Failed to turn off".to_string();
                    }
                    return socket.read().await.status();
                }

                // Если запрос неизвестен, возвращаем сообщение об ошибке.
                format!("Unknown request: {}", req)
            })
            .await;

        if let Err(e) = processing_result {
            eprintln!("Ошибка обработки запроса: {}", e);
            break;
        }
    }
}
