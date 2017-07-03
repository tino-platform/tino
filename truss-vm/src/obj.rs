use std::hash::{Hash, Hasher};
use std::rc::Rc;

use classload::ClassIdentifier;

#[derive(Clone)]
pub enum VmType {
    Object(Rc<VmClass>),
    Dynamic,
    Integer,
    Byte,
    Boolean
}

#[derive(Clone)]
pub struct VmFieldDef { // Is it bad to have all these fields pub?
    pub name: String,
    pub field_type: VmType,
    pub flags: u32
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
    name: String,
    package: String,
    parent: Option<Rc<VmClass>>,
    local_fields: Vec<VmFieldDef>
}

impl VmClass {

    pub fn new_dummy(id: ClassIdentifier) -> VmClass {
        VmClass {
            name: id.name.clone(),
            package: id.package.clone(),
            parent: None,
            local_fields: vec![]
        }
    }

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

    pub fn to_identifier(&self) -> ClassIdentifier {
        ClassIdentifier::new(self.name.clone(), self.package.clone())
    }

}

impl Hash for VmClass {

    fn hash<H: Hasher>(&self, state: &mut H) {
        self.to_identifier().hash(state);
    }

}
