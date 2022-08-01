use anyhow::{anyhow, Result};

use crate::*;

pub use crate::Unique;

#[derive(Debug, gtirb_derive::Node)]
pub struct Module {
    uuid: Uuid,
    name: String,
    binary_path: String,
    entry_point: Option<Uuid>,
    byte_order: ByteOrder,
    isa: ISA,
    rebase_delta: i64,
    preferred_address: Addr,
    file_format: FileFormat,
    sections: Vec<NodeBox<Section>>,
    symbols: Vec<NodeBox<Symbol>>,
    proxy_blocks: Vec<NodeBox<ProxyBlock>>,
    parent: WNodeBox<IR>,
}

impl Module {
    pub fn new(context: &mut Context, name: &str) -> ModuleRef {
        let module = Module {
            name: name.to_owned(),
            uuid: Uuid::new_v4(),
            binary_path: String::new(),
            entry_point: None,
            byte_order: ByteOrder::Undefined,
            isa: ISA::Undefined,
            rebase_delta: 0,
            preferred_address: Addr(0),
            file_format: FileFormat::FormatUndefined,
            sections: Vec::new(),
            symbols: Vec::new(),
            proxy_blocks: Vec::new(),
            parent: WNodeBox::<IR>::new(),
        };
        context.add_module(module)
    }

    pub fn load_protobuf(
        context: &mut Context,
        message: proto::Module,
    ) -> Result<ModuleRef> {
        let format = FileFormat::from_i32(message.file_format)
            .ok_or(anyhow!("Invalid FileFormat"))?;

        let isa = ISA::from_i32(message.isa).ok_or(anyhow!("Invalid ISA"))?;

        let byte_order = ByteOrder::from_i32(message.byte_order)
            .ok_or(anyhow!("Invalid ByteOrder"))?;

        let module = Module {
            name: message.name,
            uuid: crate::util::parse_uuid(&message.uuid)?,
            binary_path: message.binary_path,
            entry_point: Some(crate::util::parse_uuid(&message.entry_point)?),
            byte_order: byte_order,
            isa: isa,
            rebase_delta: message.rebase_delta,
            preferred_address: Addr(message.preferred_addr),
            file_format: format,
            sections: Vec::new(),
            symbols: Vec::new(),
            proxy_blocks: Vec::new(),
            parent: WNodeBox::<IR>::new(),
        };

        let mut module = context.add_module(module);

        // Load Section protobuf messages.
        for m in message.sections.into_iter() {
            let mut section = Section::load_protobuf(context, m)?;
            module.add_section(&mut section);
        }

        // Load Symbol protobuf messages.
        for m in message.symbols.into_iter() {
            let mut symbol = Symbol::load_protobuf(context, m)?;
            module.add_symbol(&mut symbol);
        }

        // Load ProxyBlock protobuf messages.
        for m in message.proxies.into_iter() {
            let mut proxy_block = ProxyBlock::load_protobuf(context, m)?;
            module.add_proxy_block(&mut proxy_block);
        }

        Ok(module)
    }
}

impl ModuleRef {
    pub fn name(&self) -> Ref<String> {
        Ref::map(self.borrow(), |module| &module.name)
    }

    pub fn set_name<T: AsRef<str>>(&mut self, name: T) {
        self.borrow_mut().name = name.as_ref().to_owned();
    }

    pub fn binary_path(&self) -> Ref<String> {
        Ref::map(self.borrow(), |module| &module.binary_path)
    }

    pub fn set_binary_path<T: AsRef<str>>(&mut self, path: T) {
        self.borrow_mut().binary_path = path.as_ref().to_owned();
    }

    pub fn file_format(&self) -> FileFormat {
        self.borrow().file_format
    }

    pub fn set_file_format(&mut self, file_format: FileFormat) {
        self.borrow_mut().file_format = file_format;
    }

    pub fn isa(&self) -> ISA {
        self.borrow().isa
    }

    pub fn set_isa(&mut self, isa: ISA) {
        self.borrow_mut().isa = isa;
    }

