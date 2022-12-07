use rand::Rng;

pub fn f64() -> f64 {
    let mut rng = rand::thread_rng();
    let v: f64 = rng.gen();
    v
}
