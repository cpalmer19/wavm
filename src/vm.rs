#![allow(dead_code)]

use crate::bytecode::{self, op::*};
use crate::value::Value;
use crate::wasm_module::WasmModule;

pub struct Vm<'a> {
    module: &'a WasmModule,
    code: &'a [u8],
    ip: usize,
    stack: Vec<Value>,
}

pub fn interpret(module: &WasmModule) {
    if module.start_function.is_none() {
        return;
    }
    let code = {
        let function = &module.functions[module.start_function.unwrap()];
        let code_start = function.code_start;
        let code_end = code_start + function.code_len;
        &module.code_section[code_start..code_end]
    };

    let mut vm = Vm {
        module,
        code,
        ip: 0,
        stack: Vec::new(),
    };
    vm.interpret();
}

//TODO Call Frames
impl<'a> Vm<'a> {
    pub fn interpret(&mut self) {
        let _local_count = self.read_byte();

        macro_rules! op_cmp {
            ($pop_func: ident, $op: tt) => {{
                let rhs = self.$pop_func();
                let lhs = self.$pop_func();
                let result = if lhs $op rhs { 1i32 } else { 0i32 };
                self.push_i32(result);
            }};
            ($pop_func: ident $op: tt $rhs: expr) => {{
                let lhs = self.$pop_func();
                let result = if lhs $op $rhs { 1i32 } else { 0i32 };
                self.push_i32(result);
            }};
        }
        macro_rules! op_binary_simple {
            ($pop_func: ident, $push_func: ident, $op: tt) => {{
                let rhs = self.$pop_func();
                let lhs = self.$pop_func();
                self.$push_func(lhs $op rhs);
            }};
        }

        self.stack.clear();

        loop {
            {
                // Stack Debugging
                for v in &self.stack {
                    print!("[{v:?}]");
                }
                print!("\n");
            }

            if self.ip >= self.code.len() {
                break;
            }

            match self.read_byte() {
                //Control Flow
                NOP => {}
                END => {}
                DROP => {
                    self.stack.pop().expect("Empty stack");
                }
                // Constants
                I32_CONST => {
                    let value = self.read_i32();
                    self.push_i32(value);
                }
                I64_CONST => {
                    let value = self.read_i64();
                    self.push_i64(value);
                }
                F32_CONST => {
                    let value = self.read_f32();
                    self.push_f32(value);
                }
                F64_CONST => {
                    let value = self.read_f64();
                    self.push_f64(value);
                }
                // Comparisons
                I32_EQZ => op_cmp!(pop_i32 == 0),
                I32_EQ => op_cmp!(pop_i32, ==),
                I32_NE => op_cmp!(pop_i32, !=),
                I32_LT_S => op_cmp!(pop_i32, <),
                I32_LT_U => op_cmp!(pop_u32, <),
                I32_GT_S => op_cmp!(pop_i32, >),
                I32_GT_U => op_cmp!(pop_u32, >),
                // I32_LE_S => op_cmp!(pop_i32, <=),
                I32_LE_U => op_cmp!(pop_u32, <=),
                I32_GE_S => op_cmp!(pop_i32, >=),
                I32_GE_U => op_cmp!(pop_u32, >=),
                I64_EQZ => op_cmp!(pop_i64 == 0),
                I64_EQ => op_cmp!(pop_i64, ==),
                I64_NE => op_cmp!(pop_i64, !=),
                I64_LT_S => op_cmp!(pop_i64, <),
                I64_LT_U => op_cmp!(pop_u64, <),
                I64_GT_S => op_cmp!(pop_i64, >),
                I64_GT_U => op_cmp!(pop_u64, >),
                // I64_LE_S => op_cmp!(pop_i64, <=),
                I64_LE_U => op_cmp!(pop_u64, <=),
                I64_GE_S => op_cmp!(pop_i64, >=),
                I64_GE_U => op_cmp!(pop_u64, >=),
                F32_EQ => op_cmp!(pop_f32, ==),
                F32_NE => op_cmp!(pop_f32, !=),
                F32_LT => op_cmp!(pop_f32, <),
                F32_GT => op_cmp!(pop_f32, >),
                F32_LE => op_cmp!(pop_f32, <=),
                F32_GE => op_cmp!(pop_f32, >=),
                F64_EQ => op_cmp!(pop_f64, ==),
                F64_NE => op_cmp!(pop_f64, !=),
                F64_LT => op_cmp!(pop_f64, <),
                F64_GT => op_cmp!(pop_f64, >),
                F64_LE => op_cmp!(pop_f64, <=),
                F64_GE => op_cmp!(pop_f64, >=),
                // i32 Arithmetic
                I32_ADD => op_binary_simple!(pop_i32, push_i32, +),
                I32_SUB => op_binary_simple!(pop_i32, push_i32, -),
                I32_MUL => op_binary_simple!(pop_i32, push_i32, *),
                I32_DIV_S => op_binary_simple!(pop_i32, push_i32, /),
                I32_DIV_U => op_binary_simple!(pop_u32, push_u32, +),
                I32_REM_S => op_binary_simple!(pop_i32, push_i32, %),
                I32_REM_U => op_binary_simple!(pop_u32, push_u32, %),
                I32_AND => op_binary_simple!(pop_i32, push_i32, &),
                I32_OR => op_binary_simple!(pop_i32, push_i32, |),
                I32_XOR => op_binary_simple!(pop_i32, push_i32, ^),
                I32_SHL => op_binary_simple!(pop_i32, push_i32, <<),
                I32_SHR_S => op_binary_simple!(pop_i32, push_i32, >>),
                I32_SHR_U => op_binary_simple!(pop_u32, push_u32, >>),
                op => unimplemented!("Instruction {op:#04x} not yet implemented"),
            }
        }

        // If there is a return value it is left on the stack. Print it
        if let Some(value) = self.stack.pop() {
            println!("{value:?}");
        }
    }

