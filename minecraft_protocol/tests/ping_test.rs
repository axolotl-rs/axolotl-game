extern crate core;

use log::{info, warn};
use serde_json::Value;
use simple_log::LogConfigBuilder;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use minecraft_protocol::data::var_int::VarInt;
use minecraft_protocol::java::handshake::{HandShake, HandShakeIO, NextState};
use minecraft_protocol::java::status::{
    ClientBoundStatusIO, ClientBoundStatusPacket, ServerBoundStatusIO, Status, StatusOrString,
};
use minecraft_protocol::simple_handlers::{NonEncryptedPacketReader, NonEncryptedPacketWriter};
use minecraft_protocol::{PacketLength, PacketReader, PacketWriter};

#[tokio::test]
pub async fn ping() -> anyhow::Result<()> {
    simple_log::new(
        LogConfigBuilder::builder()
            .level("debug")
            .output_console()
            .build(),
    )
    .expect("Failed to initialize logger");
    info!("Starting test");

    let mut socket = TcpStream::connect(option_env!("IP").unwrap_or("127.0.0.1:25565")).await?;
    let handshake = HandShake {
        protocol_version: VarInt(760),
        server_address: "mc.hypixel.net".to_string(),
        server_port: 25565,
        next_state: NextState::Status,
    };
    info!("Sending handshake");
    let now = std::time::Instant::now();
    {
        let mut handler = NonEncryptedPacketWriter::<HandShakeIO>::default();
        handler.write_packet(handshake)?;
        socket.write_all(&handler.get_buffer()).await?;
    }
    info!("Sent handshake");
    info!("Sending status request");
    {
        let mut status = NonEncryptedPacketWriter::<ServerBoundStatusIO>::default();
        status.write_packet(minecraft_protocol::java::status::ServerBoundStatusPacket::Request)?;
        socket.write_all(&status.get_buffer()).await?;
    }
    info!("Sent status request");

    let mut status = NonEncryptedPacketReader::<ClientBoundStatusIO>::default();
    socket.read_buf(status.get_read_buffer()).await?;
    let mut data = status.attempt_packet_read()?;
    while data.is_none() {
        socket.read_buf(status.get_read_buffer()).await?;
        data = status.attempt_packet_read()?;
    }
    let data = data.unwrap();
    match data {
        ClientBoundStatusPacket::Response(v) => match v {
            StatusOrString::Status(status) => {
                info!("{:?}", status.description);
            }
            StatusOrString::JsonString(c) => {
                warn!("Got json string");
                let v: serde_json::Result<Status<Value>> = serde_json::from_str(&c);
                if let Err(e) = v {
                    warn!("Failed to parse json: {}", e);
                }
            }
        },
        ClientBoundStatusPacket::Ping(_wr) => {
            info!("Failed to get response");
        }
    }
    assert_eq!(status.packet_len, PacketLength::Incomplete);
    assert_eq!(status.get_read_buffer().len(), 0);
    info!("Done in {}", now.elapsed().as_millis());
    Ok(())
}
