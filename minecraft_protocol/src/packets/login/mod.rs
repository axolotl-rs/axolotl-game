use serde::{Deserialize, Serialize};

use crate::packets::define_group;
use crate::PacketContent;

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct SigData {
    pub timestamp: i64,
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub enum VerifyMethod {
    VerifyToken { token: Vec<u8> },
    MessageSignature { salt: i64, message: Vec<u8> },
}

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

define_group!(ServerBoundLogin {
    LoginStart: server_bound::ServerBoundLoginStart,
    EncryptionResponse: server_bound::ServerBoundEncryptionResponse,
    PluginResponse: server_bound::ServerBoundLoginPluginResponse
});

pub mod server_bound {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use crate::data::var_int::VarInt;

    use crate::PacketContent;

    /// First Packet sent after the handshake
    #[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
    pub struct ServerBoundLoginStart {
        // The username of the player no longer than 16 characters
        pub name: String,
        // A true or false is written before the option to indicate if it is present
        pub uuid: Option<Uuid>,
    }

    impl PacketContent for ServerBoundLoginStart {}

    /// Encryption Response
    #[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
    pub struct ServerBoundEncryptionResponse {
        pub shared_secret: Vec<u8>,
        pub verify_token: Vec<u8>,
    }

    impl PacketContent for ServerBoundEncryptionResponse {}

    #[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
    pub struct ServerBoundLoginPluginResponse {
        pub message_id: VarInt,
        pub successful: bool,
        pub data: Vec<u8>,
    }

    impl PacketContent for ServerBoundLoginPluginResponse {}
}
define_group!(ClientBoundLogin {
    LoginDisconnect: client_bound::Disconnect,
    EncryptionRequest: client_bound::ClientBoundEncryptionRequest,
    LoginSuccess: client_bound::LoginSuccess,
    SetCompression: client_bound::SetCompression,
    LoginPluginRequest: client_bound::ClientBoundPluginRequest
});

pub mod client_bound {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    use crate::data::var_int::VarInt;
    use crate::PacketContent;

    /// First Packet sent after the handshake
    #[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
    pub struct Disconnect {
        // Implied that it is a chat message
        pub reason: String,
    }

    impl PacketContent for Disconnect {}

    /// Encryption Response
    #[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
    pub struct ClientBoundEncryptionRequest {
        pub server_id: String,
        pub public_key: Vec<u8>,
        pub verify_token: Vec<u8>,
    }

    impl PacketContent for ClientBoundEncryptionRequest {}

    #[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
    pub struct LoginSuccess {
        pub uuid: Uuid,
        pub username: String,
        pub properties: Vec<super::Property>,
    }

    impl PacketContent for LoginSuccess {}

    #[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
    pub struct SetCompression(pub(crate) VarInt);

    impl PacketContent for SetCompression {}

    #[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
    pub struct ClientBoundPluginRequest {
        pub message_id: VarInt,
        pub channel: String,
        pub data: Vec<u8>,
    }

    impl PacketContent for ClientBoundPluginRequest {}
}
