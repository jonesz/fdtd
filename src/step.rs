// src/step.rs
use crate::grid::Grid;

// TODO: Can macros be nested; can we generate these macros via a macro?
macro_rules! dim {
    ($grid:ident, $name:ident, $x:expr, $y:expr) => {
        $grid.$name[$x * ($grid.y_sz) + $y]
    };

    ($grid:ident, $name:ident, $x:expr, $y:expr, $z:expr) => {
        $grid.$name[($x * $grid.y_sz + $y) * $grid.z_sz + $z]
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
    for m in 0..g.x_sz {
        for n in 0..g.y_sz - 1 {
            // hx(m, n) = chxh(m, nn) * hx(m, nn)
            //  - chxe(m, n) * (ez(m, n + 1) - ez(m, n))
            dim!(g, hx, m, n) = dim!(g, chxh, m, n) * dim!(g, hx, m, n)
                - dim!(g, chxe, m, n) * (dim!(g, ez, m, (n + 1)) - dim!(g, ez, m, n));
        }
    }

    for m in 0..g.x_sz - 1 {
        for n in 0..g.y_sz {
            // hy(m, n) = chyh(m, n) * hy(m, n)
            //  + chye(m, n) * (ez((m + 1), n) - ez(m, n))
            dim!(g, hy, m, n) = dim!(g, chyh, m, n) * dim!(g, hy, m, n)
                + dim!(g, chye, m, n) * (dim!(g, ez, (m + 1), n) - dim!(g, ez, m, n));
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
                dim!(g, hx, m, n, p) = dim!(g, chxh, m, n, p) * dim!(g, hx, m, n, p)
                    + dim!(g, chxe, m, n, p)
                        * ((dim!(g, ey, m, n, p + 1) - dim!(g, ey, m, n, p))
                            - (dim!(g, ez, m, n + 1, p) - dim!(g, ez, m, n, p)))
            }
        }
    }

    for m in 0..g.x_sz - 1 {
        for n in 0..g.y_sz {
            for p in 0..g.z_sz - 1 {
                // hy(m, n, p) = chyh(m, n, p) * hy(m, n, p) +
                //  chye(m, n, p) * ((ez(m + 1, n, p) - ez(m, n, p)) -
                //      (ex(m, n, p + 1) - ex(m, n, p)))
                dim!(g, hy, m, n, p) = dim!(g, chyh, m, n, p) * dim!(g, hy, m, n, p)
                    + dim!(g, chye, m, n, p)
                        * ((dim!(g, ez, m + 1, n, p) - dim!(g, ez, m, n, p))
                            - (dim!(g, ex, m, n, p + 1) - dim!(g, ex, m, n, p)))
            }
        }
    }

    for m in 0..g.x_sz - 1 {
        for n in 0..g.y_sz - 1 {
            for p in 0..g.z_sz {
                // hz(m, n, p) = chzh(m, n, p) * hz(m, n, p) +
                // chze(m, n, p) * ((ex(m, n + 1, p) - ex(m, n, p)) -
                //      (ey(m + 1, n, p) - ey(m, n, p)))
                dim!(g, hz, m, n, p) = dim!(g, chzh, m, n, p) * dim!(g, hz, m, n, p)
                    + dim!(g, chze, m, n, p)
                        * ((dim!(g, ex, m, n + 1, p) - dim!(g, ex, m, n, p))
                            - (dim!(g, ey, m + 1, n, p) - dim!(g, ey, m, n, p)))
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
    for m in 1..g.x_sz {
        for n in 1..g.y_sz {
            // ez(m, n) = ceze(m, n) * ez(m, n)
            //  + cezh(m, n)* ((hy(m, n) - hy((m - 1), n)) -
            //      (hx(m, n) - hx(m, (n - 1))))
            dim!(g, ez, m, n) = dim!(g, ceze, m, n) * dim!(g, ez, m, n)
                + dim!(g, cezh, m, n)
                    * ((dim!(g, hy, m, n) - dim!(g, hy, (m - 1), n))
                        - (dim!(g, hx, m, n) - dim!(g, hx, m, (n - 1))));
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
                dim!(g, ex, m, n, p) = dim!(g, cexe, m, n, p) * dim!(g, ex, m, n, p)
                    + dim!(g, cexh, m, n, p)
                        * ((dim!(g, hz, m, n, p) - dim!(g, hz, m, n - 1, p))
                            - (dim!(g, hy, m, n, p) - dim!(g, hy, m, n, p - 1)))
            }
        }
    }

    // ey(m, n, p) = ceye(m, n, p) * ey(m, n, p) + ceyh(m, n, p)
    //  * ((hx(m, n, p) - hx(m, n, p - 1)) - (hz(m, n, p) - hz(m - 1, n, p)))
    for m in 1..g.x_sz {
        for n in 0..g.y_sz {
            for p in 1..g.z_sz {
                dim!(g, ey, m, n, p) = dim!(g, ceye, m, n, p) * dim!(g, ey, m, n, p)
                    + dim!(g, ceyh, m, n, p)
                        * ((dim!(g, hx, m, n, p) - dim!(g, hx, m, n, p - 1))
                            - (dim!(g, hz, m, n, p) - dim!(g, hz, m - 1, n, p)))
            }
        }
    }

    // ez(m, n, p) = ceze(m, n, p) * ez(m, n, p) + cezh(m, n, p)
    //  * ((hy(m, n, p) - hy(m - 1, n, p)) - (hx(m, n, p) - hx(m, n - 1, p)))
    for m in 1..g.x_sz {
        for n in 1..g.y_sz {
            for p in 0..g.z_sz {
                dim!(g, ez, m, n, p) = dim!(g, ceze, m, n, p) * dim!(g, ez, m, n, p)
                    + dim!(g, cezh, m, n, p)
                        * ((dim!(g, hy, m, n, p) - dim!(g, hy, m - 1, n, p))
                            - (dim!(g, hx, m, n, p) - dim!(g, hx, m, n - 1, p)))
            }
        }
    }
}
