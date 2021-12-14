use std::collections::HashMap;

use crate::*;

#[derive(Debug, Default)]
pub(crate) struct NodeIndex {
    pub(crate) ir: HashMap<Uuid, NodeBox<IR>>,
    pub(crate) modules: HashMap<Uuid, NodeBox<Module>>,
    pub(crate) symbols: HashMap<Uuid, NodeBox<Symbol>>,
    pub(crate) sections: HashMap<Uuid, NodeBox<Section>>,
    pub(crate) byte_intervals: HashMap<Uuid, NodeBox<ByteInterval>>,
    pub(crate) code_blocks: HashMap<Uuid, NodeBox<CodeBlock>>,
    pub(crate) data_blocks: HashMap<Uuid, NodeBox<DataBlock>>,
    pub(crate) proxy_blocks: HashMap<Uuid, NodeBox<ProxyBlock>>,
}

#[derive(Clone, Debug)]
pub struct Context {
    pub(crate) index: Rc<RefCell<NodeIndex>>,
}

impl Context {
    pub fn new() -> Self {
        let index = NodeIndex {
            ..Default::default()
        };
        Context {
            index: Rc::new(RefCell::new(index)),
        }
    }

    pub fn add_node<T>(&mut self, node: T) -> Node<T>
    where
        T: Index + Unique,
    {
        let boxed = T::insert(self, node);
        Node::new(&self, boxed)
    }

    pub fn find_node<T>(&self, uuid: &Uuid) -> Option<Node<T>>
    where
        T: Index + Unique,
    {
        T::search(self, uuid).map(|ptr| Node::new(&self, ptr))
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
        assert_eq!(*module.name(), "bar");
    }
}
