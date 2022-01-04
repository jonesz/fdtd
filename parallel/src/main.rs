// src/main.rs
use genfut::{genfut, Opt};

fn main() {
    genfut(Opt {
        name: "fdtd-futhark".to_string(),
        file: std::path::PathBuf::from("lib/fdtd/step.fut"),
        author: "Ethan Jones <etn.jones@gmail.com>".to_string(),
        version: "0.1.0".to_string(),
        license: "".to_string(),
        description: "FDTD in Futhark.".to_string(),
    })
}
