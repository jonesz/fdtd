// src/fdtd.rs
//! Referenced from "Understanding the Finite-Difference Time-Domain Method"
//! by John. B Schneider; https://eecs.wsu.edu/~schneidj/ufdtd/ufdtd.pdf.

use serde::Serialize;

/// Characteristic impedance of free space.
pub const IMP0: f64 = 377.0;

#[derive(Serialize)]
pub struct FDTDSim {
    q_time: f64,
    sz: usize,
    ez: Vec<f64>,
    hy: Vec<f64>,
}

impl FDTDSim {
    pub fn new(sz: usize) -> Self {
        let mut ez = Vec::new();
        let mut hy = Vec::new();
        ez.resize(sz, 0.0);
        hy.resize(sz, 0.0);

        FDTDSim {
            q_time: 0.0,
            sz,
            ez,
            hy,
        }
    }

    fn abc_magnetic(&mut self) {
        self.hy[self.sz - 1] = self.hy[self.sz - 2];
    }

    fn abc_electric(&mut self) {
        self.ez[0] = self.ez[1];
    }

    fn update_magnetic(&mut self) {
        for mm in 0..self.sz - 1 {
            self.hy[mm] = self.hy[mm] + (self.ez[mm + 1] - self.ez[mm]) / IMP0;
        }
    }

    fn update_electric(&mut self) {
        for mm in 1..self.sz {
            self.ez[mm] = self.ez[mm] + (self.hy[mm] - self.hy[mm - 1]) * IMP0;
        }
    }

    fn _hardwired_source(&mut self) {
        self.ez[0] = (-(self.q_time - 30.0) * (self.q_time - 30.0) / 100.0).exp();
    }

    fn additive_source(&mut self) {
        self.ez[50] += (-(self.q_time - 30.0) * (self.q_time - 30.0) / 100.0).exp();
    }

    pub fn step(&mut self) {
        self.abc_magnetic();
        self.update_magnetic();

        self.abc_electric();
        self.update_electric();

        self.additive_source();
        self.q_time += 1.0;
    }

    pub fn ez50(&self) -> f64 {
        self.ez[50]
    }

    pub fn ez_dump(&self) -> &[f64] {
        self.ez.as_slice()
    }
}
