#![allow(dead_code)]

pub enum Value {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    V128(i128),
    RefNull(RefType),
}

pub enum RefType {
    FuncRef,
    ExternRef,
}

#[derive(Debug)]
pub enum ValType {
    I32 = 0x7f,
    I64 = 0x7e,
    F32 = 0x7d,
    F64 = 0x7c,
}

#[derive(Debug)]
pub enum TypeIndex {
    Val(ValType),
    Index(u32),
}

#[derive(Debug, Default)]
pub struct FuncType {
    pub params: Vec<ValType>,
    pub results: Vec<ValType>,
}

#[derive(Debug)]
pub struct Function {
    pub functype: usize,
    pub code_start: usize,
    pub code_len: usize,
}

impl Function {
    pub fn new(functype_idx: usize) -> Self {
        Self {
            functype: functype_idx,
            code_start: 0,
            code_len: 0,
        }
    }
}