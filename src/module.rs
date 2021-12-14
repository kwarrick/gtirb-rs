use anyhow::{anyhow, Result};

use crate::*;

pub use crate::Unique;

#[derive(Debug, PartialEq)]
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
    pub(crate) parent: Option<*const RefCell<IR>>,
}

impl Module {
    pub fn new(context: &mut Context, name: &str) -> Node<Module> {
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
            parent: None,
        };
        context.add_node(module)
    }

    pub fn load_protobuf(
        context: &mut Context,
        message: proto::Module,
    ) -> Result<Node<Module>> {
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
            parent: None,
        };

        let mut module = context.add_node(module);

        // Load Section protobuf messages.
        for m in message.sections.into_iter() {
            let section = Section::load_protobuf(context, m)?;
            module.add_section(section);
        }

        // Load Symbol protobuf messages.
        for m in message.symbols.into_iter() {
            let symbol = Symbol::load_protobuf(context, m)?;
            module.add_symbol(symbol);
        }

        // Load ProxyBlock protobuf messages.
        for m in message.proxies.into_iter() {
            let proxy_block = ProxyBlock::load_protobuf(context, m)?;
            module.add_proxy_block(proxy_block);
        }

        Ok(module)
    }
}

impl Node<Module> {
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

    pub fn symbols(&self) -> Iter<Symbol> {
        Iter {
            inner: Some(Ref::map(self.borrow(), |module| &module.symbols[..])),
            context: &self.context,
        }
    }

    pub fn add_symbol(&mut self, symbol: Node<Symbol>) -> Node<Symbol> {
        let ptr = Weak::into_raw(Rc::downgrade(&Rc::clone(&self.inner)));
        symbol.inner.borrow_mut().parent = Some(ptr);
        self.borrow_mut().symbols.push(Rc::clone(&symbol.inner));
        symbol
    }

    pub fn remove_symbol(&self, uuid: Uuid) -> Option<Node<Symbol>> {
        let mut module = self.inner.borrow_mut();
        if let Some(pos) = module
            .symbols
            .iter()
            .position(|m| m.borrow().uuid() == uuid)
        {
            let ptr = module.symbols.remove(pos);
            ptr.borrow_mut().parent = None;
            Some(Node::new(&self.context, ptr))
        } else {
            None
        }
    }

    pub fn add_section(&mut self, section: Node<Section>) -> Node<Section> {
        let ptr = Weak::into_raw(Rc::downgrade(&Rc::clone(&self.inner)));
        section.inner.borrow_mut().parent = Some(ptr);
        self.borrow_mut().sections.push(Rc::clone(&section.inner));
        section
    }

    pub fn remove_section(&mut self, uuid: Uuid) -> Option<Node<Section>> {
        let mut module = self.inner.borrow_mut();
        if let Some(pos) = module
            .sections
            .iter()
            .position(|m| m.borrow().uuid() == uuid)
        {
            let ptr = module.sections.remove(pos);
            ptr.borrow_mut().parent = None;
            Some(Node::new(&self.context, ptr))
        } else {
            None
        }
    }

