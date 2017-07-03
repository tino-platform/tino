use classload::ClassIdentifier;
use vm::isn::Instruction;

pub struct VmFunction {
    pub name: String,
    pub clazz: Option<ClassIdentifier>, // Do we want this?
    pub isns: Vec<Instruction>
}
