use super::{ControllableDevice, Result};
use crate::{error::Error, smart_house::Report};
use connection_lib::tcp_client::SimpleClient;

#[derive(Debug)]
pub struct Socket {
    name: String,
    client: Option<SimpleClient>,
    is_real: bool, // Флаг для режима имитации
}

impl Socket {
    /// Создает новую розетку с реальным TCP-соединением
    pub fn new_real(name: &str, address: &str) -> Result<Self> {
        let Ok(client) = SimpleClient::connect(address) else {
            return Err(Error::FailToConnectDevice(address.to_string()));
        };
        Ok(Self {
            name: name.to_string(),
            client: Some(client),
            is_real: true,
        })
    }

    /// Создает розетку в тестовом режиме (имитация)
    pub fn new_mock(name: &str) -> Self {
        Self {
            name: name.to_string(),
            client: None, // Фиктивное соединение
            is_real: false,
        }
    }

    /// Внутренний метод для обработки команд
    fn process_command(&self, command: &str) -> Result<String> {
        if self.is_real {
            // Реальный режим - отправка по TCP
            let Some(client) = &self.client else {
                return Err(Error::FailToConnectDevice(self.name.clone()));
            };
            client
                .send_request(command)
                .map_err(|e| Error::FailToSendRequest(e.to_string()))
        } else {
            // Имитационный режим
            match command {
                "on" => Ok("ON".to_string()),
                "off" => Ok("OFF".to_string()),
                "status" => Ok("OFF".into()),
                "power" => Ok("1100.0".to_string()),
                _ => Err(Error::UnknownCommand(command.to_string())),
            }
        }
    }
}

impl ControllableDevice for Socket {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_value(&self) -> Option<String> {
        self.process_command("power").ok()
    }

    fn get_status(&self) -> Option<String> {
        self.process_command("status").ok()
    }

    fn turn_on(&self) -> Result<String> {
        let state = self.process_command("on")?;
        Ok(state)
    }

    fn turn_off(&self) -> Result<String> {
        let state = self.process_command("off")?;
        Ok(state)
    }
}

impl Report for Socket {
    fn get_report(&self) -> String {
        format!(
            "Socket (name: {}, status: {}, value: {} W)",
            self.name,
            self.get_status().unwrap_or_else(|| "UNKNOWN".to_string()),
            self.get_value().unwrap_or_else(|| "0.0".to_string())
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::time::Duration;

    // Тест с имитацией реальной работы
    // cargo test -p smart-home-lib test_mock_socket -- --nocapture
    #[test]
    fn test_mock_socket() {
        let socket = Socket::new_mock("test_socket");
        // Проверяем состояние
        assert_eq!(socket.get_status().as_deref(), Some("OFF"));
        // Включение
        assert_eq!(socket.turn_on().ok().as_deref(), Some("ON"));
        // Мощность
        assert_eq!(socket.get_value().as_deref(), Some("1100.0"));
        // Выключение
        assert_eq!(socket.turn_off().ok().as_deref(), Some("OFF"));
    }

    // Тест с реальной работой (реальный TCP-обмен)
    // cargo test -p smart-home-lib test_real_socket -- --nocapture
    #[tokio::test]
    async fn test_real_socket() {
        use tokio::process::Command;

        let addr = "127.0.0.1:9001";

        // Запускаем тестовый сервер в отдельм процессе
        // cargo run -p tcp-smart-socket-sim -- --listen 127.0.0.1:9001
        let _server_process = Command::new("cargo")
            .arg("run")
            .arg("-p")
            .arg("tcp-smart-socket-sim")
            .arg("--")
            .arg("--listen")
            .arg(addr)
            .kill_on_drop(true)
            .spawn()
            .expect("Ошибка запуска сервера");

        // Даем серверу время на запуск
        tokio::time::sleep(Duration::from_secs(1)).await;

        // Создаем клиент с реальным подключением
        let socket = Socket::new_real("test_socket", &addr.to_string()).unwrap();

        // Проверяем состояние
        let res = socket.get_status();
        println!("Response: {:?}", res);
        assert_eq!(res.as_deref(), Some("ON"));

        // Проверяем мощность
        let res = socket.get_value();
        println!("Response: {:?}", res);
        assert!(res.as_deref().is_some());

        // Проверяем включение (уже включено)
        let res = socket.turn_on();
        println!("Response: {:?}", res);
        assert_eq!(res.ok().as_deref(), Some("ON"));

        // Проверяем выключение
        let res = socket.turn_off();
        println!("Response: {:?}", res);
        assert_eq!(res.ok().as_deref(), Some("OFF"));

        // Проверяет вывод отчета (включаем сервер и печатаем отчет)
        let _res = socket.turn_on();
        let report = socket.get_report();
        println!("Отчет: {}", report);
        // Проверяем общую структуру отчёта через регулярное выражение
        let re = regex::Regex::new(
            r"Socket \(name: [\w-]+, status: (ON|OFF|UNKNOWN), value: \d+\.?\d* W\)",
        )
        .unwrap();
        assert!(re.is_match(&report), "Несоответствие формата отчета");
    }
}
