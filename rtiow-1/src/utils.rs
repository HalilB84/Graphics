use rand::Rng;

pub fn random_double() -> f64 {
    let mut rng = rand::rng();
    rng.random::<f64>()
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double()
}