    // pub fn entry_point(&self) -> Option<Node<CodeBlock>> {
    //     self.borrow()
    //         .entry_point
    //         .and_then(|uuid| self.ir().find_node(uuid))
    // }

    // pub fn set_entry_point(&self, block: Node<CodeBlock>) {
    //     self.borrow_mut().entry_point.replace(block.uuid());
    // }

    pub fn byte_order(&self) -> ByteOrder {
        self.borrow().byte_order
    }

    pub fn set_byte_order(&mut self, byte_order: ByteOrder) {
        self.borrow_mut().byte_order = byte_order;
    }

    pub fn preferred_address(&self) -> Addr {
        self.borrow().preferred_address
    }

    pub fn set_preferred_address(&mut self, address: Addr) {
        self.borrow_mut().preferred_address = address;
    }

    pub fn rebase_delta(&self) -> i64 {
        self.borrow().rebase_delta
    }

    pub fn set_rebase_delta(&mut self, rebase_delta: i64) {
        self.borrow_mut().rebase_delta = rebase_delta;
    }

    pub fn is_relocated(&self) -> bool {
        self.borrow().rebase_delta != 0
    }

    pub fn symbols(&self) -> Iter<Symbol, SymbolRef> {
        Iter {
            inner: Some(Ref::map(self.borrow(), |module| &module.symbols[..])),
            context: &self.context,
            phantom: PhantomData,
        }
    }

    pub fn add_symbol(&mut self, symbol: &mut SymbolRef) {
        symbol.borrow_mut().set_parent(Some(&self.inner));
        self.borrow_mut()
            .symbols
            .push(Rc::clone(&symbol.get_inner()));
    }

    pub fn remove_symbol(&mut self, uuid: Uuid) -> Option<SymbolRef> {
        let mut module = self.inner.borrow_mut();
        if let Some(pos) = module
            .symbols
            .iter()
            .position(|m| m.borrow().uuid() == uuid)
        {
            let ptr = module.symbols.remove(pos);
            ptr.borrow_mut().set_parent(None);
            Some(SymbolRef::new(&self.context, ptr))
        } else {
            None
        }
    }

    pub fn add_section(&mut self, section: &mut SectionRef) {
        section.borrow_mut().set_parent(Some(&self.inner));
        self.borrow_mut()
            .sections
            .push(Rc::clone(&section.get_inner()));
    }

    pub fn remove_section(&mut self, uuid: Uuid) -> Option<SectionRef> {
        let mut module = self.inner.borrow_mut();
        if let Some(pos) = module
            .sections
            .iter()
            .position(|m| m.borrow().uuid() == uuid)
        {
            let ptr = module.sections.remove(pos);
            ptr.borrow_mut().set_parent(None);
            Some(SectionRef::new(&self.context, ptr))
        } else {
            None
        }
    }

