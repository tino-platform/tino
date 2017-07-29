use std::rc::Rc;

use std::ops::Index;

use classload::ClassIdentifier;
use vm::isn::Instruction;

#[derive(Clone)]
pub struct VmFunction {
    name: String,
    clazz: Option<ClassIdentifier>, // Do we want this?
    pub isns: Vec<Instruction>,
    flags: u64,
}

#[derive(Clone)]
pub struct StackFrame {
    pub func: Rc<VmFunction>,
    pub next_isn: u64
}

impl StackFrame {
    pub fn get_target_isn(&self) -> Option<Instruction> {
        self.func.isns.get(self.next_isn as usize).cloned()
    }
}

pub struct CallStack {
    stack: Vec<StackFrame>
}

impl CallStack {

    pub fn push(&mut self, f: StackFrame) {
        self.stack.push(f);
    }

    pub fn pop(&mut self) -> Option<StackFrame> {
        self.stack.pop()
    }

    pub fn len(&self) -> u64 {
        self.stack.len() as u64
    }

    pub fn handle_return(&mut self) {
        self.stack.pop();
        match self.stack.pop() {
            Some(mut f) => f.next_isn += 1,
            None => {} // ???
        }
    }

}
