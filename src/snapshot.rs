// src/snapshot.rs
use crate::fdtd::FDTDSim;
use std::fs;
use std::io::prelude::*;
use std::time::SystemTime;

pub fn create_output_dir() -> std::io::Result<String> {
    let fdir = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(t) => format!("snapshots{}", t.as_secs()),
        Err(_) => panic!("Unable to capture time since UNIX epoch."),
    };

    fs::create_dir(&fdir)?;
    Ok(fdir)
}

pub fn write(sim: &FDTDSim, fdir: &String, iteration: usize) -> std::io::Result<()> {
    let mut file = fs::File::create(format!("{}/{}.json", fdir, iteration))?;
    let serialized = serde_json::to_string(sim).unwrap();
    file.write(serialized.as_bytes())?;
    Ok(())
}
