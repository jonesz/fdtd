// tests/fut_2d.rs
/// Prove some level of equivalency of the three implementations of the
/// futhark code.
use fdtd::error;
use fdtd::fdtd::{Backend, FDTDSim, GridDimension, Polarization};
use fdtd::grid::Grid;

mod util;

const SIZE_X: usize = 10;
const SIZE_Y: usize = 10;

/// Return a simulation that *should* call step_single_futhark (we have an
/// post_electric fn).
fn setup_step_single_futhark(
) -> Result<FDTDSim<impl FnMut(usize, &mut Grid), impl FnMut(usize, &mut Grid)>, error::FDTDError> {
    // TODO: Does the compiler optimize these out?
    let post_magnetic = |_t: usize, _g: &mut Grid| {};
    let post_electric = |_t: usize, _g: &mut Grid| {};

    let mut fdtd_sim = FDTDSim::new(
        Some(GridDimension::Two(Polarization::Magnetic)),
        Some(Backend::Futhark),
        Some(post_magnetic),
        Some(post_electric),
        None,
    )?;

    fdtd_sim.set_post_magnetic(None);

    Ok(fdtd_sim)
}

/// Return a simulation that *should* call step_split_futhark (we have both
/// post_magnetic and post_electric functions).
fn setup_step_split_futhark(
) -> Result<FDTDSim<impl FnMut(usize, &mut Grid), impl FnMut(usize, &mut Grid)>, error::FDTDError> {
    // TODO: Does the compiler optimize these out?
    let post_magnetic = |_t: usize, _g: &mut Grid| {};
    let post_electric = |_t: usize, _g: &mut Grid| {};

    let fdtd_sim = FDTDSim::new(
        Some(GridDimension::Two(Polarization::Magnetic)),
        Some(Backend::Futhark),
        Some(post_magnetic),
        Some(post_electric),
        None,
    )?;

    Ok(fdtd_sim)
}

/// Return a simulation that *should* call step_mul_futhark (we have neither
/// post_magnetic or post_electric).
fn setup_step_mul_futhark(
) -> Result<FDTDSim<impl FnMut(usize, &mut Grid), impl FnMut(usize, &mut Grid)>, error::FDTDError> {
    let post_magnetic = |_t: usize, _g: &mut Grid| {};
    let post_electric = |_t: usize, _g: &mut Grid| {};

    let mut res = FDTDSim::new(
        Some(GridDimension::Two(Polarization::Magnetic)),
        Some(Backend::Futhark),
        Some(post_magnetic),
        Some(post_electric),
        None,
    )?;

    res.set_post_magnetic(None);
    res.set_post_electric(None);

    Ok(res)
}

/// Return a simulation that *should* call the native backend.
fn setup_step_native(
) -> Result<FDTDSim<impl FnMut(usize, &mut Grid), impl FnMut(usize, &mut Grid)>, error::FDTDError> {
    // TODO: Does the compiler optimize these out?
    let post_magnetic = |_t: usize, _g: &mut Grid| {};
    let post_electric = |_t: usize, _g: &mut Grid| {};

    let mut res = FDTDSim::new(
        Some(GridDimension::Two(Polarization::Magnetic)),
        Some(Backend::Native),
        Some(post_magnetic),
        Some(post_electric),
        None,
    )?;

    res.set_post_magnetic(None);
    res.set_post_electric(None);

    Ok(res)
}

#[test]
fn test_default_grid() {
    let mut grid1 = util::create_grid::default_grid(
        SIZE_X,
        Some(SIZE_Y),
        None,
        GridDimension::Two(Polarization::Magnetic),
    );
    let mut grid2 = grid1.clone();
    let mut grid3 = grid1.clone();
    let mut grid4 = grid1.clone();

    let mut sim_single = setup_step_single_futhark().unwrap();
    let mut sim_split = setup_step_split_futhark().unwrap();
    let mut sim_mul = setup_step_mul_futhark().unwrap();
    let mut sim_native = setup_step_native().unwrap();

    for _ in 0..500 {
        assert_eq!(sim_single.step(&mut grid1).is_ok(), true);
        assert_eq!(sim_split.step(&mut grid2).is_ok(), true);
        assert_eq!(sim_mul.step(&mut grid3).is_ok(), true);
        assert_eq!(sim_native.step(&mut grid4).is_ok(), true);

        assert_eq!(grid1.eq(&grid2), true);
        assert_eq!(grid1.eq(&grid3), true);
        assert_eq!(grid1.eq(&grid4), true);
    }
}

#[test]
fn test_precomputed_grid() {
    let mut grid1 = util::create_grid::precomputed_grid(
        SIZE_X,
        Some(SIZE_Y),
        None,
        GridDimension::Two(Polarization::Magnetic),
    );
    let mut grid2 = grid1.clone();
    let mut grid3 = grid1.clone();
    let mut grid4 = grid1.clone();

    let mut sim_mul = setup_step_mul_futhark().unwrap();
    let mut sim_single = setup_step_single_futhark().unwrap();
    let mut sim_split = setup_step_split_futhark().unwrap();
    let mut sim_native = setup_step_native().unwrap();

    for _ in 0..500 {
        assert_eq!(sim_single.step(&mut grid1).is_ok(), true);
        assert_eq!(sim_split.step(&mut grid2).is_ok(), true);
        assert_eq!(sim_mul.step(&mut grid3).is_ok(), true);
        assert_eq!(sim_native.step(&mut grid4).is_ok(), true);

        assert_eq!(grid1.eq(&grid2), true);
        assert_eq!(grid1.eq(&grid3), true);
        assert_eq!(grid1.eq(&grid4), true);
    }
}

#[test]
fn test_random_grid() {
    let mut grid1 = util::create_grid::random_grid(
        SIZE_X,
        Some(SIZE_Y),
        None,
        GridDimension::Two(Polarization::Magnetic),
    );
    let mut grid2 = grid1.clone();
    let mut grid3 = grid1.clone();
    let mut grid4 = grid1.clone();

    assert_eq!(grid1.eq(&grid2), true);
    assert_eq!(grid1.eq(&grid3), true);
    assert_eq!(grid1.eq(&grid4), true);

    let mut sim_single = setup_step_single_futhark().unwrap();
    let mut sim_split = setup_step_split_futhark().unwrap();
    let mut sim_mul = setup_step_mul_futhark().unwrap();
    let mut sim_native = setup_step_native().unwrap();

    for _ in 0..500 {
        assert_eq!(sim_single.step(&mut grid1).is_ok(), true);
        assert_eq!(sim_split.step(&mut grid2).is_ok(), true);
        assert_eq!(sim_mul.step(&mut grid3).is_ok(), true);
        assert_eq!(sim_native.step(&mut grid4).is_ok(), true);

        assert_eq!(util::grid_eq::grid_eq(&grid1, &grid2), true);
        assert_eq!(util::grid_eq::grid_eq(&grid1, &grid3), true);
        assert_eq!(util::grid_eq::grid_eq(&grid1, &grid4), true);
    }
}
