// src/grid.rs
use serde::Serialize;

/// Characteristic impedance of free space.
pub const IMP0: f64 = 377.0;

#[derive(Serialize, Clone, PartialEq)]
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

    pub cdtds: f64,
}

impl Default for Grid {
    fn default() -> Self {
        Grid {
            x_sz: 0,
            y_sz: 0,
            z_sz: 0,

            hx: Vec::new(),
            chxh: Vec::new(),
            chxe: Vec::new(),

            hy: Vec::new(),
            chyh: Vec::new(),
            chye: Vec::new(),

            hz: Vec::new(),
            chzh: Vec::new(),
            chze: Vec::new(),

            ex: Vec::new(),
            cexe: Vec::new(),
            cexh: Vec::new(),

            ey: Vec::new(),
            ceye: Vec::new(),
            ceyh: Vec::new(),

            ez: Vec::new(),
            ceze: Vec::new(),
            cezh: Vec::new(),

            cdtds: 1.0,
        }
    }
}

impl Grid {
    /// Produce a vector of length 'sz' populated with 'value'.
    fn build_vec(sz: usize, value: f64) -> Vec<f64> {
        let mut v = Vec::with_capacity(sz);
        v.resize(sz, value);
        v
    }

    /// Build a new 1D grid.
    pub fn new_1d(x_sz: usize) -> Self {
        Grid {
            x_sz,
            ez: Grid::build_vec(x_sz, 0.0),
            ceze: Grid::build_vec(x_sz, 1.0),
            cezh: Grid::build_vec(x_sz, IMP0),

            hy: Grid::build_vec(x_sz, 0.0),
            chyh: Grid::build_vec(x_sz, 1.0),
            chye: Grid::build_vec(x_sz, 1.0 / IMP0),
            ..Default::default()
        }
    }

    /// Build a new 2D grid.
    pub fn new_2d(x_sz: usize, y_sz: usize, cdtds: Option<f64>) -> Self {
        let len = x_sz * y_sz;
        let cdtds = cdtds.unwrap_or(1.0 / 2.0f64.sqrt());

        Grid {
            x_sz,
            y_sz,

            hx: Grid::build_vec(len, 0.0),
            chxh: Grid::build_vec(len, 1.0),
            chxe: Grid::build_vec(len, cdtds / IMP0),

            hy: Grid::build_vec(len, 0.0),
            chyh: Grid::build_vec(len, 1.0),
            chye: Grid::build_vec(len, 1.0 / IMP0),

            ez: Grid::build_vec(len, 0.0),
            ceze: Grid::build_vec(len, 1.0),
            cezh: Grid::build_vec(len, cdtds * IMP0),

            ..Default::default()
        }
    }
}
