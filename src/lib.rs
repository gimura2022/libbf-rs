use std::{fmt::Display, vec::Vec};

pub mod interprer;
pub mod parser;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Command {
    Plus,
    Minus,

    MoveLeft,
    MoveRight,

    Get,
    Put,

    Loop(Vec<Command>),
}

pub struct Commands(Vec<Command>);

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Command::Plus => "+".to_string(),
                Command::Minus => "-".to_string(),

                Command::MoveLeft => "<".to_string(),
                Command::MoveRight => ">".to_string(),

                Command::Get => ",".to_string(),
                Command::Put => ".".to_string(),

                Command::Loop(cmds) => format!(
                    "[{}]",
                    cmds.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join("")
                ),
            }
            .as_str()
        )
    }
}

impl Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("")
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::parse;

    #[test]
    fn to_string_test() {
        assert_eq!(
            "++--[+-<<>>[-]]",
            parse("++--[+-<<>>[-]]").unwrap().to_string()
        );
    }
}
