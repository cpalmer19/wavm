#![allow(dead_code)]

use crate::value::*;
use crate::bytecode;

pub fn load(bytecode: &[u8]) -> Result<WasmModule, WasmLoadError> {
    WasmModuleLoader::new(bytecode).load()
}

#[derive(Debug, Default)]
pub struct WasmModule {
    pub version: u32,
    pub types: Vec<FuncType>,
    pub functions: Vec<Function>,
    pub start_function: Option<usize>,
    pub code_section: Vec<u8>
}

pub struct WasmLoadError {
    byte: usize,
    msg: String,
}

impl WasmLoadError {
    pub fn formatted(&self) -> String {
        format!("Error at byte {:#04x}: {}", self.byte-1, self.msg)
    }
}

const WASM_BINARY_MAGIC: u32 = 0x0061_736d;

struct WasmModuleLoader<'a> {
    bytecode: &'a [u8],
    byte: usize,
    module: WasmModule,
    error: Option<WasmLoadError>,
}

impl<'a> WasmModuleLoader<'a> {
    fn new(bytecode: &'a [u8]) -> Self {
        Self {
            bytecode,
            byte: 0,
            module: WasmModule::default(),
            error: None,
        }
    }

    fn load(mut self) -> Result<WasmModule, WasmLoadError> {
        match self.preliminary() {
            Ok(version) => self.module.version = version,
            Err(msg) => self.error(&msg),
        }

        while self.error.is_none() && self.byte < self.bytecode.len() {
            match self.read_byte() {
                0x00 => self.custom(),
                0x01 => self.types(),
                0x02 => self.imports(),
                0x03 => self.functions(),
                0x04 => self.tables(),
                0x05 => self.memory(),
                0x06 => self.globals(),
                0x07 => self.exports(),
                0x08 => self.start(),
                0x09 => self.element(),
                0x0a => self.code(),
                0x0b => self.data(),
                b => self.error(&format!("Invalid section code {b:#04x}.")),
            }
        }

        match self.error {
            None => Ok(self.module),
            Some(err) => Err(err),
        }
    }

    fn preliminary(&mut self) -> Result<u32, String> {
        let magic = self.read_32();
        if magic != WASM_BINARY_MAGIC {
            return Err(String::from("Magic Number not found"));
        }
        Ok(self.read_32())
    }

    fn custom(&mut self) {
        self.skip_section();
    }

    fn types(&mut self) {
        self.read_size();       // section size

        let num_types = self.read_size();
        for _ in 0..num_types {
            if self.read_byte() != 0x60 {
                return self.error("Expected the function type 0x60");
            }

            let mut func_type = FuncType::default();

            let num_params = self.read_size();
            for _ in 0..num_params {
                match self.value_type() {
                    Ok(t) => func_type.params.push(t),
                    Err(msg) => return self.error(&msg),
                }
            }

            let num_results = self.read_size();
            for _ in 0..num_results {
                match self.value_type() {
                    Ok(t) => func_type.results.push(t),
                    Err(msg) => return self.error(&msg),
                }
            }

            self.module.types.push(func_type);
        }
    }

    fn imports(&mut self) {
        self.unimplemented_section();
    }

    fn functions(&mut self) {
        self.read_size();       // section size

        let num_funcs = self.read_size();
        for _ in 0..num_funcs {
            let type_idx = self.read_byte() as usize;
            self.module.functions.push(Function::new(type_idx));
        }
    }

    fn tables(&mut self) {
        self.unimplemented_section();
    }

    fn memory(&mut self) {
        self.unimplemented_section();
    }

    fn globals(&mut self) {
        self.unimplemented_section();
    }

    fn exports(&mut self) {
        self.unimplemented_section();
    }

    fn start(&mut self) {
        self.read_size();       // section size
        let function_idx = self.read_byte() as usize;
        self.module.start_function = Some(function_idx);
    }

    fn element(&mut self) {
        self.unimplemented_section();
    }

    fn code(&mut self) {
        let code_section_size = self.read_size();
        let code_start = self.byte;
        let code_end = code_start + code_section_size;

        // Create a copy of all the code into the module itself
        self.module.code_section.extend_from_slice(&self.bytecode[code_start..code_end]);

        let num_funcs = self.read_size();
        for i in 0..num_funcs {
            let body_size = self.read_size();

            // The Function refers to the code within the module, not the original bytecode
            let function = &mut self.module.functions[i];
            function.code_start = self.byte - code_start;
            function.code_len = body_size;
            self.byte += body_size;
        }
    }

    fn data(&mut self) {
        self.unimplemented_section();
    }

    fn skip_section(&mut self) {
        println!("Skipping section {:#04x}", self.bytecode[self.byte-1]);
        let size = self.read_size();
        self.byte += size;
    }

    fn unimplemented_section(&mut self) {
        let section_code = self.bytecode[self.byte-1];
        self.error(&format!("Section {section_code:#04x} is currently unimplemented"));
    }

    fn value_type(&mut self) -> Result<ValType, String> {
        match self.read_byte() {
            0x7f => Ok(ValType::I32),
            0x7e => Ok(ValType::I64),
            0x7d => Ok(ValType::F32),
            0x7c => Ok(ValType::F64),
            other => Err(format!("Invalid value type {other:#04x}")),
        }
    }

    fn name(&mut self) -> Result<String, String> {
        let len = self.read_size();
        let end = self.byte + (len as usize);
        match std::str::from_utf8(&self.bytecode[self.byte..end]) {
            Ok(s) => Ok(s.to_string()),
            Err(e) => Err(format!("{e}")),
        }
    }

    fn read_byte(&mut self) -> u8 {
        self.byte += 1;
        self.bytecode[self.byte - 1]
    }

    fn read_16(&mut self) -> u16 {
        self.byte += 2;
        bytecode::read::read_16(self.bytecode, self.byte - 2)
    }

    fn read_32(&mut self) -> u32 {
        self.byte += 4;
        bytecode::read::read_32(self.bytecode, self.byte - 4)
    }

    fn read_size(&mut self) -> usize {
        let (val, offset) = bytecode::read::read_size(self.bytecode, self.byte);
        self.byte += offset;
        val as usize
    }

    fn error(&mut self, msg: &str) {
        self.error = Some(WasmLoadError {
            byte: self.byte - 1,
            msg: msg.to_string(),
        });
    }
}