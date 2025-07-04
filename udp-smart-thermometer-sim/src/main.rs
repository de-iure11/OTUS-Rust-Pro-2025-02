use anyhow::Result;
use connection_lib::udp_sender::UdpSender;
use serde::Deserialize;
use std::{fs, path::Path};

#[derive(Debug, Deserialize)]
struct Config {
    address: String,
    interval_secs: u64,
}

/// Имитатор умного термометра:
/// Реализован с использованием неблокирующего сетевого взаимодействия.
/// Читает адрес для отправки UDP-пакетов и временной период отправки из файла.
/// Отправляет произвольное значение температуры на указанный адрес с указанной периодичностью.
#[tokio::main]
async fn main() -> Result<()> {
    let config = read_config(Path::new("config.toml"))?;

    let sender = UdpSender::new(config.address, config.interval_secs).await?;
    sender.run().await?;

    Ok(())
}

fn read_config(path: &Path) -> Result<Config> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
