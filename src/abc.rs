// src/abc.rs
use crate::grid::Grid;

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
        let end = g.x_sz;

        g.ez[0] = old_left + coef_left * (g.ez[1] - g.ez[0]);
        old_left = g.ez[1];

        g.ez[end - 1] = old_right + coef_right * (g.ez[end - 2] - g.ez[end - 1]);
        old_right = g.ez[end - 2];
    };

    f
}

// 2nd order 1D advection ABC.
pub fn advection_abc_2nd_order(cezh: &[f64], chye: &[f64]) -> impl FnMut(usize, &mut Grid) {
    let len = cezh.len();

    let tmp1 = (cezh[0] * chye[0]).sqrt();
    let tmp2 = 1.0 / tmp1 + 2.0 + tmp1;
    let coef_left = [
        -(1.0 / tmp1 - 2.0 + tmp1) / tmp2,
        -2.0 * (tmp1 - 1.0 / tmp1) / tmp2,
        4.0 * (tmp1 + 1.0 / tmp1) / tmp2,
    ];

    let tmp1 = (cezh[len - 1] * chye[len - 2]).sqrt();
    let tmp2 = 1.0 / tmp1 + 2.0 + tmp1;
    let coef_right = [
        -(1.0 / tmp1 - 2.0 + tmp1) / tmp2,
        -2.0 * (tmp1 - 1.0 / tmp1) / tmp2,
        4.0 * (tmp1 + 1.0 / tmp1) / tmp2,
    ];

    let mut old_left1 = [0.0, 0.0, 0.0];
    let mut old_left2 = [0.0, 0.0, 0.0];
    let mut old_right1 = [0.0, 0.0, 0.0];
    let mut old_right2 = [0.0, 0.0, 0.0];

    let f = move |_: usize, g: &mut Grid| {
        let end = g.x_sz;

        g.ez[0] = coef_left[0] * (g.ez[2] + old_left2[0])
            + coef_left[1] * (old_left1[0] + old_left1[2] - g.ez[1] - old_left2[1])
            + coef_left[2] * old_left1[1]
            - old_left2[2];

        g.ez[end - 1] = coef_right[0] * (g.ez[end - 3] + old_right2[0])
            + coef_right[1] * (old_right1[0] + old_right1[2] - g.ez[end - 2] - old_right2[1])
            + coef_right[2] * old_right1[1]
            - old_right2[2];

        for mm in 0..3 {
            old_left2[mm] = old_left1[mm];
            old_left1[mm] = g.ez[mm];

            old_right2[mm] = old_right1[mm];
            old_right1[mm] = g.ez[end - 1 - mm];
        }
    };

    f
}
