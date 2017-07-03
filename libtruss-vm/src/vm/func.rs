use std::rc::Rc;

use classload::ClassIdentifier;

use vm::isn::Instruction;

#[derive(Clone)]
pub struct VmFunction {
    name: String,
    clazz: Option<ClassIdentifier>, // Do we want this?
    isns: Vec<Instruction>,
    flags: u64,
}

#[allow(dead_code)]
struct StackFrame {
    func: Rc<VmFunction>,
    next_isn: usize
}

#[allow(dead_code)]
pub struct CallStack {
    stack: Vec<StackFrame>
}
