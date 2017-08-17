use std::rc::Rc;

use classload::ClassIdentifier;
use vm::isn::Instruction;

#[derive(Clone)]
pub struct VmFunction {
    name: String,
    pub isns: Vec<Instruction>,
    flags: u64,
}

#[derive(Clone)]
pub struct StackFrame {
    pub func: Rc<VmFunction>,
    pub next_isn: usize
}

impl StackFrame {

    pub fn new(func: Rc<VmFunction>) -> StackFrame {
        StackFrame { func: func, next_isn: 0 }
    }

    pub fn get_target_isn(&self) -> Instruction {
        self.func.isns
            .get(self.next_isn as usize)
            .cloned()
            .unwrap_or(Instruction::Return)
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

    pub fn top(&self) -> Option<StackFrame> {
        self.stack.last().cloned()
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

}
