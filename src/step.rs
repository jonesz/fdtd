// src/step.rs
use crate::grid::Grid;

// TODO: Can macros be nested; can we generate these macros via a macro?
macro_rules! hx {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.hx[$x * ($grid.y_sz) + $y]
    };

    ($grid:ident, $x:expr, $y:expr, $z:expr) => {
        $grid.hx[($x * ($grid.y_sz) + $y) * $grid.z_sz + $z]
    };
}

macro_rules! chxh {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.chxh[$x * ($grid.y_sz) + $y]
    };

    ($grid:ident, $x:expr, $y:expr, $z:expr) => {
        $grid.chxh[($x * ($grid.y_sz) + $y) * $grid.z_sz + $y]
    };
}

macro_rules! chxe {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.chxe[$x * ($grid.y_sz) + $y]
    };

    ($grid:ident, $x:expr, $y:expr, $z:expr) => {
        $grid.chxe[$x * ($grid.y_sz) + $y]
    };
}

macro_rules! hy {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.hy[$x * ($grid.y_sz) + $y]
    };

    ($grid:ident, $x:expr, $y:expr, $z:expr) => {
        $grid.hy[$x * ($grid.y_sz) + $y]
    };
}

macro_rules! chyh {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.chyh[$x * ($grid.y_sz) + $y]
    };

    ($grid:ident, $x:expr, $y:expr, $z:expr) => {
        $grid.chyh[$x * ($grid.y_sz) + $y]
    };
}

macro_rules! chye {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.chye[$x * ($grid.y_sz) + $y]
    };

    ($grid:ident, $x:expr, $y:expr, $z:expr) => {
        $grid.chye[$x * ($grid.y_sz) + $y]
    };
}

macro_rules! hz {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.hz[$x * ($grid.y_sz) + $y]
    };

    ($grid:ident, $x:expr, $y:expr, $z:expr) => {
        $grid.hz[$x * ($grid.y_sz) + $y]
    };
}

macro_rules! chzh {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.chzh[$x * ($grid.y_sz) + $y]
    };

    ($grid:ident, $x:expr, $y:expr, $z:expr) => {
        $grid.chzh[$x * ($grid.y_sz) + $y]
    };
}

macro_rules! chze {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.chze[$x * ($grid.y_sz) + $y]
    };

    ($grid:ident, $x:expr, $y:expr, $z:expr) => {
        $grid.chze[$x * ($grid.y_sz) + $y]
    };
}

macro_rules! ex {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.ex[$x * ($grid.y_sz) + $y]
    };

    ($grid:ident, $x:expr, $y:expr, $z:expr) => {
        $grid.ex[$x * ($grid.y_sz) + $y]
    };
}

macro_rules! cexe {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.cexe[$x * ($grid.y_sz) + $y]
    };

    ($grid:ident, $x:expr, $y:expr, $z:expr) => {
        $grid.cexe[$x * ($grid.y_sz) + $y]
    };
}

macro_rules! cexh {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.cexh[$x * ($grid.y_sz) + $y]
    };

    ($grid:ident, $x:expr, $y:expr, $z:expr) => {
        $grid.cexh[$x * ($grid.y_sz) + $y]
    };
}

macro_rules! ey {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.ey[$x * ($grid.y_sz) + $y]
    };

    ($grid:ident, $x:expr, $y:expr, $z:expr) => {
        $grid.ey[$x * ($grid.y_sz) + $y]
    };
}

macro_rules! ceye {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.ceye[$x * ($grid.y_sz) + $y]
    };

    ($grid:ident, $x:expr, $y:expr, $z:expr) => {
        $grid.ceye[$x * ($grid.y_sz) + $y]
    };
}

macro_rules! ceyh {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.ceyh[$x * ($grid.y_sz) + $y]
    };

    ($grid:ident, $x:expr, $y:expr, $z:expr) => {
        $grid.ceyh[$x * ($grid.y_sz) + $y]
    };
}

macro_rules! ez {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.ez[$x * ($grid.y_sz) + $y]
    };

    ($grid:ident, $x:expr, $y:expr, $z:expr) => {
        $grid.ez[$x * ($grid.y_sz) + $y]
    };
}

macro_rules! ceze {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.ceze[$x * ($grid.y_sz) + $y]
    };

    ($grid:ident, $x:expr, $y:expr, $z:expr) => {
        $grid.ceze[$x * ($grid.y_sz) + $y]
    };
}

macro_rules! cezh {
    ($grid:ident, $x:expr, $y:expr) => {
        $grid.cezh[$x * ($grid.y_sz) + $y]
    };

    ($grid:ident, $x:expr, $y:expr, $z:expr) => {
        $grid.cezh[$x * ($grid.y_sz) + $y]
    };
}

