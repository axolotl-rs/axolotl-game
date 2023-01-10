use std::str::FromStr;

use log::info;
use simple_log::LogConfigBuilder;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use uuid::Uuid;

use minecraft_protocol::data::var_int::VarInt;
use minecraft_protocol::java::handshake::{HandShake, HandShakeIO, NextState};
use minecraft_protocol::java::v_761::login::ServerIO;
use minecraft_protocol::java::v_761::play::ClientIO;
use minecraft_protocol::java::v_761::{login, play};
use minecraft_protocol::packets::login::server_bound::ServerBoundLoginStart;
use minecraft_protocol::packets::login::ClientBoundLogin;
use minecraft_protocol::packets::play::client::ClientBoundPlay;
use minecraft_protocol::packets::play::server::{ConfirmTeleport, ServerBoundPlay};
use minecraft_protocol::simple_handlers::{NonEncryptedPacketReader, NonEncryptedPacketWriter};
use minecraft_protocol::{PacketReadError, PacketReader, PacketWriter};

#[tokio::test]
pub async fn test() -> anyhow::Result<()> {
    simple_log::new(
        LogConfigBuilder::builder()
            .level("debug")
            .output_console()
            .build(),
    )
    .expect("Failed to initialize logger");
    info!("Starting test");

    let mut socket = TcpStream::connect(option_env!("IP").unwrap_or("127.0.0.1:25565")).await?;
    {
        let handshake = HandShake {
            protocol_version: VarInt(760),
            server_address: "127.0.0.1".to_string(),
            server_port: 25565,
            next_state: NextState::Login,
        };
        info!("Sending handshake");
        let mut handler = NonEncryptedPacketWriter::<HandShakeIO>::default();
        handler.write_packet(handshake)?;
        socket.write_all(&handler.get_buffer()).await?;
    }
    // Send LoginStart
    {
        let mut handler = NonEncryptedPacketWriter::<ServerIO>::default();
        handler.write_packet(ServerBoundLoginStart {
            name: "KingTux".to_string(),
            uuid: Uuid::from_str("d087006b-d72c-4cdf-924d-6f903704d05c").ok(),
        })?;
        socket.write_all(&handler.get_buffer()).await?;
    }
    let mut login_success = None;
    while login_success.is_none() {
        // Read LoginSuccess
        let mut handler = NonEncryptedPacketReader::<login::ClientIO>::default();
        socket.read_buf(handler.get_read_buffer()).await?;

        let mut data = handler.attempt_packet_read()?;
        while data.is_none() {
            socket.read_buf(handler.get_read_buffer()).await?;
            data = handler.attempt_packet_read()?;
        }
        if let Some(data) = data {
            match data {
                ClientBoundLogin::LoginDisconnect(_) => {}
                ClientBoundLogin::EncryptionRequest(_) => {}
                ClientBoundLogin::LoginSuccess(ok) => {
                    login_success = Some(ok);
                }
                ClientBoundLogin::SetCompression(_) => {}
                ClientBoundLogin::LoginPluginRequest(_) => {}
            }
        }
    }
    let login_success = login_success.unwrap();
    info!("Login Success: {:?}", login_success);
    let mut writer = NonEncryptedPacketWriter::<play::ServerIO>::default();
    let mut handler = NonEncryptedPacketReader::<ClientIO>::default();
    let mut packet = read_next_packet(&mut socket, &mut handler).await;
    loop {
        if let Ok(packet) = packet {
            match packet {
                ClientBoundPlay::Login(_) => {}
                ClientBoundPlay::Disconnect(_) => {}
                ClientBoundPlay::ServerData(_) => {}
                ClientBoundPlay::PluginMessage(_) => {}
                ClientBoundPlay::Abilities(_) => {}
                ClientBoundPlay::ChangeDifficulty(_) => {}
                ClientBoundPlay::KeepAlive(keep) => {
                    write_packet(&mut socket, &mut writer, ServerBoundPlay::KeepAlive(keep))
                        .await?;
                }
                ClientBoundPlay::Ping(ping) => {
                    write_packet(&mut socket, &mut writer, ServerBoundPlay::Ping(ping)).await?;
                }
                ClientBoundPlay::SyncPlayerPosition(sync) => {
                    write_packet(
                        &mut socket,
                        &mut writer,
                        ServerBoundPlay::ConfirmTeleport(ConfirmTeleport(sync.teleport_id)),
                    )
                    .await?;
                }
                ClientBoundPlay::PlayerInfo(_) => {}
                ClientBoundPlay::ChunkData(data) => {
                    info!("Chunk Data: {} {}", data.chunk_x, data.chunk_z);
                }
                ClientBoundPlay::UpdateLight(_) => {}
            }
        }

        packet = read_next_packet(&mut socket, &mut handler).await;
    }
}
async fn write_packet(
    socket: &mut TcpStream,
    handler: &mut NonEncryptedPacketWriter<play::ServerIO>,
    packet: ServerBoundPlay,
) -> anyhow::Result<()> {
    handler.write_packet(packet)?;
    socket.write_all(&handler.get_buffer()).await?;
    socket.flush().await?;
    handler.force_buffer_clear();
    Ok(())
}
async fn read_next_packet(
    socket: &mut TcpStream,
    handler: &mut NonEncryptedPacketReader<ClientIO>,
) -> Result<ClientBoundPlay, PacketReadError> {
    socket
        .read_buf(handler.get_read_buffer())
        .await
        .expect("Failed to read");

    let mut data = handler.attempt_packet_read()?;
    while data.is_none() {
        socket
            .read_buf(handler.get_read_buffer())
            .await
            .expect("Failed to read");
        data = handler.attempt_packet_read()?;
    }
    let data = data.unwrap();
    Ok(data)
}
