use std::cell::Cell;
use std::rc::Rc;

use obj::VmClass;
use vm::func::VmFunction;

#[derive(Clone)]
pub struct HeapObject {
    vtable: VTable, // Should this be a reference?  What if we want to do composition?
    fields: Vec<Cell<HeapValue>>
}

#[derive(Clone)]
struct VTable {
    class: Rc<VmClass>,
    funcs: Vec<Rc<VmFunction>>
}

impl VTable {

    fn from_class(clazz: Rc<VmClass>) -> VTable {

        let mut funcs = vec![];

        // TODO Get the functions from the class.

        VTable { class: clazz, funcs: funcs }

    }

}

impl HeapObject {

    pub fn new(clazz: Rc<VmClass>) -> HeapObject {

        let cnt = clazz.get_ancestor_fields().len();
        HeapObject {
            vtable: VTable::from_class(clazz),
            fields: vec![Cell::new(HeapValue::Empty); cnt]
        }

    }

    pub fn get_class(&self) -> Rc<VmClass> {
        self.vtable.class.clone()
    }

    pub fn get_field(&self, name: String) -> Option<HeapValue> {

        let clazz = self.get_class();
        let fields = clazz.get_ancestor_fields().clone();
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

    pub fn get_function_index(&self, idx: usize) -> Rc<VmFunction> {
        self.vtable.funcs[idx].clone()
    }

}

#[allow(non_camel_case_types)]
pub type HeapIndex = usize; // Used for indexing objects in heap vectors.

// TODO All of these are much wider than they actually need to be.
#[derive(Copy, Clone)]
pub enum HeapValue {
    Object(HeapIndex),
    Dynamic(HeapIndex),
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