    pub fn sections<'a>(&'a self) -> Iter<Section> {
        Iter {
            inner: Some(Ref::map(self.borrow(), |module| &module.sections[..])),
            context: &self.context,
        }
    }

    pub fn add_proxy_block(
        &mut self,
        proxy_block: Node<ProxyBlock>,
    ) -> Node<ProxyBlock> {
        let ptr = Weak::into_raw(Rc::downgrade(&Rc::clone(&self.inner)));
        proxy_block.inner.borrow_mut().parent = Some(ptr);
        self.borrow_mut()
            .proxy_blocks
            .push(Rc::clone(&proxy_block.inner));
        proxy_block
    }

    pub fn remove_proxy_block(
        &mut self,
        uuid: Uuid,
    ) -> Option<Node<ProxyBlock>> {
        let mut module = self.inner.borrow_mut();
        if let Some(pos) = module
            .proxy_blocks
            .iter()
            .position(|m| m.borrow().uuid() == uuid)
        {
            let ptr = module.proxy_blocks.remove(pos);
            ptr.borrow_mut().parent = None;
            Some(Node::new(&self.context, ptr))
        } else {
            None
        }
    }

    pub fn proxy_blocks(&self) -> Iter<ProxyBlock> {
        Iter {
            inner: Some(Ref::map(self.borrow(), |module| {
                &module.proxy_blocks[..]
            })),
            context: &self.context,
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
}

impl Unique for Module {
    fn uuid(&self) -> Uuid {
        self.uuid
    }

    fn set_uuid(&mut self, uuid: Uuid) {
        self.uuid = uuid;
    }
}

impl Node<Module> {
    pub fn ir(&self) -> Option<Node<IR>> {
        self.inner
            .borrow()
            .parent
            .map(|ptr| unsafe { Weak::from_raw(ptr) })
            .map(|weak| weak.upgrade())
            .flatten()
            .map(|strong| Node::new(&self.context, strong))
    }
}

impl Index for Module {
    fn insert(context: &mut Context, node: Self) -> NodeBox<Self> {
        let uuid = node.uuid();
        let boxed = Rc::new(RefCell::new(node));
        context
            .index
            .borrow_mut()
            .modules
            .insert(uuid, Rc::clone(&boxed));
        boxed
    }

    fn remove(context: &mut Context, ptr: NodeBox<Self>) -> NodeBox<Self> {
        let uuid = ptr.borrow().uuid();
        context.index.borrow_mut().modules.remove(&uuid);
        ptr
    }

    fn search(context: &Context, uuid: &Uuid) -> Option<NodeBox<Self>> {
        context
            .index
            .borrow()
            .modules
            .get(uuid)
            .map(|ptr| Rc::clone(&ptr))
    }

    fn rooted(ptr: NodeBox<Self>) -> bool {
        ptr.borrow().parent.is_some()
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
        let _module = ir.add_module(Module::new(&mut ctx, "dummy"));

        // TODO:
        // assert_eq!(module.symbols().count(), 0);
        // assert_eq!(module.sections().count(), 0);
        // assert_eq!(module.proxy_blocks().count(), 0);
    }

    #[test]
    fn can_set_binary_path() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        let path = "/home/gt/irb/foo";
        let mut module = ir.add_module(Module::new(&mut ctx, "dummy"));
        module.set_binary_path(path);
        assert_eq!(*module.binary_path(), path);
    }

    #[test]
    fn can_get_file_format_default() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        let module = ir.add_module(Module::new(&mut ctx, "dummy"));
        assert_eq!(module.file_format(), FileFormat::FormatUndefined);
    }

    #[test]
    fn can_set_file_format() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        let mut module = ir.add_module(Module::new(&mut ctx, "dummy"));
        module.set_file_format(FileFormat::Coff);
        assert_eq!(module.file_format(), FileFormat::Coff);

        module.set_file_format(FileFormat::Macho);
        assert_eq!(module.file_format(), FileFormat::Macho);
    }

    #[test]
    fn can_set_name() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        let mut module = ir.add_module(Module::new(&mut ctx, "dummy"));
        module.set_name("example");
        assert_eq!(*module.name(), "example");
    }

    #[test]
    fn can_relocate_module() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        let mut module = ir.add_module(Module::new(&mut ctx, "dummy"));
        assert!(!module.is_relocated());
        assert_eq!(module.rebase_delta(), 0);

        module.set_rebase_delta(0x1000);
        assert!(module.is_relocated());
        assert_eq!(module.rebase_delta(), 0x1000);
    }

    //     #[test]
    //     fn can_add_new_section() {
    //         let ir = IR::new();
    //         let module = Module::new("dummy");
    //         let module = ir.add_module(module);
    //         assert_eq!(module.ir(), ir);
    //     }

    //     #[test]
    //     fn can_remove_section() {
    //         let ir = IR::new();
    //         let module = ir.add_module(Module::new("foo"));
    //         let section = module.add_section(Section::new("bar"));
    //         module.remove_section(section);
    //         assert_eq!(module.sections().count(), 0);
    //     }

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
