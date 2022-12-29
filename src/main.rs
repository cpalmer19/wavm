mod bytecode;
mod value;
mod wasm_module;
mod vm;

fn main() {
    let _simple_two_func =
        "AGFzbQEAAAABCwJgAn9/AX9gAAF9AwMCAAEKFQILAEGZBhogACABagsHAEPD9UhACwAMBG5hbWUCBQIAAAEA";
    let simple_one_func_start =
        "AGFzbQEAAAABBAFgAAADAgEACAEACgoBCABBCkECbBoLABMEbmFtZQEHAQAEbWFpbgIDAQAA";
    
    let wasm_test = simple_one_func_start;
    
    let bytecode = base64::decode(wasm_test).unwrap();
    let result = wasm_module::load(&bytecode);

    match result {
        Ok(module) => {
            // dbg!(&module);
            vm::interpret(&module);
        }
        Err(err) => {
            eprintln!("{}", err.formatted());
        }
    }
}
