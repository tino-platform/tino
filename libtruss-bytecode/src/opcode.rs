
use model::{ClassId, FuncId};

pub enum OpCode {
    Nop,
    Add,
    Mul,
    Push(Literal),
    PushField(ClassId, String),
    //PushNew(ClassId), TODO
    Pop,
    Xchg,
    InvokeStatic(FuncId),
    InvokeVirtual(ClassId, String),
}

pub enum Literal {
    Dynamic(Vec<u8>),
    Integer(i64),
    Byte(u8)
}
