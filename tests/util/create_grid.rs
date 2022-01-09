// tests/create_grid.rs
use fdtd::fdtd::{GridDimension, Polarization};
use fdtd::grid::Grid;
use rand::thread_rng;
use rand::Rng;

/// Create a default grid.
pub fn default_grid(
    x_sz: usize,
    y_sz: Option<usize>,
    z_sz: Option<usize>,
    g: GridDimension,
) -> Grid {
    match g {
        GridDimension::One => Grid::new_1d(x_sz),
        GridDimension::Two(Polarization::Magnetic) => {
            Grid::new_2d(x_sz, y_sz.expect("Need 'y_sz' for 2 dimensions."), None)
        }
        GridDimension::Three => Grid::new_3d(
            x_sz,
            y_sz.expect("Need 'y_sz' for 3 dimensions."),
            z_sz.expect("Need 'z_sz' for 3 dimensions."),
            None,
        ),
        _ => panic!("Unimplemented!"),
    }
}

// TODO: Place a Gaussian pulse or something in here.
/// Create a grid populated with precomputed values.
pub fn precomputed_grid(
    x_sz: usize,
    y_sz: Option<usize>,
    z_sz: Option<usize>,
    g: GridDimension,
) -> Grid {
    match g {
        GridDimension::One => Grid::new_1d(x_sz),
        GridDimension::Two(Polarization::Magnetic) => {
            Grid::new_2d(x_sz, y_sz.expect("Need 'y_sz' for 2 dimensions."), None)
        }
        GridDimension::Three => Grid::new_3d(
            x_sz,
            y_sz.expect("Need 'y_sz' for 3 dimensions."),
            z_sz.expect("Need 'z_sz' for 3 dimensions."),
            None,
        ),
        _ => panic!("Unimplemented!"),
    }
}

/// Create a grid populated with random values.
pub fn random_grid(
    x_sz: usize,
    y_sz: Option<usize>,
    z_sz: Option<usize>,
    grid_type: GridDimension,
) -> Grid {
    let mut rng = thread_rng();
    let mut g = match grid_type {
        GridDimension::One => Grid::new_1d(x_sz),
        GridDimension::Two(Polarization::Magnetic) => {
            Grid::new_2d(x_sz, y_sz.expect("Need 'y_sz' for 2 dimensions."), None)
        }
        GridDimension::Three => Grid::new_3d(
            x_sz,
            y_sz.expect("Need 'y_sz' for 3 dimensions."),
            z_sz.expect("Need 'z_sz' for 3 dimensions."),
            None,
        ),
        _ => panic!("Unimplemented!"),
    };

    let iter_len = match grid_type {
        GridDimension::One => x_sz,
        GridDimension::Two(_) => x_sz * y_sz.expect("Need 'y_sz' for 2 dimensions."),
        GridDimension::Three => {
            x_sz * y_sz.expect("Need 'y_sz' for 2 dimensions.")
                * z_sz.expect("Need 'z_sz' for 2 dimensions.")
        }
    };

    for i in 0..iter_len {
        if let GridDimension::Three = grid_type {
            g.ex[i] = rng.gen_range(-10.0..10.0);
            g.cexh[i] = rng.gen_range(-10.0..10.0);
            g.cexe[i] = rng.gen_range(-10.0..10.0);
            g.ey[i] = rng.gen_range(-10.0..10.0);
            g.ceyh[i] = rng.gen_range(-10.0..10.0);
            g.ceye[i] = rng.gen_range(-10.0..10.0);
        }

        if let GridDimension::Two(_) | GridDimension::Three = grid_type {
            g.hx[i] = rng.gen_range(-10.0..10.0);
            g.chxh[i] = rng.gen_range(-10.0..10.0);
            g.chxe[i] = rng.gen_range(-10.0..10.0);
            g.hy[i] = rng.gen_range(-10.0..10.0);
            g.chyh[i] = rng.gen_range(-10.0..10.0);
            g.chye[i] = rng.gen_range(-10.0..10.0);
        }

        g.hy[i] = rng.gen_range(-10.0..10.0);
        g.chyh[i] = rng.gen_range(-10.0..10.0);
        g.chye[i] = rng.gen_range(-10.0..10.0);
        g.ez[i] = rng.gen_range(-10.0..10.0);
        g.cezh[i] = rng.gen_range(-10.0..10.0);
        g.ceze[i] = rng.gen_range(-10.0..10.0);
    }

    g
}
