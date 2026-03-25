use rand::*;

pub fn random_double() -> f64 {
    rng().random::<f64>()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}

pub fn random_int(min: i64, max: i64) -> i64 {
    random_double_range(min as f64, (max + 1) as f64) as i64
}
