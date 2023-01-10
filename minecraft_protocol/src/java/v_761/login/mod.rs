use std::io::{Read, Write};

pub use client_bound::PacketIOImpl as ClientIO;
pub use server_bound::PacketIOImpl as ServerIO;

use crate::java::v_761::login::encryption::ServerBoundEncryptionResponseImpl;
use crate::java::v_761::login::plugin::ServerBoundPluginResponseImpl;
use crate::java::v_761::login::server_bound_login_start::ServerBoundLoginStartImpl;
use crate::packets::login::ServerBoundLogin;
use crate::Packet;

pub mod encryption;
pub mod other;
pub mod plugin;
pub mod server_bound_login_start;

mod server_bound {
    use minecraft_protocol_macros::define_io;

    use crate::PacketIO;
    use crate::{PacketReadError, PacketWriteError};

    use super::*;

    define_io!(ServerBoundLogin {
        0x00 => {
            type_name: ServerBoundLoginStartImpl
            g_var: LoginStart
        },
        0x01 => {
            type_name: ServerBoundEncryptionResponseImpl
            g_var: EncryptionResponse
        },
        0x02 => {
            type_name: ServerBoundPluginResponseImpl
            g_var: PluginResponse
        }
    });
}

mod client_bound {
    use minecraft_protocol_macros::define_io;

    use crate::java::v_761::login::encryption::ClientBoundEncryptionRequestImpl;
    use crate::java::v_761::login::other::{
        ClientBoundDisconnectImpl, ClientBoundLoginSuccessImpl, ClientBoundSetCompressionImpl,
    };
    use crate::java::v_761::login::plugin::ClientBoundPluginImpl;
    use crate::packets::login::ClientBoundLogin;
    use crate::PacketIO;
    use crate::{PacketReadError, PacketWriteError};

    use super::*;

    define_io!(ClientBoundLogin {
       0x00 => {
            type_name: ClientBoundDisconnectImpl
            g_var: LoginDisconnect
        },
        0x01 => {
            type_name: ClientBoundEncryptionRequestImpl
            g_var: EncryptionRequest
        },
        0x02 => {
            type_name: ClientBoundLoginSuccessImpl
            g_var: LoginSuccess
        },
        0x03 => {
            type_name: ClientBoundSetCompressionImpl
            g_var: SetCompression
        },
        0x04 => {
            type_name: ClientBoundPluginImpl
            g_var: LoginPluginRequest
        }
    });
}
