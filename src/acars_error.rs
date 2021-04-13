use core::fmt;
use std::fmt::Formatter;
use std::error::Error;

#[derive(Debug)]
pub struct AcarsError {
    pub message: String
}

impl fmt::Display for AcarsError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
       write!(f, "Acars Error: {}", self.message)
    }
}

impl Error for AcarsError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self)
    }
}
