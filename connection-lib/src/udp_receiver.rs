use crate::error::{Error, Result};

use std::net::SocketAddr;
use tokio::net::{ToSocketAddrs, UdpSocket};

/// UDP сервера.
#[derive(Debug)]
pub struct UdpReceiver {
    socket: UdpSocket,
    _server_addr: SocketAddr,
}

impl UdpReceiver {
    /// Создаем новый UDP сервер
    pub async fn new<Addrs: ToSocketAddrs>(server_addr: Addrs) -> Result<Self> {
        // Разрешаем адрес сервера
        let mut resolved = tokio::net::lookup_host(server_addr)
            .await
            .map_err(|e| Error::Io(e))?;
        let server_addr = resolved.next().ok_or(Error::AddrResolutionFailed)?;

        // Биндимся на указанный адрес (сервер)
        let socket = UdpSocket::bind(server_addr).await?;

        Ok(Self {
            socket,
            _server_addr: server_addr,
        })
    }

    /// Запускает цикл обработки входящих пакетов
    pub async fn run<F, Fut>(&self, mut handler: F) -> Result<()>
    where
        F: FnMut(String) -> Fut,
        Fut: Future<Output = String>,
    {
        let mut buf = [0u8; 1024];

        loop {
            let (len, addr) = self.socket.recv_from(&mut buf).await?;
            let request = String::from_utf8_lossy(&buf[..len]).to_string();
            println!("Получен от {}: {:?}", addr, request);
            let _response = handler(request).await;
        }
    }
}
