
use opcode::OpCode;

#[derive(Clone)]
pub struct ClassId {
    class_module: String,
    name: String,
    generic_names: Vec<String>
}

#[derive(Clone)]
pub struct FuncId {
    func_module: String,
    class_name: Option<ClassId>,
    name: String
}

pub struct ClassDef {
    id: ClassId,
    members: Vec<FieldType>,
    funcs: Vec<FunctionDef>
}

enum TypeReference {
    Specific(ClassId, Box<Vec<TypeReference>>),
    Generic(String)
}

enum FieldType {
    Byte,
    Integer,
    Reference(TypeReference),
    Dynamic(Option<usize>)
}

pub struct FunctionDef {
    id: FuncId,
    generic_binds: Vec<String>,
    ops: Vec<OpCode> // TODO Figure out how these work.
}
