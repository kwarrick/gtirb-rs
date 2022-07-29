use std::collections::HashMap;

use crate::*;

#[derive(Debug, Default)]
pub(crate) struct NodeIndex {
    pub(crate) ir: HashMap<Uuid, WNodeBox<IR>>,
    pub(crate) modules: HashMap<Uuid, WNodeBox<Module>>,
    pub(crate) symbols: HashMap<Uuid, WNodeBox<Symbol>>,
    pub(crate) sections: HashMap<Uuid, WNodeBox<Section>>,
    pub(crate) byte_intervals: HashMap<Uuid, WNodeBox<ByteInterval>>,
    pub(crate) code_blocks: HashMap<Uuid, WNodeBox<CodeBlock>>,
    pub(crate) data_blocks: HashMap<Uuid, WNodeBox<DataBlock>>,
    pub(crate) proxy_blocks: HashMap<Uuid, WNodeBox<ProxyBlock>>,
}

#[derive(Clone, Debug)]
pub struct Context {
    pub(crate) index: Rc<RefCell<NodeIndex>>,
}

macro_rules! ctx_find_method {
    ( $find_func: ident, $array: ident, $reftype: ident ) => {
        pub fn $find_func(&self, uuid: &Uuid) -> Option<$reftype> {
            self.index
                .borrow()
                .$array
                .get(uuid)
                .map(|ptr| ptr.upgrade())
                .flatten()
                .map(|ptr| $reftype::new(Node::new(&self, ptr)))
        }
    };
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

    pub(crate) fn add_node<T>(&mut self, node: T) -> Node<T>
    where
        T: Index + Unique,
    {
        let boxed = T::insert(self, node);
        Node::new(&self, boxed)
    }

    ctx_find_method!(find_ir, ir, IRRef);
    ctx_find_method!(find_module, modules, ModuleRef);
    ctx_find_method!(find_symbol, symbols, SymbolRef);
    ctx_find_method!(find_section, sections, SectionRef);
    ctx_find_method!(find_byte_interval, byte_intervals, ByteIntervalRef);
    ctx_find_method!(find_code_block, code_blocks, CodeBlockRef);
    ctx_find_method!(find_data_block, data_blocks, DataBlockRef);
    ctx_find_method!(find_proxy_block, proxy_blocks, ProxyBlockRef);
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn context_persists_with_nodes() {
    //     let ir = {
    //         let mut ctx = Context::new();
    //         IR::new(&mut ctx)
    //     };
    //     ir.node.context.index.borrow();
    // }

    #[test]
    fn can_find_node_by_uuid() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        let module = Module::new(&mut ctx, "foo");
        let uuid = module.uuid();
        ir.add_module(&module);

        let node = ctx.find_module(&uuid);
        assert!(node.is_some());
        assert_eq!(uuid, node.unwrap().uuid());

        let mut node = ctx.find_module(&uuid).unwrap();
        node.set_name("bar");

        let module = ir.modules().last().unwrap();
        assert_eq!(*module.name(), "bar");
    }
}
