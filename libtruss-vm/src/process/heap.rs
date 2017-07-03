use std::cell::Cell;
use std::rc::Rc;

use obj::VmClass;

pub struct HeapObject {
    obj_type: Rc<VmClass>,
    fields: Vec<Cell<HeapValue>>
}

impl HeapObject {

    pub fn new(clazz: Rc<VmClass>) -> HeapObject {

        let cnt = clazz.get_ancestor_fields().len();
        HeapObject {
            obj_type: clazz,
            fields: vec![Cell::new(HeapValue::Empty); cnt]
        }

    }

    pub fn get_field(&self, name: String) -> Option<HeapValue> {

        let fields = self.obj_type.get_ancestor_fields();
        for i in 0..fields.len() {
            if fields[i].name == name {
                return self.get_field_index(i)
            }
        }

        return None

    }

    pub fn get_field_index(&self, idx: usize) -> Option<HeapValue> {
        if idx < self.fields.len() {
            Some(self.fields[idx].get())
        } else {
            None
        }
    }

}

#[allow(non_camel_case_types)]
type hindex = u64; // Used for indexing objects in heap vectors.

// TODO All of these are much wider than they actually need to be.
#[derive(Copy, Clone)]
pub enum HeapValue {
    Object(hindex),
    Dynamic(hindex),
    Integer(i64),
    Byte(u8),
    Boolean(bool),
    Empty
}

pub enum HeapEntry {
    Object(Box<HeapObject>),
    Dynamic(Vec<u8>),
    Empty
}

pub type StackValue = HeapValue; // TODO Figure this out.
