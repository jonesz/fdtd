// src/fdtd.rs
//! Referenced from "Understanding the Finite-Difference Time-Domain Method"
//! by John. B Schneider; https://eecs.wsu.edu/~schneidj/ufdtd/ufdtd.pdf.
use crate::error;
use serde::Serialize;

/// Characteristic impedance of free space.
pub const IMP0: f64 = 377.0;

/// TM^z or TE^z.
#[derive(Copy, Clone)]
pub enum Polarization {
    Magnetic,
    Electric,
}

#[derive(Copy, Clone)]
pub enum GridDimension {
    One,
    Two(Polarization),
    Three,
}

impl Default for GridDimension {
    fn default() -> Self {
        GridDimension::One
    }
}

#[derive(Serialize)]
pub struct Grid {
    // TODO: For 1d, 2d, etc. we don't need all of these vectors; at this
    // point, we allocate what ends up being a 0-length(?) vector on the heap.
    // This could be an Option instead?
    // TODO: Rather than expose these as public, provide getter/setter
    // functions?
    pub x_sz: usize,
    pub y_sz: usize,
    pub z_sz: usize,

    pub hx: Vec<f64>,
    pub chxh: Vec<f64>,
    pub chxe: Vec<f64>,

    pub hy: Vec<f64>,
    pub chyh: Vec<f64>,
    pub chye: Vec<f64>,

    pub hz: Vec<f64>,
    pub chzh: Vec<f64>,
    pub chze: Vec<f64>,

    pub ex: Vec<f64>,
    pub cexe: Vec<f64>,
    pub cexh: Vec<f64>,

    pub ey: Vec<f64>,
    pub ceye: Vec<f64>,
    pub ceyh: Vec<f64>,

    pub ez: Vec<f64>,
    pub ceze: Vec<f64>,
    pub cezh: Vec<f64>,

    cdtds: f64, // Courant number.
}

macro_rules! hx {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.hx[$x * ($grid.y_sz - 1) + $y]
    };
}

macro_rules! chxh {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.chxh[$x * ($grid.y_sz - 1) + $y]
    };
}

macro_rules! chxe {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.chxe[$x * ($grid.y_sz - 1) + $y]
    };
}

macro_rules! hy {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.hy[$x * ($grid.y_sz) + $y]
    };
}

macro_rules! chyh {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.chyh[$x * ($grid.y_sz) + $y]
    };
}

macro_rules! chye {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.chye[$x * ($grid.y_sz) + $y]
    };
}

macro_rules! ez {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.ez[$x * ($grid.y_sz) + $y]
    };
}

macro_rules! ceze {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.ceze[$x * ($grid.y_sz) + $y]
    };
}

macro_rules! cezh {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.cezh[$x * ($grid.y_sz) + $y]
    };
}

impl Grid {
    fn magnetic_1d(&mut self) {
        for mm in 0..self.x_sz - 1 {
            self.hy[mm] =
                self.chyh[mm] * self.hy[mm] + self.chye[mm] * (self.ez[mm + 1] - self.ez[mm]);
        }
    }

    fn magnetic_2d(&mut self) {
        for mm in 0..self.x_sz {
            for nn in 0..self.y_sz - 1 {
                // hx(mm, nn) = chxh(mm, nn) * hx(mm, nn)
                //  - chxe(mm, nn) * (ez(mm, nn + 1) - ez(mm, nn))
                hx!(self, mm, nn) = chxh!(self, mm, nn) * hx!(self, mm, nn)
                    - chxe!(self, mm, nn) * (ez!(self, mm, (nn + 1)) - ez!(self, mm, nn));
            }
        }

        for mm in 0..self.x_sz - 1 {
            for nn in 0..self.y_sz {
                // hy(mm, nn) = chyh(mm, nn) * hy(mm, nn)
                //  + chye(mm, nn) * (ez((mm + 1), nn) - ez(mm, nn))
                hy!(self, mm, nn) = chyh!(self, mm, nn) * hy!(self, mm, nn)
                    + chye!(self, mm, nn) * (ez!(self, (mm + 1), nn) - ez!(self, mm, nn));
            }
        }
    }

