// src/error.rs
use std::fmt;

pub enum FDTDError {
    LengthMismatch,
}

impl fmt::Display for FDTDError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FDTDError::LengthMismatch => write!(f, "LengthMismatch"),
        }
    }
}
