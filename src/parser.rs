use std::str::Chars;

use crate::{Command, Commands, error::BfError};

#[derive(Debug, Clone)]
pub enum ParseError {
    LoopStartNotFound,
    LoopEndNotFound,
    UndefinedCommand(char),
}

impl BfError for ParseError {
    #[inline(always)]
    fn error_type(&self) -> String {
        "Parse".to_string()
    }

    fn description(&self) -> String {
        match self {
            ParseError::UndefinedCommand(c) => format!("undefined command '{}'", c),
            ParseError::LoopStartNotFound => "'[' not found".to_string(),
            ParseError::LoopEndNotFound => "']' not found".to_string(),
        }
    }
}

pub fn parse(s: &str) -> Result<Commands, ParseError> {
    let mut res = Vec::<Command>::new();
    let mut it = s.chars();
    let mut c = it.next();

    while c.is_some() {
        res.push(parse_command(&mut it, c.unwrap())?);
        c = it.next();
    }

    Ok(Commands(res))
}

fn parse_command(it: &mut Chars, c: char) -> Result<Command, ParseError> {
    match c {
        '+' => Ok(Command::Plus),
        '-' => Ok(Command::Minus),

        '<' => Ok(Command::MoveLeft),
        '>' => Ok(Command::MoveRight),

        '.' => Ok(Command::Put),
        ',' => Ok(Command::Get),

        '[' => parse_loop(it),
        ']' => Err(ParseError::LoopStartNotFound),

        _ => Err(ParseError::UndefinedCommand(c)),
    }
}

fn parse_loop(it: &mut Chars) -> Result<Command, ParseError> {
    let mut res = Vec::<Command>::new();
    let mut c = it.next();

    while c.is_some() && c.is_some_and(|x| x != ']') {
        res.push(parse_command(it, c.unwrap())?);
        c = it.next();
    }

    if c.is_none() {
        Err(ParseError::LoopEndNotFound)
    } else {
        Ok(Command::Loop(res))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_test() {
        assert_eq!(vec![Command::Plus, Command::Put], parse("+.").unwrap().0);
        assert_eq!(
            vec![
                Command::Plus,
                Command::Put,
                Command::Loop(vec![Command::Plus])
            ],
            parse("+.[+]").unwrap().0
        );
    }
}
