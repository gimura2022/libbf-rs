use std::fmt::Display;

use colored::Colorize;

pub trait BfError {
    fn description(&self) -> String;
    fn error_type(&self) -> String;
}

pub struct BfErrorable<T: BfError>(pub T);

impl<T: BfError> Display for BfErrorable<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.0.error_type().bright_red(),
            "error:".bright_red(),
            self.0.description().bright_white()
        )
    }
}
