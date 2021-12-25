// src/ricker.rs

/// Ricker wavelet.
pub fn ricker(time: f64, location: f64, cdtds: f64, ppw: f64) -> f64 {
    let arg = std::f64::consts::PI * ((cdtds * time - location) / ppw - 1.0);
    let arg = arg * arg;
    (1.0 - 2.0 * arg) * (-arg).exp()
}
