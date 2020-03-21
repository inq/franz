use serde::Serialize;
use tokio::net::TcpStream;

pub struct Client {
    stream: TcpStream,
    codec: bincode::Config,
}

impl Client {
    pub async fn connect(addr: &str) -> Result<Self, failure::Error> {
        use std::net::SocketAddr;

        let codec = {
            let mut res = bincode::config();
            res.big_endian();
            res
        };

        let stream = TcpStream::connect(addr.parse::<SocketAddr>()?).await?;
        Ok(Self { stream, codec })
    }

    async fn send<T: Serialize>(&mut self, request: T) -> Result<(), failure::Error> {
        use crate::protocol::Size;
        use tokio::io::AsyncWriteExt;

        let request_buffer = self.codec.serialize(&request)?;
        let size = Size::new(request_buffer.len() as i32);
        let size_buffer = self.codec.serialize(&size)?;
        self.stream.write_all(&size_buffer).await?;
        self.stream.write_all(&request_buffer).await?;
        Ok(())
    }

    pub async fn send_api_version_request(&mut self) -> Result<(), failure::Error> {
        use crate::protocol::request::RequestHeaderV0;

        let request = RequestHeaderV0::api_version_request_v0();
        self.send(request).await
    }

    pub async fn wip_recv(&mut self) -> Result<(), failure::Error> {
        use crate::protocol::Size;
        use tokio::io::AsyncReadExt;

        let mut size_buffer = [0u8; std::mem::size_of::<Size>()];
        self.stream.read_exact(&mut size_buffer).await.unwrap();
        let size: Size = self.codec.deserialize(&size_buffer)?;

        let mut response_buffer = Vec::with_capacity(size.0 as usize);
        unsafe { response_buffer.set_len(size.0 as usize) };
        self.stream
            .read_exact(response_buffer.as_mut_slice())
            .await
            .unwrap();

        Ok(())
    }
}
