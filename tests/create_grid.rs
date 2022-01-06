// tests/create_grid.rs
use fdtd::grid::Grid;
use rand::{rngs::StdRng, RngCore, SeedableRng};

/// Create a default 1D grid.
fn default_1d_grid(sz: usize) -> Grid {
    Grid::new_1d(sz)
}

// TODO: Place a Gaussian pulse or something in here.
/// Create a 1D grid populated with precomputed values.
fn precomputed_1d_grid(sz: usize) -> Grid {
    Grid::new_1d(sz)
}

/// Create a 1D grid populated with random values.
fn random_1d_grid(sz: usize, seed: [u8; 32]) -> Grid {
    let mut rng = StdRng::from_seed(seed);
    let mut g = Grid::new_1d(sz);

    for i in 0..sz {
        // TODO: I don't think this produces NaN values?
        g.hy[i] = rng.next_u32() as f64;
        g.chyh[i] = rng.next_u32() as f64;
        g.chye[i] = rng.next_u32() as f64;
        g.ez[i] = rng.next_u32() as f64;
        g.ceze[i] = rng.next_u32() as f64;
        g.cezh[i] = rng.next_u32() as f64;
    }

    g
}
