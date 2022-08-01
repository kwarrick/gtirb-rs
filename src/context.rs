use std::collections::HashMap;

use crate::*;

#[derive(Debug, Default)]
pub(crate) struct NodeIndex {
    pub(crate) irs: HashMap<Uuid, WNodeBox<IR>>,
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

macro_rules! ctx_node_methods {
    ( $add_func: ident, $find_func: ident, $remove_func: ident, $array: ident, $nodetype: ident, $reftype: ident ) => {
        pub(crate) fn $add_func(&mut self, node: $nodetype) -> $reftype {
            let uuid = node.uuid();
            let boxed = Rc::new(RefCell::new(node));
            self.index
                .borrow_mut()
                .$array
                .insert(uuid, Rc::downgrade(&boxed));
            $reftype::new(&self, boxed)
        }

        pub fn $find_func(&self, uuid: &Uuid) -> Option<$reftype> {
            self.index
                .borrow()
                .$array
                .get(uuid)
                .map(|ptr| ptr.upgrade())
                .flatten()
                .map(|ptr| $reftype::new(&self, ptr))
        }

        pub(crate) fn $remove_func(&mut self, uuid: &Uuid) {
            self.index.borrow_mut().$array.remove(&uuid);
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

    ctx_node_methods!(add_ir, find_ir, remove_ir, irs, IR, IRRef);
    ctx_node_methods!(
        add_module,
        find_module,
        remove_module,
        modules,
        Module,
        ModuleRef
    );
    ctx_node_methods!(
        add_symbol,
        find_symbol,
        remove_symbol,
        symbols,
        Symbol,
        SymbolRef
    );
    ctx_node_methods!(
        add_section,
        find_section,
        remove_section,
        sections,
        Section,
        SectionRef
    );
    ctx_node_methods!(
        add_byte_interval,
        find_byte_interval,
        remove_byte_interval,
        byte_intervals,
        ByteInterval,
        ByteIntervalRef
    );
    ctx_node_methods!(
        add_code_block,
        find_code_block,
        remove_code_block,
        code_blocks,
        CodeBlock,
        CodeBlockRef
    );
    ctx_node_methods!(
        add_data_block,
        find_data_block,
        remove_data_block,
        data_blocks,
        DataBlock,
        DataBlockRef
    );
    ctx_node_methods!(
        add_proxy_block,
        find_proxy_block,
        remove_proxy_block,
        proxy_blocks,
        ProxyBlock,
        ProxyBlockRef
    );
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
        let mut module = Module::new(&mut ctx, "foo");
        let uuid = module.uuid();
        ir.add_module(&mut module);

        let node = ctx.find_module(&uuid);
        assert!(node.is_some());
        assert_eq!(uuid, node.unwrap().uuid());

        let mut node = ctx.find_module(&uuid).unwrap();
        node.set_name("bar");

        let module = ir.modules().last().unwrap();
        assert_eq!(*module.name(), "bar");
    }
}
