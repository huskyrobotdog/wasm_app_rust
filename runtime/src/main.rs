use anyhow::Result;
use sdk::rand;
use wasmtime::*;

mod init;

fn main() -> Result<()> {
    init::log();

    let engine = Engine::default();
    let mut store = Store::new(&engine, ());

    let mut linker: Linker<()> = Linker::new(&engine);
    linker.func_wrap("host", "rand_f64", rand::f64)?;

    let module = Module::new(&engine, &[])?;
    let instance = linker.instantiate(&mut store, &module)?;

    let run = instance.get_typed_func::<(), (), _>(&mut store, "run")?;

    run.call(&mut store, ())?;
    Ok(())
}
