use std::future::Future;
use std::pin::Pin;
use crate::errors::FastSocketError;

pub trait Message {
    fn respond<'a>(&'a mut self) -> Pin<Box<dyn Future<Output=Result<(), FastSocketError>> + Send + 'a>>;
}
