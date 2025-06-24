use crate::error::{Error, Result};
use std::io::{Read, Write};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub mod error;
pub mod tcp_client;
pub mod tcp_server;

/// Синхронно отправляет четыре байта `data.len()`, а потом сами данные.
pub fn send_string<D, W>(data: D, mut writer: W) -> Result<()>
where
    D: AsRef<str>,
    W: Write,
{
    let bytes = data.as_ref().as_bytes();
    let len = bytes.len() as u32;
    let len_bytes = len.to_be_bytes();
    writer.write_all(&len_bytes)?;
    writer.write_all(bytes)?;
    Ok(())
}

/// Асинхронно отправляет четыре байта `data.len()`, а потом сами данные.
pub async fn send_string_async<D, W>(data: D, writer: &mut W) -> Result<()>
where
    D: AsRef<str>,
    W: AsyncWrite + Unpin,
{
    let bytes = data.as_ref().as_bytes();
    let len = bytes.len() as u32;
    let len_bytes = len.to_be_bytes();
    writer.write_all(&len_bytes).await?;
    writer.write_all(bytes).await?;
    Ok(())
}

/// Синхронно читает четыре байта длины, а потом сами данные.
pub fn recv_string<R: Read>(mut reader: R) -> Result<String>
where
    R: Read,
{
    let mut buf = [0; 4];
    reader.read_exact(&mut buf)?;
    let len = u32::from_be_bytes(buf);
    let mut buf = vec![0; len as _];
    reader.read_exact(&mut buf)?;
    String::from_utf8(buf).map_err(|_| Error::BadEncoding)
}

/// Асинхронно читает четыре байта длины, а потом сами данные.
pub async fn recv_string_async<R>(reader: &mut R) -> Result<String>
where
    R: AsyncRead + Unpin,
{
    let mut buf = [0; 4];
    reader.read_exact(&mut buf).await?;
    let len = u32::from_be_bytes(buf);

    let mut buf = vec![0; len as _];
    reader.read_exact(&mut buf).await?;
    String::from_utf8(buf).map_err(|_| Error::BadEncoding)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_send_recv() {
        let data = String::from("hello");
        let mut buf = Vec::new();

        send_string(&data, &mut buf).unwrap();
        let result = recv_string(&buf[..]).unwrap();
        assert_eq!(data, result);
    }

    #[tokio::test]
    async fn test_send_recv_async_ok() {
        let data = String::from("hello");
        let mut buffer = Vec::new();

        send_string_async(&data, &mut buffer).await.unwrap();

        let mut cursor = Cursor::new(buffer);
        let result = recv_string_async(&mut cursor).await.unwrap();

        assert_eq!(data, result);
    }

    #[tokio::test]
    async fn test_send_string_async_ok() {
        let data = String::from("hello");
        let mut buffer = Vec::new();

        send_string_async(&data, &mut buffer).await.unwrap();

        let len = u32::from_be_bytes(buffer[..4].try_into().unwrap());
        let string_data = String::from_utf8(buffer[4..].to_vec()).unwrap();

        assert_eq!(data, string_data);
        assert_eq!(len, 5);
    }

    #[tokio::test]
    async fn test_recv_string_async_ok() {
        let data = String::from("hello");
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&5_u32.to_be_bytes());
        buffer.extend_from_slice(data.as_bytes());

        let mut cursor = Cursor::new(buffer);
        let received = recv_string_async(&mut cursor).await.unwrap();
        assert_eq!(data, received);
    }
}
