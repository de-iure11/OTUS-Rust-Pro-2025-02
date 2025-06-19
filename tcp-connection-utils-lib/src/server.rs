use crate::error::Result;
use std::net::SocketAddr;
use tokio::{
    io,
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

/// Сервер - иммитатор умной розетки.
pub struct SimpleServer {
    server: TcpListener,
}

impl SimpleServer {
    /// Закрепляем сервер на сокете.
    pub async fn bind<Addrs: ToSocketAddrs>(addrs: Addrs) -> io::Result<Self> {
        let server = TcpListener::bind(addrs).await?;
        Ok(Self { server })
    }

    /// Принимаем входящее соединение.
    pub async fn accept(&self) -> Result<Connection> {
        let (stream, _) = self.server.accept().await?;
        Ok(Connection { stream })
    }

    pub async fn local_addr(&self) -> io::Result<SocketAddr> {
        self.server.local_addr()
    }
}

/// Соединение с клиентом и обработка запросов.
pub struct Connection {
    stream: TcpStream,
}

impl Connection {
    /// Обработка запроса асинхронно (неблокирующее сетевое взаимодействие).
    pub async fn process_request_async<F, Fut>(&mut self, handler: F) -> Result<()>
    where
        Fut: Future<Output = String>,
        F: FnOnce(String) -> Fut,
    {
        let request = super::recv_string_async(&mut self.stream).await?;
        let response = handler(request).await;
        super::send_string_async(&response, &mut self.stream).await?;
        Ok(())
    }

    /// Адрес подключенного клиента
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        self.stream.peer_addr()
    }
}
