use process::Process;
use process::heap::StackValue;

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

pub enum IsnSegue {
    Next,
    RelJump(i16)
}

impl Instruction {
    pub fn execute(&self, p: &mut Process) -> Result<IsnSegue, &str> {

        use process::heap::HeapValue::*;
        use self::IsnSegue::*;

        match self {

            &Instruction::Nop => Ok(Next), // Always succeeds

            &Instruction::PushLiteral(v) => {
                p.stack.push(v);
                Ok(Next)
            },

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

            }

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
