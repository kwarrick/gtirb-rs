use std::collections::HashMap;

use crate::*;

#[derive(Debug)]
pub struct Context {
    pub(crate) ir: HashMap<Uuid, *mut IR>,
    pub(crate) modules: HashMap<Uuid, *mut Module>,
}

// TODO: What happens if a Context is dropped before its Nodes? We may have to
// use a closure ...

impl Context {
    pub fn new() -> Box<Context> {
        Box::new(Context {
            ir: HashMap::new(),
            modules: HashMap::new(),
        })
    }

    pub fn find_node<T>(&self, uuid: &Uuid) -> Option<&T>
    where
        T: Index<T>,
    {
        T::find(self, uuid).map(|ptr| unsafe { &*ptr })
    }

    // TODO: Pin? Allows multiple mutable references:
    pub fn find_node_mut<T>(&mut self, uuid: &Uuid) -> Option<&mut T>
    where
        T: Index<T>,
    {
        T::find(self, uuid).map(|ptr| unsafe { &mut *ptr })
    }
}
