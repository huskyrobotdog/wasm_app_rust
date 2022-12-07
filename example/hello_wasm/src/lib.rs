#[link(wasm_import_module = "std")]
extern "C" {
    fn rand_f64() -> f64;
}

#[link(wasm_import_module = "market")]
extern "C" {
    fn market_avg_price() -> f64;
    fn market_high_price() -> f64;
    fn market_low_price() -> f64;
}

#[no_mangle]
pub unsafe extern "C" fn run(a: i32, b: i32) -> f64 {
    let c = a + b;
    let d = rand_f64();
    let avg_price = market_avg_price();
    let high_price = market_high_price();
    let low_price = market_low_price();

    c as f64 + d + avg_price + high_price + low_price
}

