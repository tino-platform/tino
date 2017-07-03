use process::{HeapValue, Process};

pub enum Instruction {
    Nop,
    PushLiteral(HeapValue),
    Pop,
    Dup,
    Xchg
}

impl Instruction {
    pub fn execute(&self, p: &mut Process) -> Result<(), &str> {
        match self {

            &Instruction::Nop => Ok(()), // Always succeeds

            &Instruction::PushLiteral(v) => {
                p.stack.borrow_mut().push(v);
                Ok(())
            },

            &Instruction::Pop => match p.stack.borrow_mut().pop() {
                Some(_) => Ok(()),
                None => Err("stack empty")
            },

            &Instruction::Dup => {

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

            },

            &Instruction::Xchg => {

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
    }
}
