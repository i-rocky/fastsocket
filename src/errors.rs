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
}
