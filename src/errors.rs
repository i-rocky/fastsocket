use thiserror::Error;

#[derive(Error, Debug)]
pub enum FastSocketError {
    #[error("Invalid AppId provided")]
    InvalidAppIdError,

    #[error("Invalid AppKey provided")]
    InvalidAppKeyError,

    #[error("Invalid AppSecret provided")]
    InvalidAppSecretError,

    #[error("Invalid message provided")]
    InvalidMessageError,

    #[error("Invalid signature provided")]
    InvalidSignatureError,

    #[error("Invalid AppName provided")]
    InvalidAppNameError,

    #[error("Invalid AppHost provided")]
    InvalidAppHostError,

    #[error("Invalid AppPath provided")]
    InvalidAppPathError,

    #[error("Invalid AppCapacity provided")]
    InvalidAppCapacityError,

    #[error("Invalid Payload provided")]
    InvalidPayloadError,

    #[error("Failed to send pong")]
    FailedToSendPongError,

    #[error("Failed to send payload")]
    FailedToSendPayloadError,

    #[error("Encryption error")]
    EncryptionError,

    #[error("Server is at capacity")]
    ServerCapacityError,

    #[error("Invalid App provided")]
    InvalidAppError,

    #[error("Upgrade failed")]
    UpgradeFailedError,

    #[error("Connection closed")]
    ConnectionClosed,

    #[error("Error handling message")]
    ErrorReadingPayload,

    #[error("Error decoding payload")]
    ErrorDecodingPayload,

    #[error("Error creating message")]
    ErrorCreatingMessage,

    #[error("Error sending pong")]
    ErrorSendingPong,

    #[error("Error handling message")]
    ErrorHandlingMessage,
}
