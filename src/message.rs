use std::{fmt::Display, error::Error};

#[derive(Debug, Clone)]
pub struct Message {
    pub text: String,
}
impl Message {
    pub fn new(text: String) -> Self {
        Self { text }
    }
    pub fn text_as_mut(&mut self) -> &mut String {
        &mut self.text
    }
}

pub const COMMAND_PREFIX: char = '.';
#[derive(Debug, Clone, PartialEq)]
pub enum ParseCommandError {
    NotCommand,
    NoHead,
    InvalidHead,
    ExpectedArguments(usize)
}
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Nickname(String),
    Exit
}

impl TryFrom<Message> for Command {
    type Error = ParseCommandError;
    fn try_from(mut value: Message) -> Result<Self, Self::Error> {
        if !value.text.starts_with(COMMAND_PREFIX) {
            return Err(ParseCommandError::NotCommand)
        }
        let cmd_string = value.text.drain(1..).collect::<String>();
        let mut parts = cmd_string.split_ascii_whitespace();
        let Some(head) = parts.next() else {
            return Err(ParseCommandError::NoHead)
        };
        match head {
            "nickname" => {
                let Some(nickname) = parts.next() else {
                    return Err(ParseCommandError::ExpectedArguments(1))
                };
                Ok(Command::Nickname(nickname.to_string()))
            }
            "exit" => {
                Ok(Command::Exit)
            }
            head => Err(ParseCommandError::InvalidHead)
        }
    }
}
impl Display for ParseCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseCommandError::NotCommand => write!(f, "not a command"),
            ParseCommandError::NoHead => write!(f, "no head"),
            ParseCommandError::InvalidHead => write!(f, "invalid head"),
            ParseCommandError::ExpectedArguments(n) => write!(f, "expected {n} argument(s)"),
        }
    }
}
impl Error for ParseCommandError {}