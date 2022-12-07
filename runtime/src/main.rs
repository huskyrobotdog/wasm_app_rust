use wasmtime::*;

mod init;
mod types;

fn main() -> anyhow::Result<()> {
    init::log();

    let engine = Engine::default();
    let mut store = Store::new(
        &engine,
        types::State {
            app_name: "MyAPP".to_string(),
        },
    );

    let mut linker: Linker<types::State> = Linker::new(&engine);
    linker.func_wrap("host", "rand_f64", sdk_std::rand::f64)?;

    let module = Module::new(
        &engine,
        include_bytes!("../../target/wasm32-wasi/release/hello_wasm.wasm"),
    )?;
    let instance = linker.instantiate(&mut store, &module)?;

    let run = instance.get_typed_func::<(i32, i32), f64, _>(&mut store, "run")?;

    let result = run.call(&mut store, (1, 3))?;
    log::info!("result: {}", result);
    Ok(())
}
