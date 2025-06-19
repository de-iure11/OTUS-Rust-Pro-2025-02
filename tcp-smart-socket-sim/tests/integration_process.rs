// -----
#[tokio::test(flavor = "multi_thread", worker_threads = 1)]
async fn test_process_connection_integration_ok() {
    use std::net::TcpStream;
    use std::sync::Arc;
    use tcp_connection_utils_lib::server::SimpleServer;
    use tcp_connection_utils_lib::{recv_string, send_string};
    use tcp_smart_socket_sim::{process_connection, smart_socket::SmartSocket};
    use tokio::sync::{RwLock, mpsc};
    use tokio::time::{Duration, timeout};

    // Создаем умную розетку
    let smart_socket = Arc::new(RwLock::new(SmartSocket {
        enabled: true,
        amperes: 5.0,
        volts: 220.0,
    }));

    // Запускаем сервер
    let server = SimpleServer::bind("127.0.0.1:0").await.unwrap();
    let addr = server.local_addr().await.unwrap();
    println!("Сервер запущен на: {}", addr);

    // Канал для управления сервером
    let (shutdown_sender, mut shutdown_receiver) = mpsc::channel(1);

    // Запускаем сервер в фоне
    let server_handle = tokio::spawn({
        async move {
            loop {
                tokio::select! {
                    // Проверяем сигнал остановки
                    _ = shutdown_receiver.recv() => {
                        println!("Сервер получил сигнал остановки");
                        break;
                    }
                    // Принимаем новые подключения
                    accept_result = server.accept() => {
                        match accept_result {
                            Ok(connect) => {
                                println!("Принято новое подключение");
                                let smart_socket = smart_socket.clone();
                                tokio::spawn(async move {
                                    process_connection(connect, smart_socket).await;
                                });
                            }
                            Err(e) => {
                                eprintln!("Ошибка при принятии подключения: {}", e);
                                break;
                            }
                        }
                    }
                }
            }
            println!("Сервер остановлен");
        }
    });

    // Даем серверу время запуститься
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Подключаемся клиентом
    println!("Клиент подключается к {}", addr);
    let mut stream = TcpStream::connect(addr).unwrap();
    println!("Клиент подключен");

    // Проверка: статус по умолчанию
    send_string("status", &mut stream).unwrap();
    let resp = recv_string(&mut stream).unwrap();
    println!("Статус по умолчанию: {}", resp);
    assert_eq!(resp, "ON");

    // Мощность
    send_string("power", &mut stream).unwrap();
    let resp = recv_string(&mut stream).unwrap();
    println!("Мощность: {}", resp);
    assert!(resp.starts_with("1100"));

    // Выключение
    send_string("off", &mut stream).unwrap();
    let resp = recv_string(&mut stream).unwrap();
    println!("Текущий статус: {}", resp);
    assert_eq!(resp, "OFF");

    // Включение
    send_string("on", &mut stream).unwrap();
    let resp = recv_string(&mut stream).unwrap();
    println!("Текущий статус: {}", resp);
    assert_eq!(resp, "ON");

    // Останавливаем сервер
    shutdown_sender.send(()).await.unwrap();

    // Ждем завершения сервера с таймаутом
    timeout(Duration::from_secs(1), server_handle)
        .await
        .unwrap_or_else(|_| panic!("Сервер не завершился за отведенное время"))
        .unwrap();

    println!("Тест успешно завершен");
}
