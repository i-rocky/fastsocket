use std::fmt::Write;
use crate::app::App;

#[derive(Debug)]
struct UserData<'ud> {
    socket_id: String,
    public_key: String,
    app: &'ud App,
}

impl<'ud> UserData<'ud> {
    #[inline]
    fn new(app: &'ud App) -> Self {
        UserData {
            socket_id: Self::generate_unique_socket_id(),
            public_key: String::with_capacity(64),
            app,
        }
    }

    #[inline]
    fn set_public_key(&mut self, public_key: String) {
        self.public_key = public_key;
    }

    fn generate_unique_socket_id() -> String {
        let mut buffer = String::with_capacity(40);
        write!(&mut buffer, "{:016x}:{:016x}",
               fastrand::u64(..),
               fastrand::u64(..)
        ).unwrap();
        buffer
    }

    #[inline]
    pub fn get_socket_id(&self) -> &str {
        &self.socket_id
    }

    pub fn encrypt_message(&self, message: String) -> *const str {
        // TODO: encrypt message
        message.as_str()
    }
}
