pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;
use rand::Rng;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * PI / 180.0;
}

pub fn random_double1() -> f64 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0.0, 1.0);
}

pub fn random_double2(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    return rng.gen_range(min, max);
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }
    if x > max {
        return max;
    }
    return x;
}
