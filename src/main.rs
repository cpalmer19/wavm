mod bytecode;
mod value;
mod wasm_module;
mod leb128;

use crate::wasm_module::WasmModuleDecoder;

fn main() {
    let _simple_two_func =
        "AGFzbQEAAAABCwJgAn9/AX9gAAF9AwMCAAEKFQILAEGZBhogACABagsHAEPD9UhACwAMBG5hbWUCBQIAAAEA";
    let simple_one_func_start =
        "AGFzbQEAAAABBAFgAAADAgEACAEACgoBCABBCkECTxoLABMEbmFtZQEHAQAEbWFpbgIDAQAA";
    
    let wasm_test = simple_one_func_start;
    
    let bytecode = base64::decode(wasm_test).unwrap();
    let module = WasmModuleDecoder::new(&bytecode).decode();

    if let Some(module) = module {
        dbg!(&module);

        // TODO Run the start function in the VM
        // if let Some(idx) = module.start_function {
        // }
    }
}
