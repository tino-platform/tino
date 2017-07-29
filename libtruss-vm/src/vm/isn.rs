use process::Process;
use process::heap::StackValue;

#[derive(Copy, Clone)]
pub enum Instruction {
    Nop,
    //PushLiteral(StackValue),
    Pop,
    Dup,
    Xchg,
    Add,
    Mul,
    LogicalAnd,
    LogicalOr,
    LogicalXor,
    LogicalNot,
    RelativeJump(i16),
    CondExec,
    Return,
    Detach
}

pub enum IsnSegue {
    Next,
    RelJump(i16),
    Return,
    Detach
}

impl Instruction {
    fn execute(&self, p: &mut Process) -> Result<IsnSegue, &str> {

        use process::heap::HeapValue::*;
        use self::IsnSegue::*;

        match self {

            &Instruction::Nop => Ok(Next), // Always succeeds

            /*&Instruction::PushLiteral(v) => {
                p.stack.push(v);
                Ok(Next)
            },*/

            &Instruction::Pop => match p.stack.pop() {
                Some(_) => Ok(Next),
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
                        Ok(Next)
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

                    Ok(Next)

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
                                Ok(Next)
                            },

                            (Integer(x), Byte(y)) => {
                                p.stack.push(Integer(x + y as i64));
                                Ok(Next)
                            },

                            (Byte(x), Integer(y)) => {
                                p.stack.push(Integer(x as i64 + y));
                                Ok(Next)
                            },

                            (Byte(x), Byte(y)) => {
                                p.stack.push(Byte(x + y));
                                Ok(Next)
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
                                Ok(Next)
                            },

                            (Integer(x), Byte(y)) => {
                                p.stack.push(Integer(x * y as i64));
                                Ok(Next)
                            },

                            (Byte(x), Integer(y)) => {
                                p.stack.push(Integer(x as i64 * y));
                                Ok(Next)
                            },

                            (Byte(x), Byte(y)) => {
                                p.stack.push(Byte(x * y));
                                Ok(Next)
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

            &Instruction::LogicalAnd => helper_boolean_op(p, &|a, b| a && b),
            &Instruction::LogicalOr => helper_boolean_op(p, &|a, b| a || b),
            &Instruction::LogicalXor => helper_boolean_op(p, &|a, b| a ^ b),

            &Instruction::LogicalNot => {

                let val = p.stack.pop();

                match val {
                    Some(v) => match v {
                        Boolean(x) => {
                            p.stack.push(Boolean(!x));
                            Ok(Next)
                        },
                        _ => {
                            p.stack.push(v);
                            Err("not a boolean")
                        }
                    },
                    _ => Err("stack empty")
                }

            },

            &Instruction::RelativeJump(d) => Ok(RelJump(d)),
            &Instruction::Return => Ok(Return),

            &Instruction::CondExec => {

                let top = p.stack.pop();

                match top {
                    Some(v) => match v {
                        Boolean(b) => Ok(if b { Next } else { RelJump(2) }),
                        _ => {
                            p.stack.push(v);
                            Err("not a boolean")
                        }
                    },
                    None => Err("stack empty")
                }

            },

            &Instruction::Detach => Ok(Detach)

        }
    }
}

fn helper_boolean_op(p: &mut Process, exp: &Fn(bool, bool) -> bool) -> Result<IsnSegue, &'static str> {

    use process::heap::HeapValue::*;
    use self::IsnSegue::*;

    let a = p.stack.pop();
    let b = p.stack.pop();

    match (a, b) {
        (Some(x), Some(y)) => {

            match (x, y) {

                (Boolean(x), Boolean(y)) => {
                    p.stack.push(Boolean(exp(x, y)));
                    Ok(Next)
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

pub fn vm_exec(p: &mut Process, isns: u64) -> Result<u64, (u64, &str)> {

    // FIXME Ugly nested matches can be made into a more functional thing.

    let mut execed = 0;

    while execed < isns {

        match p.call_stack.pop() {
            Some(frame) => {
                let mut f = frame.clone();
                match f.get_target_isn() {
                    Some(i) => match i.execute(p) {
                        Ok(segue) => {
                            match segue {
                                IsnSegue::Next => {
                                    f.next_isn += 1;
                                    p.call_stack.push(f);
                                    execed += 1;
                                },
                                IsnSegue::RelJump(diff) => {
                                    f.next_isn = ((f.next_isn as i64) + (diff as i64)) as u64;
                                    p.call_stack.push(f);
                                    execed += 1;
                                },
                                IsnSegue::Return => execed += 1, // Don't push it back onto the stack.
                                IsnSegue::Detach => {
                                    f.next_isn += 1;
                                    p.call_stack.push(f);
                                    return Ok(execed + 1);
                                }
                            }
                        },
                        Err(r) => return Err((execed + 1, r))
                    },
                    None => {} // Don't push it back onto the stack and don't increment execed.
                }
            },
            None => return Err((execed, "process exited"))
        }

        execed += 1;

    }

    return Ok(execed);

}
