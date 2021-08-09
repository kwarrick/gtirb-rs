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

    // TODO: XXX: Allows multiple mutable references, e.g.:
    //   let mut a = ir.modules_mut().nth(0).unwrap();
    //   let mut b = ctx.find_node_mut::<Module>(&uuid).unwrap();
    pub fn find_node<T>(&mut self, uuid: &Uuid) -> Option<Node<T>>
    where
        T: Allocate + Deallocate + Index<T>,
    {
        T::find(self, uuid).map(|ptr| Node::from_raw(self, ptr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bad_bad_multiple_muts() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        let module = Module::new(&mut ctx, "foo");
        let uuid = module.uuid();
        ir.add_module(module);

        let mut a = ir.modules().nth(0).unwrap();
        let mut a_ = ctx.find_node::<Module>(&uuid).unwrap();

        a.set_name("foo");
        a_.set_name("bar");
    }

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