    pub fn sections<'a>(&'a self) -> Iter<Section, SectionRef> {
        Iter {
            inner: Some(Ref::map(self.borrow(), |module| &module.sections[..])),
            context: &self.context,
            phantom: PhantomData,
        }
    }

    pub fn add_proxy_block(&mut self, proxy_block: &mut ProxyBlockRef) {
        proxy_block.borrow_mut().set_parent(Some(&self.inner));
        self.borrow_mut()
            .proxy_blocks
            .push(Rc::clone(&proxy_block.get_inner()));
    }

    pub fn remove_proxy_block(&mut self, uuid: Uuid) -> Option<ProxyBlockRef> {
        let mut module = self.inner.borrow_mut();
        if let Some(pos) = module
            .proxy_blocks
            .iter()
            .position(|m| m.borrow().uuid() == uuid)
        {
            let ptr = module.proxy_blocks.remove(pos);
            ptr.borrow_mut().set_parent(None);
            Some(ProxyBlockRef::new(&self.context, ptr))
        } else {
            None
        }
    }

    pub fn proxy_blocks(&self) -> Iter<ProxyBlock, ProxyBlockRef> {
        Iter {
            inner: Some(Ref::map(self.borrow(), |module| {
                &module.proxy_blocks[..]
            })),
            context: &self.context,
            phantom: PhantomData,
        }
    }

    // pub fn size(&self) -> Option<u64> {
    //     let min: Option<Addr> =
    //         self.sections().map(|i| i.address()).min().flatten();
    //     let max: Option<Addr> = self
    //         .sections()
    //         .map(|i| {
    //             i.address()
    //                 .zip(i.size())
    //                 .map(|(addr, size)| addr + size.into())
    //         })
    //         .max()
    //         .flatten();
    //     if let (Some(min), Some(max)) = (min, max) {
    //         Some(u64::from(max - min))
    //     } else {
    //         None
    //     }
    // }

    // pub fn address(&self) -> Option<Addr> {
    //     self.sections().map(|s| s.address()).min().flatten()
    // }

    // pub fn byte_intervals(&self) -> NodeIterator<ByteInterval> {
    //     let iter = self.sections().flat_map(|interval| {
    //         <Node<Section> as Parent<ByteInterval>>::nodes(&interval)
    //             .clone()
    //             .into_iter()
    //     });
    //     NodeIterator {
    //         iter: Box::new(iter),
    //         context: self.context.clone(),
    //         kind: PhantomData,
    //     }
    // }

    // pub fn code_blocks(&self) -> NodeIterator<CodeBlock> {
    //     let iter = self.sections().flat_map(|section| {
    //         section.byte_intervals().flat_map(|interval| {
    //             <Node<ByteInterval> as Parent<CodeBlock>>::nodes(&interval)
    //                 .clone()
    //                 .into_iter()
    //         })
    //     });
    //     NodeIterator {
    //         iter: Box::new(iter),
    //         context: self.context.clone(),
    //         kind: PhantomData,
    //     }
    // }

    // pub fn data_blocks(&self) -> NodeIterator<DataBlock> {
    //     let iter = self.sections().flat_map(|section| {
    //         section.byte_intervals().flat_map(|interval| {
    //             <Node<ByteInterval> as Parent<DataBlock>>::nodes(&interval)
    //                 .clone()
    //                 .into_iter()
    //         })
    //     });
    //     NodeIterator {
    //         iter: Box::new(iter),
    //         context: self.context.clone(),
    //         kind: PhantomData,
    //     }
    // }

    // symbolic_expressions()
    // get_symbol_reference<T>(symbol: Symbol) -> Node<T>

    pub fn ir(&self) -> Option<IRRef> {
        self.borrow()
            .get_parent()
            .map(|ptr| IRRef::new(&self.context, ptr))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_module_is_unique() {
        let mut ctx = Context::new();
        assert_ne!(Module::new(&mut ctx, "a"), Module::new(&mut ctx, "a"));
    }

    #[test]
    fn new_module_is_empty() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        let mut module = Module::new(&mut ctx, "dummy");
        ir.add_module(&mut module);
        assert_eq!(module.symbols().count(), 0);
        assert_eq!(module.sections().count(), 0);
        assert_eq!(module.proxy_blocks().count(), 0);
    }

    #[test]
    fn can_set_binary_path() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        let path = "/home/gt/irb/foo";
        let mut module = Module::new(&mut ctx, "dummy");
        ir.add_module(&mut module);
        module.set_binary_path(path);
        assert_eq!(*module.binary_path(), path);
    }

    #[test]
    fn can_get_file_format_default() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        let mut module = Module::new(&mut ctx, "dummy");
        ir.add_module(&mut module);
        assert_eq!(module.file_format(), FileFormat::FormatUndefined);
    }

    #[test]
    fn can_set_file_format() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        let mut module = Module::new(&mut ctx, "dummy");
        ir.add_module(&mut module);
        module.set_file_format(FileFormat::Coff);
        assert_eq!(module.file_format(), FileFormat::Coff);

        module.set_file_format(FileFormat::Macho);
        assert_eq!(module.file_format(), FileFormat::Macho);
    }

    #[test]
    fn can_set_name() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        let mut module = Module::new(&mut ctx, "dummy");
        ir.add_module(&mut module);
        module.set_name("example");
        assert_eq!(*module.name(), "example");
    }

    #[test]
    fn can_relocate_module() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        let mut module = Module::new(&mut ctx, "dummy");
        ir.add_module(&mut module);
        assert!(!module.is_relocated());
        assert_eq!(module.rebase_delta(), 0);

        module.set_rebase_delta(0x1000);
        assert!(module.is_relocated());
        assert_eq!(module.rebase_delta(), 0x1000);
    }

    #[test]
    fn can_add_new_section() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        let mut module = Module::new(&mut ctx, "dummy");
        ir.add_module(&mut module);
        module.add_section(&mut Section::new(&mut ctx, "foo"));
        assert_eq!(module.sections().count(), 1);
    }

    #[test]
    fn can_remove_section() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        let mut module1 = Module::new(&mut ctx, "mod1");
        ir.add_module(&mut module1);
        let mut module2 = Module::new(&mut ctx, "mod2");
        ir.add_module(&mut module2);
        let mut section = Section::new(&mut ctx, "foo");
        module1.add_section(&mut section);
        let uuid = section.uuid();
        assert_eq!(module1.sections().count(), 1);
        {
            let section = module1.remove_section(uuid);
            assert_eq!(module1.sections().count(), 0);

            let mut section = section.unwrap();
            module2.add_section(&mut section);
            assert_eq!(module2.sections().count(), 1);
            module2.remove_section(section.uuid());
        }
        // Section should be dropped after preceding scope.
        let node = ctx.find_module(&uuid);
        assert!(node.is_none());
    }

    //     #[test]
    //     fn can_iterate_over_code_blocks() {
    //         let ir = IR::new();
    //         let module = ir.add_module(Module::new("dummy"));
    //         let section = module.add_section(Section::new(".dummy"));
    //         let b1 = section.add_byte_interval(ByteInterval::new());
    //         let b2 = section.add_byte_interval(ByteInterval::new());
    //         let cb1 = b1.add_code_block(CodeBlock::new());
    //         let cb2 = b2.add_code_block(CodeBlock::new());
    //         assert_eq!(
    //             module
    //                 .code_blocks()
    //                 .map(|cb| cb.uuid())
    //                 .collect::<Vec<Uuid>>(),
    //             vec![cb1.uuid(), cb2.uuid()]
    //         );
    //         assert_eq!(
    //             section
    //                 .code_blocks()
    //                 .map(|cb| cb.uuid())
    //                 .collect::<Vec<Uuid>>(),
    //             vec![cb1.uuid(), cb2.uuid()]
    //         );
    //     }

    //     #[test]
    //     fn can_calculate_size() {
    //         let ir = IR::new();
    //         let module = ir.add_module(Module::new("dummy"));
    //         assert_eq!(module.size(), None);
    //         assert_eq!(module.address(), None);

    //         let text = module.add_section(Section::new(".text"));
    //         let bytes = text.add_byte_interval(ByteInterval::new());
    //         bytes.set_address(Some(Addr(200)));
    //         bytes.set_size(100);

    //         assert!(module.address().is_some());
    //         assert_eq!(module.size(), Some(100));
    //         assert_eq!(module.address(), Some(Addr(200)));

    //         bytes.set_address(Some(Addr(0)));
    //         assert_eq!(module.address(), Some(Addr(0)));

    //         let data = module.add_section(Section::new(".data"));
    //         let bytes = data.add_byte_interval(ByteInterval::new());
    //         bytes.set_address(Some(Addr(300)));
    //         bytes.set_size(100);
    //         assert_eq!(module.size(), Some(400));
    //         assert_eq!(module.address(), Some(Addr(0)));

    //         assert_eq!(module.byte_intervals().count(), 2);
    //     }
}
