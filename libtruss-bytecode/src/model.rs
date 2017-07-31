
use opcode::OpCode;

#[derive(Clone)]
pub struct ClassId {
    class_module: String,
    name: String,
    generic_names: Vec<String>
}

pub struct ClassDef {
    id: ClassId,
    members: Vec<FieldType>,
    funcs: Vec<FunctionDef>
}

enum TypeReference {
    Specific(ClassId, Vec<Box<TypeReference>>),
    Generic(String)
}

enum FieldType {
    Byte,
    Integer,
    Reference(TypeReference),
    Dynamic(Option<usize>)
}

pub struct FunctionDef {
    generic_names: Vec<String>,
    opcode: Vec<OpCode> // TODO Figure out how these work.
}
