use super::Result;
use crate::{error::Error, smart_house::Report};
use connection_lib::udp_receiver::UdpReceiver;

use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct Thermo {
    name: String,
    value: Arc<Mutex<f32>>,
    thermometer: Option<UdpReceiver>,
}

impl Thermo {
    pub async fn new(name: &str) -> Result<Self> {
        let addr = "127.0.0.1:8282";
        let Ok(thermometer) = UdpReceiver::new(addr).await else {
            return Err(Error::FailToConnectDevice(addr.to_string()));
        };

        Ok(Self {
            name: name.to_string(),
            value: Arc::new(Mutex::new(0.0_f32)),
            thermometer: Some(thermometer),
        })
    }
}

impl Thermo {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_value(&self) -> Option<String> {
        let value = *self.value.lock();
        Some(format!("{:.2}°C", value))
    }
}

impl Report for Thermo {
    fn get_report(&self) -> String {
        let value = self.get_value().unwrap_or("N/A".into());
        format!("Thermometer (name: {}, value: {} °C)", self.name, value)
    }
}

#[cfg(test)]
mod tests {
    // use super::*;
    // use connection_lib::udp_sender::UdpSender;
    // use std::{net::SocketAddr, time::Duration};
    // use tokio::time;

    // #[tokio::test]
    // async fn test_real_udp_communication() {
    //     // 1. Запускаем термометр (слушает порт 8282)
    //     let thermo = Thermo::new("test_thermo").await.unwrap();

    //     // 2. Запускаем имитатор (отправляет на порт 8282)
    //     let sender = UdpSender::new("127.0.0.1:8282", 1).await.unwrap();
    //     tokio::spawn(async move {
    //         sender.run().await.unwrap();
    //     });

    //     // 3. Даем время на обмен данными
    //     time::sleep(Duration::from_secs(3)).await;

    //     // 4. Проверяем, что термометр получил данные
    //     let value = thermo.get_value().unwrap();
    //     assert!(value.ends_with("°C"));

    //     let temp_value = value.replace("°C", "").parse::<f32>().unwrap();
    //     assert!(temp_value >= 15.0 && temp_value <= 30.0);

    //     // 5. Проверяем статус
    //     assert_eq!(thermo.get_status().unwrap(), "ON");

    //     // 6. Проверяем выключение
    //     thermo.turn_off().unwrap();
    //     assert_eq!(thermo.get_status().unwrap(), "OFF");
    //     assert_eq!(thermo.get_value().unwrap(), "0.00°C");
    // }
}
