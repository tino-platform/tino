use std::rc::Rc;

use vm::func::VmFunction;
use vm::func::StackFrame;

use process::Process;
use process::heap::{HeapIndex, HeapObject};

use obj::VmClass;

#[derive(Copy, Clone)]
pub enum StackValue {
    Object(HeapIndex),
    Dynamic(HeapIndex),
    Integer(i64),
    Byte(u8),
    Boolean(bool)
}

#[derive(Clone)]
pub enum Instruction {
    Nop,
    PushLitInteger(i64),
    PushLitByte(u8),
    PushLitBoolean(bool),
    PushNewObject(Rc<VmClass>),
    Pop,
    Dup,
    Xchg,
    Add,
    Mul,
    LogicalAnd,
    LogicalOr,
    LogicalXor,
    LogicalNot,
    RelativeJump(isize),
    CondExec,
    InvokeStd(Rc<VmFunction>),
    InvokeVirtual(Rc<VmClass>, usize),
    Return,
    Detach
}

pub enum IsnSegue {
    Next,
    RelJump(isize),
    Return,
    Detach
}

#[derive(Clone, Debug)]
pub enum ExecError {
    NotImplementedYet,
    StackUnderflow,
    TypeError,
}

impl Instruction {
    fn execute(&self, p: &mut Process) -> Result<IsnSegue, ExecError> {

        use self::StackValue::*;
        use self::ExecError::*;
        use self::IsnSegue::*;

        match self {

            &Instruction::Nop => Ok(Next), // Always succeeds

            &Instruction::PushLitInteger(v) => {
                p.stack.push(Integer(v));
                Ok(Next)
            },

            &Instruction::PushLitByte(v) => {
                p.stack.push(Byte(v));
                Ok(Next)
            },

            &Instruction::PushLitBoolean(v) => {
                p.stack.push(Boolean(v));
                Ok(Next)
            },

            &Instruction::PushNewObject(ref class) => {

                let mut created = HeapObject::new(class.clone());

                // TODO Install all the fields, as necessary.

                let idx = unimplemented!(); // TODO Add the object to the heap and get a reference.

                p.stack.push(Object(idx));
                Ok(Next)

            }

            &Instruction::Pop => match p.stack.pop() {
                Some(_) => Ok(Next),
                None => Err(StackUnderflow),
            },

            &Instruction::Dup => {

                let top = match p.stack.len() {
                    0 => None,
                    n => Some(p.stack[n - 1]),
                };

                match top {
                    Some(v) => {
                        p.stack.push(v);
                        Ok(Next)
                    },
                    None => Err(StackUnderflow),
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
                    Err(StackUnderflow)
                }

            },

            &Instruction::Add => helper_intbyte_op(p, &|a, b| a + b, &|a, b| a + b),
            &Instruction::Mul => helper_intbyte_op(p, &|a, b| a * b, &|a, b| a * b),
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
                            Err(TypeError)
                        }
                    },
                    _ => Err(StackUnderflow),
                }

            },

            &Instruction::RelativeJump(d) => Ok(RelJump(d)),

            &Instruction::InvokeStd(ref func) => {

                match p.call_stack.top() {
                    Some(mut e) => {
                        e.next_isn += 1;
                        p.call_stack.push(e);
                    },
                    None => {}
                };

                p.call_stack.push(StackFrame::new(func.clone()));
                Ok(Next)

            },

            &Instruction::InvokeVirtual(ref class, usize) => {

                match p.call_stack.top() {
                    Some(mut e) => {
                        e.next_isn += 1; // Are we doing this right?
                        p.call_stack.push(e);
                    },
                    None => {}
                };

                let top = p.stack.pop();

                match top {
                    Some(v) => match v {
                        Object(_) => {
                            // TODO Call the function.
                            Ok(Next)
                        },
                        _ => {
                            p.stack.push(v);
                            Err(TypeError)
                        }
                    },
                    None => Err(StackUnderflow)
                }

            }

            &Instruction::Return => Ok(Return),

            &Instruction::CondExec => {

                let top = p.stack.pop();

                match top {
                    Some(v) => match v {
                        Boolean(b) => Ok(if b { Next } else { RelJump(2) }),
                        _ => {
                            p.stack.push(v);
                            Err(TypeError)
                        }
                    },
                    None => Err(StackUnderflow),
                }

            },

            &Instruction::Detach => Ok(Detach)

        }
    }
}

