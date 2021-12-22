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

    // TODO: Rather than expose these as public, provide getter/setter
    // functions?
    pub ez: Vec<f64>,
    pub ceze: Vec<f64>,
    pub cezh: Vec<f64>,
    pub hy: Vec<f64>,
    pub chyh: Vec<f64>,
    pub chye: Vec<f64>,

    cdtds: f64, // Courant number.
}

impl Grid {
    fn update_magnetic(&mut self) {
        for mm in 0..self.sz - 1 {
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

pub struct FDTDSim<A, B>
where
    A: Fn(usize, &mut Grid), // post-magnetic update.
    B: Fn(usize, &mut Grid), // post-electric update.
{
    g: Grid,

    // TODO: There's multiple ways to do this: function pointers, closures,
    // boxed closures, etc. What's the most performant/flexible?
    post_magnetic: Option<A>,
    post_electric: Option<B>,
    time: usize,
}

impl<A, B> FDTDSim<A, B>
where
    A: Fn(usize, &mut Grid),
    B: Fn(usize, &mut Grid),
{
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
                post_magnetic: None,
                post_electric: None,
                time: 0,
            })
        }
    }

    pub fn new(sz: usize) -> Result<Self, error::FDTDError> {
        FDTDSim::new_opts(sz, None, None, None, None, None, None, None)
    }

    pub fn set_post_magnetic(&mut self, f: Option<A>) {
        self.post_magnetic = f;
    }

    pub fn set_post_electric(&mut self, f: Option<B>) {
        self.post_electric = f;
    }

    pub fn step(&mut self) {
        self.time += 1;
        self.g.update_magnetic();

        match &self.post_magnetic {
            Some(v) => v(self.time, &mut self.g),
            None => (),
        }

        self.g.update_electric();

        match &self.post_electric {
            Some(v) => v(self.time, &mut self.g),
            None => (),
        }
    }
}
