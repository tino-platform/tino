use std::cell::Cell;

pub mod heap;

use classload::ClassPool;
use process::heap::{HeapObject, StackValue};
use vm::func::CallStack;

#[allow(dead_code)]
pub struct Process {
    pub name: String,
    classpool: ClassPool,
    pub heap: Vec<Cell<HeapObject>>,
    pub stack: Vec<StackValue>,
    pub call_stack: CallStack
}
