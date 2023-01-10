use crate::player::Player;
use crate::server::Server;
use std::error::Error;

pub struct CommandError(Box<dyn Error>);
pub trait Command<S: Server> {
    type Error: Error + Send + Sync + 'static;
    type CommandSuggestions: CommandSuggestions<S>;
    fn execute(
        &self,
        server: &S,
        player: &S::Player,
        command: &str,
        arguments: Vec<String>,
    ) -> Result<(), Self::Error>;
}

pub trait CommandSuggestions<S: Server> {
    type Error: Error + Send + Sync + 'static;
    fn get_suggestions(
        &self,
        server: &S,
        player: &S::Player,
        command: &str,
        arguments: Vec<String>,
    ) -> Result<Vec<String>, Self::Error>;
}
