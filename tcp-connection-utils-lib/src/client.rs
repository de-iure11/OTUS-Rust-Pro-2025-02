use crate::error::Result;
use std::net::{TcpStream, ToSocketAddrs};

/// Клиент STP.
pub struct SimpleClient {
    stream: TcpStream,
}

impl SimpleClient {
    /// Пытаемся подключится к серверу
    pub fn connect<Addrs>(addrs: Addrs) -> Result<Self>
    where
        Addrs: ToSocketAddrs,
    {
        let stream = TcpStream::connect(addrs)?;
        Ok(Self { stream })
    }

    /// Отправка запроса на сервер и получение ответа (взаимодействие организовано синхронно).
    pub fn send_request<R: AsRef<str>>(&mut self, req: R) -> Result<String> {
        crate::send_string(req, &mut self.stream)?;
        let response = crate::recv_string(&mut self.stream)?;
        Ok(response)
    }
}