    fn read_byte(&mut self) -> u8 {
        self.ip += 1;
        self.code[self.ip - 1]
    }

    fn read_i32(&mut self) -> i32 {
        let (num, offset) = bytecode::read::read_i32(self.code, self.ip);
        self.ip += offset;
        num as i32
    }

    fn read_i64(&mut self) -> i64 {
        let (num, offset) = bytecode::read::read_i64(self.code, self.ip);
        self.ip += offset;
        num
    }

    fn read_f32(&mut self) -> f32 {
        self.ip += 4;
        bytecode::read::read_f32(self.code, self.ip - 4)
    }

    fn read_f64(&mut self) -> f64 {
        self.ip += 8;
        bytecode::read::read_f64(self.code, self.ip - 4)
    }

    fn pop_u32(&mut self) -> u32 {
        unsafe { std::mem::transmute(self.pop_i32()) }
    }

    fn pop_u64(&mut self) -> u64 {
        unsafe { std::mem::transmute(self.pop_i64()) }
    }

    fn pop_i32(&mut self) -> i32 {
        match self.stack.pop() {
            Some(Value::I32(v)) => v,
            _ => panic!("Runtime Error: Expected i32"),
        }
    }

    fn pop_i64(&mut self) -> i64 {
        match self.stack.pop() {
            Some(Value::I64(v)) => v,
            _ => panic!("Runtime Error: Expected i64"),
        }
    }

    fn pop_f32(&mut self) -> f32 {
        match self.stack.pop() {
            Some(Value::F32(v)) => v,
            _ => panic!("Runtime Error: Expected f32"),
        }
    }

    fn pop_f64(&mut self) -> f64 {
        match self.stack.pop() {
            Some(Value::F64(v)) => v,
            _ => panic!("Runtime Error: Expected f64"),
        }
    }

    fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    fn push_u32(&mut self, v: u32) {
        self.push_i32(unsafe { std::mem::transmute(v) });
    }

    fn push_u64(&mut self, v: u64) {
        self.push_i64(unsafe { std::mem::transmute(v) });
    }

    fn push_i32(&mut self, v: i32) {
        self.stack.push(Value::I32(v));
    }

    fn push_i64(&mut self, v: i64) {
        self.stack.push(Value::I64(v));
    }

    fn push_f32(&mut self, v: f32) {
        self.stack.push(Value::F32(v));
    }

    fn push_f64(&mut self, v: f64) {
        self.stack.push(Value::F64(v));
    }
}