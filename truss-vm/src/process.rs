use std::boxed::Box;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

use obj::VmClass;

pub struct HeapObject {
    obj_type: Box<VmClass>,
    fields: Vec<Cell<HeapValue>>
}

impl HeapObject {

    pub fn new(clazz: Box<VmClass>) -> HeapObject {

        let cnt = clazz.get_ancestor_fields().len();
        HeapObject {
            obj_type: clazz,
            fields: vec![Cell::new(HeapValue::Empty); cnt]
        }

    }

}

#[allow(non_camel_case_types)]
type uindex = u64; // Used for indexing objects in heap vectors.

// TODO All of these are much wider than they actually need to be.
#[derive(Copy, Clone)]
pub enum HeapValue {
    Object(uindex),
    Dynamic(uindex),
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

pub struct Process {
    name: String,
    heap: Vec<Cell<HeapObject>>
}
