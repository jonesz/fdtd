// src/snapshot.rs
use crate::fdtd::FDTDSim;
use std::fs::File;
use std::io::prelude::*;
use std::time::SystemTime;

pub fn write(sim: &FDTDSim, iteration: usize) -> std::io::Result<()> {
    let fname = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(t) => format!("snapshot{}-{}.json", t.as_secs(), iteration),
        Err(_) => panic!("Unable to capture time since UNIX epoch."),
    };

    let mut file = File::create(fname)?;
    let serialized = serde_json::to_string(sim).unwrap();
    file.write(serialized.as_bytes())?;

    Ok(())
}
