// src/abc.rs
use crate::fdtd::Grid;

// 1st order 1D advection ABC.
pub fn advection_abc_1st_order(cezh: &[f64], chye: &[f64]) -> impl FnMut(usize, &mut Grid) {
    let len = cezh.len();

    let tmp = (cezh[0] * chye[0]).sqrt();
    let coef_left = (tmp - 1.0) / (tmp + 1.0);
    let tmp = (cezh[len - 1] * chye[len - 2]).sqrt();
    let coef_right = (tmp - 1.0) / (tmp + 1.0);

    let mut old_left: f64 = 0.0;
    let mut old_right: f64 = 0.0;

    let f = move |_: usize, g: &mut Grid| {
        let end = g.sz - 1;

        g.ez[0] = old_left + coef_left * (g.ez[1] - g.ez[0]);
        g.ez[end] = old_right + coef_right * (g.ez[end - 2] - g.ez[end - 1]);

        old_left = g.ez[1];
        old_right = g.ez[end - 2];
    };

    f
}
