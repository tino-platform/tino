use process::Process;
use process::StackValue;
use vm::Instruction;

pub struct PushLiteralIsn {
    val: StackValue
}

impl PushLiteralIsn {
    pub fn new(val: StackValue) -> PushLiteralIsn {
        PushLiteralIsn { val: val }
    }
}

impl Instruction for PushLiteralIsn {
    fn exec(&self, p: &mut Process) -> Result<(), &str> {
        p.stack.borrow_mut().push(self.val);
        Ok(())
    }
}

pub struct PopIsn;

impl PopIsn {
    pub fn new() -> PopIsn {
        PopIsn { }
    }
}

impl Instruction for PopIsn {
    fn exec(&self, p: &mut Process) -> Result<(), &str> {
        match p.stack.borrow_mut().pop() {
            Some(_) => Ok(()),
            None => Err("stack empty")
        }
    }
}

pub struct DupIsn;

impl DupIsn {
    pub fn new() -> DupIsn {
        DupIsn {}
    }
}

impl Instruction for DupIsn {
    fn exec(&self, p: &mut Process) -> Result<(), &str> {

        let mut stack = p.stack.borrow_mut();

        let top = match stack.len() {
            0 => None,
            n => Some(stack[n - 1])
        };

        match top {
            Some(v) => {
                stack.push(v);
                Ok(())
            },
            None => Err("stack is empty")
        }

    }
}

pub struct XchgIsn;

impl XchgIsn {
    pub fn new() -> XchgIsn {
        XchgIsn { }
    }
}

impl Instruction for XchgIsn {
    fn exec(&self, p: &mut Process) -> Result<(), &str> {

        let mut stack = p.stack.borrow_mut();

        if stack.len() >= 2 {

            let a = stack.pop().unwrap();
            let b = stack.pop().unwrap();

            stack.push(a);
            stack.push(b);

            Ok(())

        } else {
            Err("not enough elements on stack")
        }

    }
}
