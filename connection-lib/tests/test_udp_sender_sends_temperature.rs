use connection_lib::udp_sender::{UdpSender, generate_temperature};
use std::net::SocketAddr;
use std::str;
use tokio::net::UdpSocket;
use tokio::task;

#[tokio::test]
async fn test_udp_sender_sends_temperature() {
    // Поднимаем UDP-сервер (имитатор приёмника)
    let server_socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
    let server_addr: SocketAddr = server_socket.local_addr().unwrap();

    // Стартуем отдельную задачу для чтения сообщения
    let receive_task = task::spawn(async move {
        let mut buf = [0u8; 1024];
        let (len, _addr) = server_socket.recv_from(&mut buf).await.unwrap();
        let msg = str::from_utf8(&buf[..len]).unwrap().to_string();
        msg
    });

    // Стартуем отправителя
    let sender = UdpSender::new(server_addr, 1).await.unwrap();

    // Только одну итерацию — без loop
    let temp = generate_temperature();
    let msg = format!("{:.2}", temp);
    sender
        .socket
        .send_to(msg.as_bytes(), server_addr)
        .await
        .unwrap();

    // Ждём приём сообщения
    let received = receive_task.await.unwrap();

    println!("Полученное сообщение: {}", received);

    // Проверяем, что сообщение — это float между 15.0 и 30.0
    let value: f32 = received.parse().unwrap();
    assert!((15.0..=30.0).contains(&value));
}
