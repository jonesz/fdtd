// tests/create_grid.rs
use fdtd::grid::Grid;
use rand::thread_rng;
use rand::Rng;

/// Create a default 1D grid.
pub fn default_1d_grid(sz: usize) -> Grid {
    Grid::new_1d(sz)
}

// TODO: Place a Gaussian pulse or something in here.
/// Create a 1D grid populated with precomputed values.
pub fn precomputed_1d_grid(sz: usize) -> Grid {
    Grid::new_1d(sz)
}

/// Create a 1D grid populated with random values.
pub fn random_1d_grid(sz: usize) -> Grid {
    let mut rng = thread_rng();
    let mut g = Grid::new_1d(sz);

    for i in 0..sz {
        // TODO: Presumably this doesn't produce NaNs or Inf.
        g.hy[i] = rng.gen_range(-10.0..10.0);
        g.chyh[i] = rng.gen_range(-10.0..10.0);
        g.chye[i] = rng.gen_range(-10.0..10.0);
        g.ez[i] = rng.gen_range(-10.0..10.0);
        g.ceze[i] = rng.gen_range(-10.0..10.0);
        g.cezh[i] = rng.gen_range(-10.0..10.0);
    }

    g
}
