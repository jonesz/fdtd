// src/error.rs
use fdtd_futhark::Error;
use std::error;
use std::fmt;

#[derive(Debug)]
enum FDTDError {
    FutharkError(Error),
}

impl fmt::Display for FDTDError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FDTDError::FutharkError(e) => write!(f, "FutharkError: {}", e),
        }
    }
}

impl error::Error for FDTDError {}
