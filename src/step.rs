// src/step.rs
use crate::grid::Grid;

// TODO: Can macros be nested?
macro_rules! hx {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.hx[$x * ($grid.y_sz) + $y]
    };
}

macro_rules! chxh {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.chxh[$x * ($grid.y_sz) + $y]
    };
}

macro_rules! chxe {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.chxe[$x * ($grid.y_sz) + $y]
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

pub fn magnetic_1d(g: &mut Grid) {
    for mm in 0..g.x_sz - 1 {
        g.hy[mm] = g.chyh[mm] * g.hy[mm] + g.chye[mm] * (g.ez[mm + 1] - g.ez[mm]);
    }
}

// TM^Z
pub fn magnetic_2d(g: &mut Grid) {
    for mm in 0..g.x_sz {
        for nn in 0..g.y_sz - 1 {
            // hx(mm, nn) = chxh(mm, nn) * hx(mm, nn)
            //  - chxe(mm, nn) * (ez(mm, nn + 1) - ez(mm, nn))
            hx!(g, mm, nn) = chxh!(g, mm, nn) * hx!(g, mm, nn)
                - chxe!(g, mm, nn) * (ez!(g, mm, (nn + 1)) - ez!(g, mm, nn));
        }
    }

    for mm in 0..g.x_sz - 1 {
        for nn in 0..g.y_sz {
            // hy(mm, nn) = chyh(mm, nn) * hy(mm, nn)
            //  + chye(mm, nn) * (ez((mm + 1), nn) - ez(mm, nn))
            hy!(g, mm, nn) = chyh!(g, mm, nn) * hy!(g, mm, nn)
                + chye!(g, mm, nn) * (ez!(g, (mm + 1), nn) - ez!(g, mm, nn));
        }
    }
}

pub fn electric_1d(g: &mut Grid) {
    for mm in 1..g.x_sz {
        g.ez[mm] = g.ceze[mm] * g.ez[mm] + g.cezh[mm] * (g.hy[mm] - g.hy[mm - 1]);
    }
}

// TM^Z
pub fn electric_2d(g: &mut Grid) {
    for mm in 1..g.x_sz - 1 {
        for nn in 1..g.y_sz - 1 {
            // ez(mm, nn) = ceze(mm, nn) * ez(mm, nn)
            //  + cezh(mm, nn)* ((hy(mm, nn) - hy((mm - 1), nn)) -
            //      (hx(mm, nn) - hx(mm, (nn - 1))))
            ez!(g, mm, nn) = ceze!(g, mm, nn) * ez!(g, mm, nn)
                + cezh!(g, mm, nn)
                    * ((hy!(g, mm, nn) - hy!(g, (mm - 1), nn))
                        - (hx!(g, mm, nn) - hx!(g, mm, (nn - 1))));
        }
    }
}
