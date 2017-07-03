use std::cell::Cell;

pub mod heap;

use classload::{ClassPool, DummySource};
use process::heap::{HeapObject, StackValue};
use vm::func::CallStack;

#[allow(dead_code)]
pub struct Process {
    pub name: String,
    classpool: ClassPool<DummySource>,
    pub heap: Vec<Cell<HeapObject>>,
    pub stack: Vec<StackValue>,
    call_stack: CallStack
}
