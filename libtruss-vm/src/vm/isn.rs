use process::{HeapValue, Process};

#[derive(Clone)]
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
                p.stack.push(v);
                Ok(())
            },

            &Instruction::Pop => match p.stack.pop() {
                Some(_) => Ok(()),
                None => Err("stack empty")
            },

            &Instruction::Dup => {

                let top = match p.stack.len() {
                    0 => None,
                    n => Some(p.stack[n - 1])
                };

                match top {
                    Some(v) => {
                        p.stack.push(v);
                        Ok(())
                    },
                    None => Err("stack is empty")
                }

            },

            &Instruction::Xchg => {

                if p.stack.len() >= 2 {

                    let a = p.stack.pop().unwrap();
                    let b = p.stack.pop().unwrap();

                    p.stack.push(a);
                    p.stack.push(b);

                    Ok(())

                } else {
                    Err("not enough elements on stack")
                }

            }

        }
    }
}
