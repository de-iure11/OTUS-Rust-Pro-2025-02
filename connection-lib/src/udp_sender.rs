use crate::error::{Error, Result};

use std::{net::SocketAddr, time::Duration};
use tokio::{
    net::{ToSocketAddrs, UdpSocket},
    time,
};

/// UDP клиент.
#[derive(Debug)]
pub struct UdpSender {
    pub socket: UdpSocket,
    server_addr: SocketAddr,
    interval: Duration,
}

impl UdpSender {
    /// Создаём клиент
    pub async fn new<Addrs: ToSocketAddrs>(server_addr: Addrs, interval_secs: u64) -> Result<Self> {
        // Биндимся на случайный порт (т.к. мы клиент)
        let socket = UdpSocket::bind("0.0.0.0:0").await?;

        // Разрешаем адрес сервера
        let mut resolved = tokio::net::lookup_host(server_addr)
            .await
            .map_err(|e| Error::Io(e))?;

        let server_addr = resolved.next().ok_or(Error::AddrResolutionFailed)?;

        Ok(Self {
            socket,
            server_addr,
            interval: Duration::from_secs(interval_secs),
        })
    }

    /// Запускаем периодическую отправку данных
    pub async fn run(&self) -> Result<()> {
        let mut interval = time::interval(self.interval);

        loop {
            interval.tick().await;

            let temp = generate_temperature();
            let message = format!("{:.2}", temp);

            self.socket
                .send_to(message.as_bytes(), self.server_addr)
                .await
                .map_err(|_| Error::FailedToSendData)?;
        }
    }
}

/// Генерация случайной температуры (15.0..30.0)
pub fn generate_temperature() -> f32 {
    15.0 + (rand::random::<f32>() * 15.0)
}
