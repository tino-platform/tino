use process::Process;
use process::heap::StackValue;
use process::heap::HeapValue::*;

#[derive(Clone)]
pub enum Instruction {
    Nop,
    PushLiteral(StackValue),
    Pop,
    Dup,
    Xchg,
    Add,
    Mul,
    LogicalAnd,
    LogicalOr,
    LogicalXor,
    LogicalNot
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

            },

            &Instruction::Add => {

                let a = p.stack.pop();
                let b = p.stack.pop();

                match (a, b) {
                    (Some(x), Some(y)) => {

                        match (x, y) {

                            (Integer(x), Integer(y)) => {
                                p.stack.push(Integer(x + y));
                                Ok(())
                            },

                            (Integer(x), Byte(y)) => {
                                p.stack.push(Integer(x + y as i64));
                                Ok(())
                            },

                            (Byte(x), Integer(y)) => {
                                p.stack.push(Integer(x as i64 + y));
                                Ok(())
                            },

                            (Byte(x), Byte(y)) => {
                                p.stack.push(Byte(x + y));
                                Ok(())
                            },

                            _ => Err("incompatible stack item types")

                        }

                    },
                    (Some(x), None) => {
                        p.stack.push(x);
                        Err("stack empty")
                    },
                    _ => Err("stack empty")
                }

            }

            &Instruction::Mul => {

                let a = p.stack.pop();
                let b = p.stack.pop();

                match (a, b) {
                    (Some(x), Some(y)) => {

                        match (x, y) {

                            (Integer(x), Integer(y)) => {
                                p.stack.push(Integer(x * y));
                                Ok(())
                            },

                            (Integer(x), Byte(y)) => {
                                p.stack.push(Integer(x * y as i64));
                                Ok(())
                            },

                            (Byte(x), Integer(y)) => {
                                p.stack.push(Integer(x as i64 * y));
                                Ok(())
                            },

                            (Byte(x), Byte(y)) => {
                                p.stack.push(Byte(x * y));
                                Ok(())
                            },

                            _ => Err("incompatible stack item types")

                        }

                    },
                    (Some(x), None) => {
                        p.stack.push(x);
                        Err("stack empty")
                    },
                    _ => Err("stack empty")
                }

            }

            &Instruction::LogicalAnd => {

                let a = p.stack.pop();
                let b = p.stack.pop();

                match (a, b) {
                    (Some(x), Some(y)) => {

                        match (x, y) {

                            (Boolean(x), Boolean(y)) => {
                                p.stack.push(Boolean(x && y));
                                Ok(())
                            },

                            _ => Err("incompatible stack item types")

                        }

                    },
                    (Some(x), None) => {
                        p.stack.push(x);
                        Err("stack empty")
                    },
                    _ => Err("stack empty")
                }

            }

            &Instruction::LogicalOr => {

                let a = p.stack.pop();
                let b = p.stack.pop();

                match (a, b) {
                    (Some(x), Some(y)) => {

                        match (x, y) {

                            (Boolean(x), Boolean(y)) => {
                                p.stack.push(Boolean(x || y));
                                Ok(())
                            },

                            _ => Err("incompatible stack item types")

                        }

                    },
                    (Some(x), None) => {
                        p.stack.push(x);
                        Err("stack empty")
                    },
                    _ => Err("stack empty")
                }

            }

            &Instruction::LogicalXor => {

                let a = p.stack.pop();
                let b = p.stack.pop();

                match (a, b) {
                    (Some(x), Some(y)) => {

                        match (x, y) {

                            (Boolean(x), Boolean(y)) => {
                                p.stack.push(Boolean(x ^ y));
                                Ok(())
                            },

                            _ => Err("incompatible stack item types")

                        }

                    },
                    (Some(x), None) => {
                        p.stack.push(x);
                        Err("stack empty")
                    },
                    _ => Err("stack empty")
                }

            }

            &Instruction::LogicalNot => {

                let val = p.stack.pop();

                match val {
                    Some(v) => match v {
                        Boolean(x) => {
                            p.stack.push(Boolean(!x));
                            Ok(())
                        },
                        _ => {
                            p.stack.push(v);
                            Err("not a boolean")
                        }
                    },
                    _ => Err("stack empty")
                }

            }

        }
    }
}
