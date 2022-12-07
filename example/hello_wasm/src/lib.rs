#[link(wasm_import_module = "host")]
extern "C" {
    fn rand_f64() -> f64;
}

#[no_mangle]
pub unsafe extern "C" fn run(a: i32, b: i32) -> f64 {
    let c = a + b;
    let d = rand_f64();
    c as f64 + d
}
