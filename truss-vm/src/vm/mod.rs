pub mod stack;

use process::Process;

pub trait Instruction {
    fn exec(&self, p: &mut Process) -> Result<(), &str>;
}
