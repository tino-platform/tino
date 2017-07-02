use std::rc::Rc;

#[derive(Clone)]
pub enum VmType {
    Object(Rc<VmClass>), // FIXME This can't be a Box.
    Str,
    Uint64,
    Int64,
    Uint32,
    Int32,
    Uint16,
    Int16,
    Byte
}

#[derive(Clone)]
pub struct VmFieldDef {
    name: String,
    field_type: VmType,
    flags: u32
}

impl VmFieldDef {

    pub fn new(name: &str, ft: VmType) -> VmFieldDef {
        VmFieldDef {
            name: String::from(name),
            field_type: ft,
            flags: 0
        }
    }

}

#[derive(Clone)]
pub struct VmClass {
    parent: Option<Rc<VmClass>>,
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
