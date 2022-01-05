// 1D FDTD simulation of a lossless dielectric region followed by a lossy
// layer which matches the impedance of the dielectric. Rust port of
// the 'Program 3.8'.
use fdtd::fdtd::{FDTDSim, GridDimension};
use fdtd::grid::{Grid, IMP0};
use fdtd::snapshot;

const SIZE: usize = 200;
const LOSS: f64 = 0.02;
const LOSS_LAYER: usize = 180;

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
        } else if mm < LOSS_LAYER {
            ceze[mm] = 1.0;
            cezh[mm] = IMP0 / 9.0;
        } else {
            ceze[mm] = (1.0 - LOSS) / (1.0 + LOSS);
            cezh[mm] = IMP0 / 9.0 / (1.0 + LOSS);
        }
    }

    // Build the magnetic field update coefficients.
    let mut chyh = Vec::with_capacity(SIZE);
    let mut chye = Vec::with_capacity(SIZE);
    chyh.resize(SIZE, 0.0);
    chye.resize(SIZE, 0.0);
    for mm in 0..SIZE - 1 {
        if mm < LOSS_LAYER {
            chyh[mm] = 1.0;
            chye[mm] = 1.0 / IMP0;
        } else {
            chyh[mm] = (1.0 - LOSS) / (1.0 + LOSS);
            chye[mm] = 1.0 / IMP0 / (1.0 + LOSS);
        }
    }

    // ABC for ez[0].
    let abc_ez_fn = |g: &mut Grid| {
        g.ez[0] = g.ez[1];
    };

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
        abc_ez_fn(g);
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

    fdtd_sim.set_post_magnetic(Some(post_magnetic));
    fdtd_sim.set_post_electric(Some(post_electric));

    for _ in 0..450 {
        fdtd_sim.step(&mut g);
    }
}
