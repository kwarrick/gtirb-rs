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

    // TODO: Pin? Allows multiple mutable references, e.g.:
    //   let mut a = ir.modules_mut().nth(0).unwrap();
    //   let mut b = ctx.find_node_mut::<Module>(&uuid).unwrap();
    pub fn find_node_mut<T>(&mut self, uuid: &Uuid) -> Option<&mut T>
    where
        T: Index<T>,
    {
        T::find(self, uuid).map(|ptr| unsafe { &mut *ptr })
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

        let a = ir.modules_mut().nth(0).unwrap();
        let a_ = ctx.find_node_mut::<Module>(&uuid).unwrap();

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

        let node: Option<&Module> = ctx.find_node(&uuid);
        assert!(node.is_some());
        assert_eq!(uuid, node.unwrap().uuid());

        let node: &mut Module = ctx.find_node_mut(&uuid).unwrap();
        node.set_name("bar");

        let module = ir.modules().last().unwrap();
        assert_eq!(module.name(), "bar");
    }
}
