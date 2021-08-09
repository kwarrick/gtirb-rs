use std::collections::HashMap;

use crate::*;

#[derive(Debug)]
pub(crate) struct NodeIndex {
    pub(crate) ir: HashMap<Uuid, *mut IR>,
    pub(crate) modules: HashMap<Uuid, *mut Module>,
}

#[derive(Clone, Debug)]
pub struct Context {
    pub(crate) index: Rc<RefCell<NodeIndex>>,
}

impl Context {
    pub fn new() -> Self {
        let index = NodeIndex {
            ir: HashMap::new(),
            modules: HashMap::new(),
        };
        Context {
            index: Rc::new(RefCell::new(index)),
        }
    }

    // TODO: XXX: Allows multiple mutable references, e.g.:
    //   let mut a = ir.modules_mut().nth(0).unwrap();
    //   let mut b = ctx.find_node_mut::<Module>(&uuid).unwrap();
    pub fn find_node<T>(&self, uuid: &Uuid) -> Option<Node<T>>
    where
        T: Allocate + Deallocate + Index + Unique,
    {
        T::search(self, uuid).map(|ptr| Node::new(&self, ptr))
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        if Rc::strong_count(&self.index) == 1 {
            let ptrs: Vec<*mut IR> =
                self.index.borrow().ir.values().map(|ptr| *ptr).collect();
            for ptr in ptrs.into_iter() {
                IR::remove(self, ptr);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn context_persists_with_nodes() {
        let ir = {
            let mut ctx = Context::new();
            IR::new(&mut ctx)
        };
        ir.context.index.borrow();
    }

    // TODO: XXX: BAD! BAD!
    // #[test]
    // fn bad_bad_multiple_muts() {
    //     let mut ctx = Context::new();
    //     let mut ir = IR::new(&mut ctx);
    //     let module = Module::new(&mut ctx, "foo");
    //     let uuid = module.uuid();
    //     ir.add_module(module);

    //     let mut a = ir.modules().nth(0).unwrap();
    //     let mut a_ = ctx.find_node::<Module>(&uuid).unwrap();

    //     a.set_name("foo");
    //     a_.set_name("bar");
    // }

    #[test]
    fn can_find_node_by_uuid() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        let module = Module::new(&mut ctx, "foo");
        let uuid = module.uuid();
        ir.add_module(module);

        let node = ctx.find_node::<Module>(&uuid);
        assert!(node.is_some());
        assert_eq!(uuid, node.unwrap().uuid());

        let mut node = ctx.find_node::<Module>(&uuid).unwrap();
        node.set_name("bar");

        let module = ir.modules().last().unwrap();
        assert_eq!(module.name(), "bar");
    }
}
