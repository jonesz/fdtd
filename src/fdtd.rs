// src/fdtd.rs
/// Referenced from "Understanding the Finite-Difference Time-Domain Method"
/// by John. B Schneider; https://eecs.wsu.edu/~schneidj/ufdtd/ufdtd.pdf.

/// Characteristic impedance of free space.
pub const IMP0: f64 = 377.0;

pub struct FDTDSim {
    qTime: f64,
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
            qTime: 0.0,
            sz,
            ez,
            hy,
        }
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

    fn source(&mut self) {
        self.ez[0] = (-(self.qTime - 30.0) * (self.qTime - 30.0) / 100.0).exp();
    }

    pub fn step(&mut self) {
        self.update_magnetic();
        self.update_electric();

        self.source();
        self.qTime += 1.0;
    }

    pub fn ez50(&self) -> f64 {
        self.ez[50]
    }
}
