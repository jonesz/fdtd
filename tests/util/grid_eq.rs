// util/grid_eq.rs
// TODO: If the Grid struct changes, this code can't be aware; in some regards,
// implementing PartialEq with the below 'veq' macro is a better choice.
use fdtd::grid::Grid;

// Determine whether two vectors are equal, ignoring NaN/Inf values.
macro_rules! veq {
    ($v1:expr, $v2:expr) => {
        $v1.iter().zip($v2).fold(true, |acc, (a, b)| {
            if (a.is_infinite() && b.is_infinite()) {
                acc & true
            } else if (a.is_nan() && b.is_nan()) {
                acc & true
            } else {
                acc & (a == b)
            }
        })
    };
}

/// Return whether two grids are equal, ignoring NaN/Inf values.
pub fn grid_eq(a: &Grid, b: &Grid) -> bool {
    (a.x_sz == b.x_sz)
        & (a.y_sz == b.y_sz)
        & (a.z_sz == b.z_sz)
        & veq!(a.hx, &b.hx)
        & veq!(a.chxh, &b.chxh)
        & veq!(a.chxe, &b.chxe)
        & veq!(a.hy, &b.hy)
        & veq!(a.chyh, &b.chyh)
        & veq!(a.chye, &b.chye)
        & veq!(a.hz, &b.hz)
        & veq!(a.chzh, &b.chzh)
        & veq!(a.chze, &b.chze)
        & veq!(a.ex, &b.ex)
        & veq!(a.cexe, &b.cexe)
        & veq!(a.cexh, &b.cexh)
        & veq!(a.ey, &b.ey)
        & veq!(a.ceye, &b.ceye)
        & veq!(a.ceyh, &b.ceyh)
        & veq!(a.ez, &b.ez)
        & veq!(a.ceze, &b.ceze)
        & veq!(a.cezh, &b.cezh)
        & (a.cdtds == b.cdtds)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIZE: usize = 100;

    #[test]
    fn grid_eq_eq() {
        let grid1 = Grid::default();
        let grid2 = Grid::default();

        assert_eq!(grid_eq(&grid1, &grid2), true);

        let mut grid1 = Grid::new_1d(SIZE);
        let mut grid2 = Grid::new_1d(SIZE);

        grid1.ez[0] = f64::INFINITY;
        grid2.ez[0] = f64::NEG_INFINITY;
        assert_eq!(grid_eq(&grid1, &grid2), true);

        grid1.ez[0] = f64::NAN;
        grid2.ez[0] = f64::NAN;
        assert_eq!(grid_eq(&grid1, &grid2), true);
    }

    #[test]
    fn grid_eq_neq() {
        let mut grid1 = Grid::new_1d(100);
        let grid2 = Grid::new_1d(100);

        grid1.ez[0] = 1.0;
        assert_eq!(grid_eq(&grid1, &grid2), false);
        grid1.ez[0] = grid2.ez[0];

        grid1.hy[3] = -3.0;
        assert_eq!(grid_eq(&grid1, &grid2), false);
        grid1.hy[3] = grid2.hy[3];

        grid1.ez[0] = f64::INFINITY;
        assert_eq!(grid_eq(&grid1, &grid2), false);

        grid1.ez[0] = f64::NAN;
        assert_eq!(grid_eq(&grid1, &grid2), false);
    }
}
