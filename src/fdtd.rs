// src/fdtd.rs
//! Referenced from "Understanding the Finite-Difference Time-Domain Method"
//! by John. B Schneider; https://eecs.wsu.edu/~schneidj/ufdtd/ufdtd.pdf.
use crate::error;
use serde::Serialize;

/// Characteristic impedance of free space.
pub const IMP0: f64 = 377.0;

#[derive(Serialize)]
pub struct Grid {
    // Grid components.
    sz: usize,

    ez: Vec<f64>,
    ceze: Vec<f64>,
    cezh: Vec<f64>,

    hy: Vec<f64>,
    chyh: Vec<f64>,
    chye: Vec<f64>,

    cdtds: f64, // Courant number.
}

impl Grid {
    fn update_magnetic(&mut self) {
        for mm in 0..self.sz {
            self.hy[mm] =
                self.chyh[mm] * self.hy[mm] + self.chye[mm] * (self.ez[mm + 1] - self.ez[mm]);
        }
    }

    fn update_electric(&mut self) {
        for mm in 1..self.sz {
            self.ez[mm] =
                self.ceze[mm] + self.ez[mm] + self.cezh[mm] * (self.hy[mm] - self.hy[mm - 1]);
        }
    }
}

// Default functions that the compiler can hopefully optimize into NOPs.
fn tfsf_default(_g: &mut Grid) {
    return;
}

fn abc_default(_g: &mut Grid) {
    return;
}

fn source_default(_g: &mut Grid) {
    return;
}

fn snapshot_default(_t: usize, _g: &mut Grid) {
    return;
}

pub struct FDTDSim {
    g: Grid,
    abc: fn(&mut Grid),
    tfsf: fn(&mut Grid),
    source: fn(&mut Grid),
    snapshot: fn(usize, &mut Grid),

    time: usize,
}

impl FDTDSim {
    /// Create a new FDTDSimulation with pre-computed parameters:
    pub fn new_opts(
        sz: usize,
        ez: Option<Vec<f64>>,
        ceze: Option<Vec<f64>>,
        cezh: Option<Vec<f64>>,
        hy: Option<Vec<f64>>,
        chyh: Option<Vec<f64>>,
        chye: Option<Vec<f64>>,
        cdtds: Option<f64>,
    ) -> Result<Self, error::FDTDError> {
        // TODO: Does Vec have a function that does this already?
        let vec_of_sz = |sz: usize, v: f64| -> Vec<f64> {
            let mut r = Vec::with_capacity(sz);
            r.resize(sz, v);
            r
        };

        // TODO: Defaults for these?
        let ez = ez.unwrap_or(vec_of_sz(sz, 0.0));
        let ceze = ceze.unwrap_or(vec_of_sz(sz, 1.0));
        let cezh = cezh.unwrap_or(vec_of_sz(sz, IMP0));

        let hy = hy.unwrap_or(vec_of_sz(sz, 0.0));
        let chyh = chyh.unwrap_or(vec_of_sz(sz, 1.0));
        let chye = chye.unwrap_or(vec_of_sz(sz, 1.0 / IMP0));

        let cdtds = cdtds.unwrap_or(1.0);

        // Assert that all passed vectors are of the same size.
        let all_sz = ez.len();
        if ceze.len() != all_sz
            || cezh.len() != all_sz
            || hy.len() != all_sz
            || chyh.len() != all_sz
            || chye.len() != all_sz
        {
            Err(error::FDTDError::LengthMismatch)
        } else {
            let g = Grid {
                sz,
                ez,
                ceze,
                cezh,
                hy,
                chyh,
                chye,
                cdtds,
            };
            Ok(FDTDSim {
                g,
                abc: abc_default,
                tfsf: tfsf_default,
                source: source_default,
                snapshot: snapshot_default,
                time: 0,
            })
        }
    }

    pub fn new(sz: usize) -> Result<Self, error::FDTDError> {
        FDTDSim::new_opts(sz, None, None, None, None, None, None, None)
    }

    pub fn abc_set(&mut self, abc: Option<fn(&mut Grid)>) {
        self.abc = abc.unwrap_or(abc_default);
    }

    pub fn tfsf_set(&mut self, tfsf: Option<fn(&mut Grid)>) {
        self.tfsf = tfsf.unwrap_or(tfsf_default);
    }

    pub fn source_set(&mut self, source: Option<fn(&mut Grid)>) {
        self.source = source.unwrap_or(source_default);
    }

    pub fn snapshot_set(&mut self, snapshot: Option<fn(usize, &mut Grid)>) {
        self.snapshot = snapshot.unwrap_or(snapshot_default);
    }

    pub fn step(&mut self) {
        self.g.update_magnetic();

        let tfsf_fn = self.tfsf;
        tfsf_fn(&mut self.g);

        let abc_fn = self.abc;
        abc_fn(&mut self.g);

        self.g.update_electric();

        self.time += 1;

        let snapshot_fn = self.snapshot;
        snapshot_fn(self.time, &mut self.g);
    }
}