// 1D
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

// 3D
pub fn magnetic_3d(g: &mut Grid) {
    for m in 0..g.x_sz {
        for n in 0..g.y_sz - 1 {
            for p in 0..g.z_sz - 1 {
                // hx(m, n, p) = chxh(m, n, p) * hx(m, n, p) +
                //  chxe(m, n, p) * ((ey(m, n, p + 1) - ey(m, n, p)) -
                //      (ez(m, n + 1, p) - ez(m, n, p)))
                hx!(g, m, n, p) = chxh!(g, m, n, p) * hx!(g, m, n, p)
                    + chxe!(g, m, n, p)
                        * ((ey!(g, m, n, p + 1) - ey!(g, m, n, p))
                            - (ez!(g, m, n + 1, p) - ez!(g, m, n, p)))
            }
        }
    }

    for m in 0..g.x_sz - 1 {
        for n in 0..g.y_sz {
            for p in 0..g.z_sz - 1 {
                // hy(m, n, p) = chyh(m, n, p) * hy(m, n, p) +
                //  chye(m, n, p) * ((ez(m + 1, n, p) - ez(m, n, p)) -
                //      (ex(m, n, p + 1) - ex(m, n, p)))
                hy!(g, m, n, p) = chyh!(g, m, n, p) * hy!(g, m, n, p)
                    + chye!(g, m, n, p)
                        * ((ez!(g, m + 1, n, p) - ez!(g, m, n, p))
                            - (ex!(g, m, n, p + 1) - ex!(g, m, n, p)))
            }
        }
    }

    for m in 0..g.x_sz - 1 {
        for n in 0..g.y_sz - 1 {
            for p in 0..g.z_sz {
                // hz(m, n, p) = chzh(m, n, p) * hz(m, n, p) +
                // chze(m, n, p) * ((ex(m, n + 1, p) - ex(m, n, p)) -
                //      (ey(m + 1, n, p) - ey(m, n, p)))
                hz!(g, m, n, p) = chzh!(g, m, n, p) * hz!(g, m, n, p)
                    + chze!(g, m, n, p)
                        * ((ex!(g, m, n + 1, p) - ex!(g, m, n, p))
                            - (ey!(g, m + 1, n, p) - ey!(g, m, n, p)))
            }
        }
    }
}

// 1D
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

// 3D
pub fn electric_3d(g: &mut Grid) {
    // ex(m, n, p) = cexe(m, n, p) * ex(m, n, p) + cexh(m, n, p)
    //  * ((hz(m, n, p) - hz(m, n - 1, p)) - (hy(m, n, p) - hy(m, n, p - 1)))
    for m in 0..g.x_sz {
        for n in 1..g.y_sz {
            for p in 1..g.z_sz {
                ex!(g, m, n, p) = cexe!(g, m, n, p) * ex!(g, m, n, p)
                    + cexh!(g, m, n, p)
                        * ((hz!(g, m, n, p) - hz!(g, m, n - 1, p))
                            - (hy!(g, m, n, p) - hy!(g, m, n, p - 1)))
            }
        }
    }

    // ey(m, n, p) = ceye(m, n, p) * ey(m, n, p) + ceyh(m, n, p)
    //  * ((hx(m, n, p) - hx(m, n, p - 1)) - (hz(m, n, p) - hz(m - 1, n, p)))
    for m in 1..g.x_sz {
        for n in 0..g.y_sz {
            for p in 1..g.z_sz {
                ey!(g, m, n, p) = ceye!(g, m, n, p) * ey!(g, m, n, p)
                    + ceyh!(g, m, n, p)
                        * ((hx!(g, m, n, p) - hx!(g, m, n, p - 1))
                            - (hz!(g, m, n, p) - hz!(g, m - 1, n, p)))
            }
        }
    }

    // ez(m, n, p) = ceze(m, n, p) * ez(m, n, p) + cezh(m, n, p)
    //  * ((hy(m, n, p) - hy(m - 1, n, p)) - (hx(m, n, p) - hx(m, n - 1, p)))
    for m in 1..g.x_sz {
        for n in 1..g.y_sz {
            for p in 0..g.z_sz {
                ez!(g, m, n, p) = ceze!(g, m, n, p) * ez!(g, m, n, p)
                    + cezh!(g, m, n, p)
                        * ((hy!(g, m, n, p) - hy!(g, m - 1, n, p))
                            - (hx!(g, m, n, p) - hx!(g, m, n - 1, p)))
            }
        }
    }
}
