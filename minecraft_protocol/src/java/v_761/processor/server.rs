use aes::cipher::KeyIvInit;
use log::{debug, error, warn};
use num_bigint::BigInt;
use rand::random;
use rsa::{PaddingScheme, RsaPrivateKey};
use serde::Deserialize;
use sha1::{Digest, Sha1};
use thiserror::Error;

use crate::packets::login::client_bound::ClientBoundEncryptionRequest;
use crate::packets::login::{ClientBoundLogin, Property, ServerBoundLogin, SigData, VerifyMethod};
use crate::{AsyncStreamCipher, Decryptor, Encryptor};

#[derive(Debug, Error)]
pub enum LoginError {
    #[error("Failed to parse login packet")]
    MissingSigData,
    #[error("Failed to access Minecraft profile {0}")]
    MinecraftLogin(#[from] reqwest::Error),
    #[error("{0}")]
    Other(String),
    #[error("Internal error. Please Report this to the developer")]
    InvalidLength,
    #[error("Json Error: {0}")]
    JsonError(#[from] serde_json::Error),
}

#[derive(Debug, Clone)]
pub struct ServerClient {
    pub server_id: String,
    pub reqwest_client: reqwest::Client,
    pub key: RsaPrivateKey,
    pub key_encoded: Vec<u8>,
}

#[derive(Clone, Debug, Default)]
pub enum LoginState {
    #[default]
    Pending,
    EncryptionRequested {
        username: String,
        random_token: [u8; 16],
    },
    Completed {
        encryptor: Encryptor,
        decryptor: Decryptor,
        data: JavaResponse,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct JavaResponse {
    pub id: String,
    pub name: String,
    pub properties: Vec<Property>,
}

pub async fn handle(
    server_client: &ServerClient,
    login: ServerBoundLogin,
    login_state: &mut LoginState,
) -> Result<Option<ClientBoundLogin>, LoginError> {
    match (login, &login_state) {
        (ServerBoundLogin::LoginStart(start), LoginState::Pending) => {
            debug!("Encryption requested");
            let bytes: [u8; 16] = random();
            let packet = ClientBoundEncryptionRequest {
                server_id: server_client.server_id.clone(),
                public_key: server_client.key_encoded.clone(),
                verify_token: Vec::from(bytes),
            };

            *login_state = LoginState::EncryptionRequested {
                username: start.name,
                random_token: bytes,
            };
            return Ok(Some(ClientBoundLogin::EncryptionRequest(packet)));
        }
        (
            ServerBoundLogin::EncryptionResponse(mut enc),
            LoginState::EncryptionRequested {
                username,
                random_token,
            },
        ) => {
            let shared_sec = match server_client
                .key
                .decrypt(PaddingScheme::PKCS1v15Encrypt, &enc.shared_secret)
            {
                Ok(ok) => ok,
                Err(err) => {
                    error!("Failed to decrypt shared secret: {}", err);
                    return Err(LoginError::Other(format!(
                        "Failed to decrypt shared secret: {}",
                        err
                    )));
                }
            };
            let encryptor = Encryptor::new_from_slices(&shared_sec, &shared_sec)
                .map_err(|_| LoginError::InvalidLength)?;
            let decryptor = Decryptor::new_from_slices(&shared_sec, &shared_sec)
                .map_err(|_| LoginError::InvalidLength)?;

            decryptor.clone().decrypt(&mut enc.verify_token);
            if enc.verify_token != *random_token {
                return Err(LoginError::Other(format!(
                    "Got {:?} but expected {:?}",
                    enc.verify_token, random_token
                )));
            }

            let mut hasher = Sha1::default();
            hasher.update(b"");
            hasher.update(&shared_sec.as_slice()[0..16]);
            hasher.update(&*server_client.key_encoded);
            let bigint = BigInt::from_signed_bytes_be(hasher.finalize().as_slice());
            let url = format!(
                "https://sessionserver.mojang.com/session/minecraft/hasJoined?username={}&serverId={:x}",
                username, bigint
            );
            let request = server_client.reqwest_client.get(url).build()?;
            let response1 = server_client.reqwest_client.execute(request).await?;
            let v = response1.text().await?;
            let response = serde_json::from_str::<JavaResponse>(&v)?;

            *login_state = LoginState::Completed {
                encryptor,
                decryptor,
                data: response,
            };
            debug!("Encryption completed");
            return Ok(None);
        }
        (ServerBoundLogin::PluginResponse(plugin), LoginState::Completed { .. }) => {
            warn!(
                "Received plugin response: Message ID: {}",
                plugin.message_id
            );
        }

        (a, b) => {
            error!("Invalid login state: {:?} {:?}", a, b);
        }
    }
    error!("Invalid login state");
    return Err(LoginError::Other("Invalid login state".to_string()));
}