    fn update_magnetic(&mut self, d: GridDimension) {
        match d {
            GridDimension::One => self.magnetic_1d(),
            GridDimension::Two(Polarization::Magnetic) => self.magnetic_2d(),
            GridDimension::Two(Polarization::Electric) => panic!("Unimplemented!"),
            GridDimension::Three => panic!("Unimplemented!"),
        }
    }

    fn electric_1d(&mut self) {
        for mm in 1..self.x_sz - 1 {
            self.ez[mm] =
                self.ceze[mm] * self.ez[mm] + self.cezh[mm] * (self.hy[mm] - self.hy[mm - 1]);
        }
    }

    fn electric_2d(&mut self) {
        for mm in 1..self.x_sz - 1 {
            for nn in 1..self.y_sz - 1 {
                // ez(mm, nn) = ceze(mm, nn) * ez(mm, nn)
                //  + cezh(mm, nn)* ((hy(mm, nn) - hy((mm - 1), nn)) -
                //      (hx(mm, nn) - hx(mm, (nn - 1))))
                ez!(self, mm, nn) = ceze!(self, mm, nn) * ez!(self, mm, nn)
                    + cezh!(self, mm, nn)
                        * ((hy!(self, mm, nn) - hy!(self, (mm - 1), nn))
                            - (hx!(self, mm, nn) - hx!(self, mm, (nn - 1))));
            }
        }
    }

    fn update_electric(&mut self, d: GridDimension) {
        match d {
            GridDimension::One => self.electric_1d(),
            GridDimension::Two(Polarization::Magnetic) => self.electric_2d(),
            GridDimension::Two(Polarization::Electric) => panic!("Unimplemented!"),
            GridDimension::Three => panic!("Unimplemented!"),
        }
    }
}

// Helper function.
fn vec_of_sz(sz: usize, v: f64) -> Vec<f64> {
    let mut r = Vec::with_capacity(sz);
    r.resize(sz, v);
    r
}

// TODO: Closures that fit type of A/B must be specified for compilation,
// even if the function is a NOP. This requires the programmer to write a NOP
// function then pass it; is this avoidable?
pub struct FDTDSim<A, B>
where
    A: FnMut(usize, &mut Grid), // post-magnetic update.
    B: FnMut(usize, &mut Grid), // post-electric update.
{
    g: Grid,
    dimension: GridDimension,

    // TODO: There's multiple ways to do this: function pointers, closures,
    // boxed closures, etc. What's the most performant/flexible?
    post_magnetic: Option<A>,
    post_electric: Option<B>,
    time: usize,
}

