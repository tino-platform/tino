pub enum VmType {
    Object(Box<VmClass>),
    Str,
    Uint64,
    Int64,
    Uint32,
    Int32,
    Uint16,
    Int16,
    Byte
}

pub struct VmFieldDef {
    name: String,
    field_type: VmType,
    flags: u32
}

impl VmFieldDef {

    fn new(name: &str, ft: VmType) -> VmFieldDef {
        VmFieldDef {
            name: String::from(name),
            field_type: ft,
            flags: 0
        }
    }

}

pub struct VmClass {
    parent: Option<Box<VmClass>>,
    package: String,
    name: String,
    local_fields: Vec<VmFieldDef>
}

impl VmClass {

    pub fn get_ancestor_fields(&self) -> Vec<&VmFieldDef> {

        let mut fields = Vec::new();

        match self.parent {
            Some(ref p) => {
                for f in p.get_ancestor_fields() {
                    fields.push(f)
                }
            }
            None => ()
        }

        for f in self.local_fields.iter() {
            fields.push(f);
        }

        fields

    }

}

pub struct HeapObject {
    obj_type: VmClass
}

pub mod test {

    use obj::*;

    #[test]
    pub fn test() {

        let clazz = VmClass {
            parent: None,
            package: String::from("foo.bar"),
            name: String::from("balls"),
            local_fields: vec![VmFieldDef::new("alice", VmType::Byte)]
        };

        let fields = clazz.get_ancestor_fields();
        println!("clazz {} has {} fields", clazz.name, fields.len());

    }
}
