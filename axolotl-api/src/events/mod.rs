use std::error::Error;
use std::fmt::{Debug, Display};
use std::sync::Arc;

pub trait Event {
    type Error: Debug;
    type Result: Debug;

    fn get_name() -> &'static str;
}

pub trait EventHandler<E: Event> {
    fn handle(&self, event: E) -> Result<<E as Event>::Result, <E as Event>::Error>;
}

impl<E: Event, Handler: EventHandler<E>> EventHandler<E> for &'_ Handler {
    fn handle(&self, event: E) -> Result<E::Result, E::Error> {
        (*self).handle(event)
    }
}
impl<E: Event, Handler: EventHandler<E>> EventHandler<E> for Arc<Handler> {
    fn handle(&self, event: E) -> Result<E::Result, E::Error> {
        self.as_ref().handle(event)
    }
}

#[derive(Debug)]
pub struct NoError;

impl Display for NoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "No error")
    }
}

impl Error for NoError {}