// TODO: See above note in FDTDSim about A/B specification when we want NOP
// functions.
impl<A, B> FDTDSim<A, B>
where
    A: FnMut(usize, &mut Grid),
    B: FnMut(usize, &mut Grid),
{
    /// Build a new 1d simulation.
    pub fn new_1d(
        x_sz: usize,
        ez: Option<Vec<f64>>,
        ceze: Option<Vec<f64>>,
        cezh: Option<Vec<f64>>,
        hy: Option<Vec<f64>>,
        chyh: Option<Vec<f64>>,
        chye: Option<Vec<f64>>,
        cdtds: Option<f64>,
    ) -> Result<Self, error::FDTDError> {
        // Defaults.
        let ez = ez.unwrap_or(vec_of_sz(x_sz, 0.0));
        let ceze = ceze.unwrap_or(vec_of_sz(x_sz, 1.0));
        let cezh = cezh.unwrap_or(vec_of_sz(x_sz, IMP0));
        let hy = hy.unwrap_or(vec_of_sz(x_sz, 0.0));
        let chyh = chyh.unwrap_or(vec_of_sz(x_sz, 1.0));
        let chye = chye.unwrap_or(vec_of_sz(x_sz, 1.0 / IMP0));
        let cdtds = cdtds.unwrap_or(1.0);

        // Assert that all passed vectors are the same size.
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
                x_sz,

                y_sz: 0,
                z_sz: 0,
                hx: Vec::new(),
                chxh: Vec::new(),
                chxe: Vec::new(),

                hy,
                chyh,
                chye,

                hz: Vec::new(),
                chzh: Vec::new(),
                chze: Vec::new(),
                ex: Vec::new(),
                cexe: Vec::new(),
                cexh: Vec::new(),
                ey: Vec::new(),
                ceye: Vec::new(),
                ceyh: Vec::new(),

                ez,
                ceze,
                cezh,
                cdtds,
            };
            Ok(FDTDSim {
                g,
                dimension: GridDimension::One,
                post_magnetic: None,
                post_electric: None,
                time: 0,
            })
        }
    }

    /// Build a new 2d simulation.
    pub fn new_2d(
        x_sz: usize,
        y_sz: usize,
        ez: Option<Vec<f64>>,
        ceze: Option<Vec<f64>>,
        cezh: Option<Vec<f64>>,
        hy: Option<Vec<f64>>,
        chyh: Option<Vec<f64>>,
        chye: Option<Vec<f64>>,
        hx: Option<Vec<f64>>,
        chxh: Option<Vec<f64>>,
        chxe: Option<Vec<f64>>,
        cdtds: Option<f64>,
    ) -> Result<Self, error::FDTDError> {
        let cdtds = cdtds.unwrap_or(1.0 / (2.0f64.sqrt()));

        // Defaults.
        let hx = hx.unwrap_or(vec_of_sz(x_sz * (y_sz - 1), 0.0));
        let chxh = chxh.unwrap_or(vec_of_sz(x_sz * (y_sz - 1), 1.0));
        let chxe = chxe.unwrap_or(vec_of_sz(x_sz * (y_sz - 1), cdtds / IMP0));

        let hy = hy.unwrap_or(vec_of_sz((x_sz - 1) * y_sz, 0.0));
        let chyh = chyh.unwrap_or(vec_of_sz((x_sz - 1) * y_sz, 1.0));
        let chye = chye.unwrap_or(vec_of_sz((x_sz - 1) * y_sz, 1.0 / IMP0));

        let ez = ez.unwrap_or(vec_of_sz(x_sz * y_sz, 0.0));
        let ceze = ceze.unwrap_or(vec_of_sz(x_sz * y_sz, 1.0));
        let cezh = cezh.unwrap_or(vec_of_sz(x_sz * y_sz, cdtds * IMP0));

        let g = Grid {
            x_sz,
            y_sz,

            z_sz: 0,
            hx,
            chxh,
            chxe,

            hy,
            chyh,
            chye,

            hz: Vec::new(),
            chzh: Vec::new(),
            chze: Vec::new(),
            ex: Vec::new(),
            cexe: Vec::new(),
            cexh: Vec::new(),
            ey: Vec::new(),
            ceye: Vec::new(),
            ceyh: Vec::new(),

            ez,
            ceze,
            cezh,
            cdtds,
        };

        Ok(FDTDSim {
            g,
            dimension: GridDimension::Two(Polarization::Magnetic),
            post_magnetic: None,
            post_electric: None,
            time: 0,
        })
    }

    pub fn set_post_magnetic(&mut self, f: Option<A>) {
        self.post_magnetic = f;
    }

    pub fn set_post_electric(&mut self, f: Option<B>) {
        self.post_electric = f;
    }

    pub fn step(&mut self) {
        self.time += 1;
        self.g.update_magnetic(self.dimension);

        match &mut self.post_magnetic {
            Some(v) => v(self.time, &mut self.g),
            None => (),
        }

        self.g.update_electric(self.dimension);

        match &mut self.post_electric {
            Some(v) => v(self.time, &mut self.g),
            None => (),
        }
    }
}
