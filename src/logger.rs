pub struct Log {
}

impl Log {
    #[inline]
    pub fn log(message: &str) {
        println!("{}", message);
    }

    #[inline]
    pub fn debug(message: &str) {
        println!("{}", message);
    }

    #[inline]
    pub fn info(message: &str) {
        println!("{}", message);
    }

    #[inline]
    pub fn warn(message: &str) {
        println!("{}", message);
    }

    #[inline]
    pub fn error(message: &str) {
        println!("{}", message);
    }
}