fn helper_intbyte_op(p: &mut Process, byte_exp: &Fn(u8, u8) -> u8, int_exp: &Fn(i64, i64) -> i64) -> Result<IsnSegue, ExecError> {

    use self::StackValue::*;
    use self::IsnSegue::*;
    use self::ExecError::*;

    let a = p.stack.pop();
    let b = p.stack.pop();

    match (a, b) {
        (Some(x), Some(y)) => {

            match (x, y) {

                (Integer(x), Integer(y)) => {
                    p.stack.push(Integer(int_exp(x, y)));
                    Ok(Next)
                },

                (Integer(x), Byte(y)) => {
                    p.stack.push(Integer(int_exp(x, y as i64)));
                    Ok(Next)
                },

                (Byte(x), Integer(y)) => {
                    p.stack.push(Integer(int_exp(x as i64, y)));
                    Ok(Next)
                },

                (Byte(x), Byte(y)) => {
                    p.stack.push(Byte(byte_exp(x, y)));
                    Ok(Next)
                },

                _ => Err(TypeError),
            }

        },
        (Some(x), None) => {
            p.stack.push(x);
            Err(StackUnderflow)
        },
        _ => Err(StackUnderflow)
    }

}

fn helper_boolean_op(p: &mut Process, exp: &Fn(bool, bool) -> bool) -> Result<IsnSegue, ExecError> {

    use self::StackValue::*;
    use self::IsnSegue::*;
    use self::ExecError::*;

    let a = p.stack.pop();
    let b = p.stack.pop();

    match (a, b) {
        (Some(x), Some(y)) => {

            match (x, y) {

                (Boolean(x), Boolean(y)) => {
                    p.stack.push(Boolean(exp(x, y)));
                    Ok(Next)
                },

                _ => Err(TypeError),

            }

        },
        (Some(x), None) => {
            p.stack.push(x);
            Err(StackUnderflow)
        },
        _ => Err(StackUnderflow),
    }

}

#[allow(dead_code)]
fn fizzbuzz(x: i32) -> String {
    match (x % 3, x % 5) {
        (0, 0) => "fizzbuzz".into(),
        (0, _) => "fizz".into(),
        (_, 0) => "buzz".into(),
        _ => format!("{}", x),
    }
}

pub enum VmExecError {
    IsnError(ExecError),
    ProcessExited,
}

pub fn vm_exec(p: &mut Process, isns: u64) -> Result<u64, (u64, VmExecError)> {

    use self::VmExecError::*;

    let mut execed = 0;

    while execed < isns {

        let frame = match p.call_stack.top() {
            Some(frame) => frame,
            None => return Err((execed, ProcessExited)),
        };

        let ii = frame.get_target_isn();
        match ii.execute(p) {
            Ok(segue) => {
                execed += 1;
                match segue {
                    IsnSegue::Next => {
                        let mut sf = p.call_stack.pop().unwrap();
                        sf.next_isn += 1;
                        p.call_stack.push(sf);
                    },
                    IsnSegue::RelJump(diff) => {
                        let mut sf = p.call_stack.pop().unwrap();
                        sf.next_isn = ((sf.next_isn as isize) + diff) as usize;
                        p.call_stack.push(sf)
                    },
                    IsnSegue::Return => {
                        p.call_stack.pop();
                    },
                    IsnSegue::Detach => {
                        let mut sf = p.call_stack.pop().unwrap();
                        sf.next_isn += 1;
                        p.call_stack.push(sf);
                        return Ok(execed + 1);
                    }
                }
            },
            Err(r) => return Err((execed + 1, IsnError(r))),
        }
    }

    return Ok(execed);
}
