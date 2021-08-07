use std::collections::HashMap;

use crate::*;

#[derive(Debug)]
pub struct Context {
    pub(crate) ir: HashMap<Uuid, *mut IR>,
    pub(crate) modules: HashMap<Uuid, *mut Module>,
}

impl Context {
    pub fn new() -> Box<Context> {
        Box::new(Context {
            ir: HashMap::new(),
            modules: HashMap::new(),
        })
    }

    // TODO:
    // fn add_node<T>(node: Node<T>) {}

    pub(crate) fn add_ir(&mut self, uuid: Uuid, node: *mut IR) -> *mut IR {
        self.ir.insert(uuid, node);
        node
    }

    pub(crate) fn add_module(
        &mut self,
        uuid: Uuid,
        node: *mut Module,
    ) -> *mut Module {
        self.modules.insert(uuid, node);
        node
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
