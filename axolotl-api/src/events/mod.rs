use std::error::Error;
use std::fmt::Display;

pub trait Event {
    fn get_name() -> &'static str;
}

pub trait EventHandler<E: Event> {
    type Error: Error;
    fn handle(&self, event: &mut E) -> Result<(), Self::Error>;
}

#[derive(Debug)]
pub struct NoError;

impl Display for NoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No error")
    }
}

impl Error for NoError {}
