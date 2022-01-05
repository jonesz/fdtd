// src/ricker2d.rs
// Rust port of 'Program 8.7'.
use fdtd::fdtd::{FDTDSim, GridDimension, Polarization};
use fdtd::grid::Grid;
use fdtd::ricker;
use fdtd::snapshot;

const SIZE_X: usize = 101;
const SIZE_Y: usize = 81;

// Must be greater than 0.0;
const PPW: f64 = 20.0;

fn main() {
    let cdtds = 1.0 / 2.0f64.sqrt();

    // Snapshot setup.
    let fdir = snapshot::create_output_dir().unwrap();
    let snapshot_fn = |t: usize, g: &mut Grid| snapshot::write(g, &fdir, t);

    let ez_inc = |t: usize, g: &mut Grid| {
        let loc = (SIZE_X / 2) * SIZE_Y + (SIZE_Y / 2);
        g.ez[loc] = ricker::ricker(t as f64, 0.0, cdtds, PPW);
    };

    let post_magnetic = |_t: usize, _g: &mut Grid| {};

    let post_electric = |t: usize, g: &mut Grid| {
        ez_inc(t, g);
        if t % 10 == 0 {
            snapshot_fn(t / 10, g).unwrap();
        }
    };

    let mut g = Grid::new_2d(SIZE_X, SIZE_Y, None);

    // Create the FDTDSim.
    let mut fdtd_sim = FDTDSim::new(
        Some(GridDimension::Two(Polarization::Magnetic)),
        Some(post_magnetic),
        Some(post_electric),
        None,
    );

    for _ in 0..300 {
        fdtd_sim.step(&mut g);
    }
}
