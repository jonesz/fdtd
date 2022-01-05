// Rust port of the 1st order ABC demo 'Program 6.2'.
use fdtd::abc;
use fdtd::fdtd::{FDTDSim, GridDimension};
use fdtd::grid::{Grid, IMP0};
use fdtd::snapshot;

const SIZE: usize = 200;
const EPSR: f64 = 9.0;

fn main() {
    // Build the electric field update coefficients.
    let mut ceze = Vec::with_capacity(SIZE);
    let mut cezh = Vec::with_capacity(SIZE);
    ceze.resize(SIZE, 0.0);
    cezh.resize(SIZE, 0.0);
    for mm in 0..SIZE {
        if mm < 100 {
            ceze[mm] = 1.0;
            cezh[mm] = IMP0;
        } else {
            ceze[mm] = 1.0;
            cezh[mm] = IMP0 / EPSR;
        }
    }

    // Build the magnetic field update coefficients.
    let mut chyh = Vec::with_capacity(SIZE);
    let mut chye = Vec::with_capacity(SIZE);
    chyh.resize(SIZE, 0.0);
    chye.resize(SIZE, 0.0);
    for mm in 0..SIZE - 1 {
        chyh[mm] = 1.0;
        chye[mm] = 1.0 / IMP0;
    }

    let mut abc_fn = abc::advection_abc_1st_order(&cezh, &chye);

    // TFSF for Hy adjacent to TFSF boundary.
    let hy_tfsf_fn = |t: usize, g: &mut Grid| {
        let t = t as f64;
        g.hy[49] -= (-(t - 30.0) * (t - 30.0) / 100.0).exp() / IMP0;
    };

    // TFSF for Ez adjacent to TFSF boundary.
    let ez_tfsf_fn = |t: usize, g: &mut Grid| {
        let t = t as f64;
        g.ez[50] += (-(t + 0.5 - (-0.5) - 30.0) * (t + 0.5 - (-0.5) - 30.0) / 100.0).exp();
    };

    // Snapshot setup.
    let fdir = snapshot::create_output_dir().unwrap();
    let snapshot_fn = |t: usize, g: &mut Grid| snapshot::write(g, &fdir, t);

    // Build the post magnetic/electric functions.
    let post_magnetic = |t: usize, g: &mut Grid| {
        hy_tfsf_fn(t, g);
        abc_fn(t, g);
    };

    let post_electric = |t: usize, g: &mut Grid| {
        ez_tfsf_fn(t, g);
        if t % 10 == 0 {
            snapshot_fn(t / 10, g).unwrap();
        }
    };

    let mut g = Grid::new_1d(SIZE);
    g.ceze = ceze;
    g.cezh = cezh;
    g.chyh = chyh;
    g.chye = chye;

    // Create the FDTDSim.
    let mut fdtd_sim = FDTDSim::new(
        Some(GridDimension::One),
        Some(post_magnetic),
        Some(post_electric),
        None,
    );

    for _ in 0..450 {
        fdtd_sim.step(&mut g);
    }
}
