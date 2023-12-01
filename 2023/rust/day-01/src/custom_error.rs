use miette::Diagnostic;
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
pub enum AocError {
    Generic(String),
    #[error(transparent)]
    #[diagnostic(code(aoc::io_error))]
    IoError(#[from] std::io::Error),
}

impl From<nom::Err<nom::error::Error<&str>>> for AocError {
    fn from(num: nom::Err<nom::error::Error<&str>>) -> Self {
        AocError::Generic(num.to_string())
    }
}
impl std::fmt::Display for AocError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            AocError::Generic(str) => {
                write!(fmt, "My name is AocError::Generic and I'm {} .", str)
            }
            AocError::IoError(_) => todo!(),
        }
    }
}
