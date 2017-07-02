use std::cell::{RefCell};
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::sync::Arc;

use obj::VmClass;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct ClassIdentifier {
    pub name: String,
    pub package: String
}

impl ClassIdentifier {
    pub fn new(name: String, pkg: String) -> ClassIdentifier {
        ClassIdentifier { name: name, package: pkg }
    }
}

pub trait ClassSource {
    fn load_class(&self, id: ClassIdentifier) -> Result<VmClass, ()>;
}

pub struct ClassPool<S: ClassSource> {
    pool: RefCell<HashMap<ClassIdentifier, Weak<VmClass>>>,
    loader: Arc<S>
}

impl<S: ClassSource> ClassPool<S> {

    pub fn new(src: Arc<S>) -> ClassPool<S> {
        ClassPool { pool: RefCell::new(HashMap::new()), loader: src.clone() }
    }

    pub fn find_class(&mut self, id: ClassIdentifier) -> Result<Rc<VmClass>, ()> {

        let mut pool = self.pool.borrow_mut();

        let cached = match pool.get_mut(&id) {
            Some(c) => match c.upgrade() {
                Some(r) => Some(r),
                None => None
            },
            None => None
        };

        match cached {
            Some(c) => Ok(c),
            None => {
                match self.loader.load_class(id.clone()) {
                    Ok(lc) => {
                        let rc = Rc::new(lc);
                        pool.insert(id.clone(), Rc::downgrade(&rc));
                        Ok(rc)
                    },
                    Err(_) => Err(())
                }
            }
        }

    }

}

pub struct DummySource;

impl ClassSource for DummySource {
    fn load_class(&self, id: ClassIdentifier) -> Result<VmClass, ()> {
        Ok(VmClass::new_dummy(id)) // FIXME Make this do something logical.
    }
}
