use crate::*;

#[derive(Debug, Default, PartialEq)]
pub(crate) struct Module {
    pub(crate) parent: Option<Index>,

    uuid: Uuid,
    name: String,
    binary_path: String,
    entry_point: Option<Index>,
    byte_order: ByteOrder,
    isa: ISA,
    rebase_delta: i64,
    preferred_address: Addr,
    file_format: FileFormat,
    sections: Vec<Index>,
    symbols: Vec<Index>,
    proxy_blocks: Vec<Index>,
}

impl Module {
    pub fn new(name: &str) -> Self {
        Module {
            uuid: Uuid::new_v4(),
            name: name.to_owned(),
            ..Default::default()
        }
    }
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
    pub fn ir(&self) -> Node<IR> {
        Node {
            index: self.borrow().parent.expect("parent node"),
            context: self.context.clone(),
            kind: PhantomData,
        }
    }

    pub fn name(&self) -> String {
        self.borrow().name.to_owned()
    }

    pub fn set_name<T: AsRef<str>>(&self, name: T) {
        self.borrow_mut().name = name.as_ref().to_owned();
    }

    pub fn binary_path(&self) -> String {
        self.borrow().binary_path.to_owned()
    }

    pub fn set_binary_path<T: AsRef<str>>(&self, binary_path: T) {
        self.borrow_mut().binary_path = binary_path.as_ref().to_owned();
    }

    pub fn file_format(&self) -> FileFormat {
        self.borrow().file_format
    }

    pub fn set_file_format(&self, file_format: FileFormat) {
        self.borrow_mut().file_format = file_format;
    }

    pub fn isa(&self) -> ISA {
        self.borrow().isa
    }

    pub fn set_isa(&self, isa: ISA) {
        self.borrow_mut().isa = isa;
    }

    pub fn entry_point(&self) -> Option<Node<CodeBlock>> {
        self.borrow().entry_point.map(|index| Node {
            index,
            context: self.context.clone(),
            kind: PhantomData,
        })
    }

    pub fn set_entry_point(&self, block: Node<CodeBlock>) {
        self.borrow_mut().entry_point.replace(block.index);
    }

    pub fn byte_order(&self) -> ByteOrder {
        self.borrow().byte_order
    }

    pub fn set_byte_order(&self, byte_order: ByteOrder) {
        self.borrow_mut().byte_order = byte_order;
    }

    pub fn preferred_address(&self) -> Addr {
        self.borrow().preferred_address
    }

    pub fn set_preferred_address(&self, preferred_address: Addr) {
        self.borrow_mut().preferred_address = preferred_address;
    }

    pub fn rebase_delta(&self) -> i64 {
        self.borrow().rebase_delta
    }

    pub fn set_rebase_delta(&self, rebase_delta: i64) {
        self.borrow_mut().rebase_delta = rebase_delta;
    }

    pub fn is_relocated(&self) -> bool {
        self.borrow().rebase_delta != 0
    }

    pub fn sections(&self) -> NodeIterator<Section> {
        self.node_iter()
    }

    pub fn add_section(&self, section: Section) -> Node<Section> {
        self.add_node(section)
    }

    pub fn remove_section(&self, node: Node<Section>) {
        self.remove_node(node);
    }

    pub fn proxy_blocks(&self) -> NodeIterator<ProxyBlock> {
        self.node_iter()
    }

    pub fn add_proxy_block(&self, proxy_block: ProxyBlock) -> Node<ProxyBlock> {
        self.add_node(proxy_block)
    }

    pub fn remove_proxy_block(&self, node: Node<ProxyBlock>) {
        self.remove_node(node);
    }

    pub fn symbols(&self) -> NodeIterator<Symbol> {
        self.node_iter()
    }

    pub fn add_symbol(&self, symbol: Symbol) -> Node<Symbol> {
        self.add_node(symbol)
    }

    pub fn remove_symbol(&self, node: Node<Symbol>) {
        self.remove_node(node);
    }

    pub fn size(&self) -> Option<u64> {
        let min: Option<Addr> =
            self.sections().map(|i| i.address()).min().flatten();
        let max: Option<Addr> = self
            .sections()
            .map(|i| i.address().zip(i.size()).map(|(addr,size)| addr + size.into()))
            .max()
            .flatten();
        if let (Some(min), Some(max)) = (min, max) {
            Some(u64::from(max - min))
        } else {
            None
        }
    }

    pub fn address(&self) -> Option<Addr> {
        self.sections().map(|s| s.address()).min().flatten()
    }

    pub fn byte_intervals(&self) -> NodeIterator<ByteInterval> {
        let iter = self.sections().flat_map(|interval| {
            <Node<Section> as Parent<ByteInterval>>::nodes(&interval)
                .clone()
                .into_iter()
        });
        NodeIterator {
            iter: Box::new(iter),
            context: self.context.clone(),
            kind: PhantomData,
        }
    }

    pub fn code_blocks(&self) -> NodeIterator<CodeBlock> {
        let iter = self.sections().flat_map(|section| {
            section.byte_intervals().flat_map(|interval| {
                <Node<ByteInterval> as Parent<CodeBlock>>::nodes(&interval)
                    .clone()
                    .into_iter()
            })
        });
        NodeIterator {
            iter: Box::new(iter),
            context: self.context.clone(),
            kind: PhantomData,
        }
    }

    pub fn data_blocks(&self) -> NodeIterator<DataBlock> {
        let iter = self.sections().flat_map(|section| {
            section.byte_intervals().flat_map(|interval| {
                <Node<ByteInterval> as Parent<DataBlock>>::nodes(&interval)
                    .clone()
                    .into_iter()
            })
        });
        NodeIterator {
            iter: Box::new(iter),
            context: self.context.clone(),
            kind: PhantomData,
        }
    }

    // symbolic_expressions()
    // get_symbol_reference<T>(symbol: Symbol) -> Node<T>
}

impl Indexed<Module> for Node<Module> {
    fn arena(&self) -> Ref<Arena<Module>> {
        Ref::map(self.context.borrow(), |ctx| &ctx.module)
    }

    fn arena_mut(&self) -> RefMut<Arena<Module>> {
        RefMut::map(self.context.borrow_mut(), |ctx| &mut ctx.module)
    }
}

impl Child<IR> for Node<Module> {
    fn parent(&self) -> (Option<Index>, PhantomData<IR>) {
        (self.borrow().parent, PhantomData)
    }

    fn set_parent(&self, (index, _): (Index, PhantomData<IR>)) {
        self.borrow_mut().parent.replace(index);
    }
}

impl Parent<Section> for Node<Module> {
    fn nodes(&self) -> Ref<Vec<Index>> {
        Ref::map(self.borrow(), |module| &module.sections)
    }

    fn nodes_mut(&self) -> RefMut<Vec<Index>> {
        RefMut::map(self.borrow_mut(), |module| &mut module.sections)
    }

    fn node_arena(&self) -> Ref<Arena<Section>> {
        Ref::map(self.context.borrow(), |ctx| &ctx.section)
    }

    fn node_arena_mut(&self) -> RefMut<Arena<Section>> {
        RefMut::map(self.context.borrow_mut(), |ctx| &mut ctx.section)
    }
}

impl Parent<ProxyBlock> for Node<Module> {
    fn nodes(&self) -> Ref<Vec<Index>> {
        Ref::map(self.borrow(), |module| &module.proxy_blocks)
    }

    fn nodes_mut(&self) -> RefMut<Vec<Index>> {
        RefMut::map(self.borrow_mut(), |module| &mut module.proxy_blocks)
    }

    fn node_arena(&self) -> Ref<Arena<ProxyBlock>> {
        Ref::map(self.context.borrow(), |ctx| &ctx.proxy_block)
    }

    fn node_arena_mut(&self) -> RefMut<Arena<ProxyBlock>> {
        RefMut::map(self.context.borrow_mut(), |ctx| &mut ctx.proxy_block)
    }
}

impl Parent<Symbol> for Node<Module> {
    fn nodes(&self) -> Ref<Vec<Index>> {
        Ref::map(self.borrow(), |module| &module.symbols)
    }

    fn nodes_mut(&self) -> RefMut<Vec<Index>> {
        RefMut::map(self.borrow_mut(), |module| &mut module.symbols)
    }

    fn node_arena(&self) -> Ref<Arena<Symbol>> {
        Ref::map(self.context.borrow(), |ctx| &ctx.symbol)
    }

    fn node_arena_mut(&self) -> RefMut<Arena<Symbol>> {
        RefMut::map(self.context.borrow_mut(), |ctx| &mut ctx.symbol)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_module_is_unique() {
        assert_ne!(Module::new("a"), Module::new("a"));
    }

    #[test]
    fn new_module_is_empty() {
        let ir = IR::new();
        let module = ir.add_module(Module::new("dummy"));

        assert_eq!(module.symbols().count(), 0);
        assert_eq!(module.sections().count(), 0);
        assert_eq!(module.proxy_blocks().count(), 0);
    }

    #[test]
    fn can_set_binary_path() {
        let ir = IR::new();
        let path = "/home/gt/irb/foo";
        let module = ir.add_module(Module::new("dummy"));
        module.set_binary_path(path);
        assert_eq!(module.binary_path(), path);
    }

    #[test]
    fn can_get_file_format_default() {
        let ir = IR::new();
        let module = ir.add_module(Module::new("dummy"));
        assert_eq!(module.file_format(), FileFormat::FormatUndefined);
    }

    #[test]
    fn can_set_file_format() {
        let ir = IR::new();
        let module = ir.add_module(Module::new("dummy"));
        module.set_file_format(FileFormat::Coff);
        assert_eq!(module.file_format(), FileFormat::Coff);

        module.set_file_format(FileFormat::Macho);
        assert_eq!(module.file_format(), FileFormat::Macho);
    }

    #[test]
    fn can_set_name() {
        let ir = IR::new();
        let module = ir.add_module(Module::new("dummy"));
        module.set_name("example");
        assert_eq!(module.name(), "example");
    }

    #[test]
    fn can_relocate_module() {
        let ir = IR::new();
        let module = ir.add_module(Module::new("dummy"));
        assert!(!module.is_relocated());
        assert_eq!(module.rebase_delta(), 0);

        module.set_rebase_delta(0x1000);
        assert!(module.is_relocated());
        assert_eq!(module.rebase_delta(), 0x1000);
    }

    #[test]
    fn can_add_new_section() {
        let ir = IR::new();
        let module = Module::new("dummy");
        let module = ir.add_module(module);
        assert_eq!(module.ir(), ir);
    }

    #[test]
    fn can_remove_section() {
        let ir = IR::new();
        let module = ir.add_module(Module::new("foo"));
        let section = module.add_section(Section::new("bar"));
        module.remove_section(section);
        assert_eq!(module.sections().count(), 0);
    }

    #[test]
    fn can_iterate_over_code_blocks() {
        let ir = IR::new();
        let module = ir.add_module(Module::new("dummy"));
        let section = module.add_section(Section::new(".dummy"));
        let b1 = section.add_byte_interval(ByteInterval::new());
        let b2 = section.add_byte_interval(ByteInterval::new());
        let cb1 = b1.add_code_block(CodeBlock::new());
        let cb2 = b2.add_code_block(CodeBlock::new());
        assert_eq!(
            module
                .code_blocks()
                .map(|cb| cb.uuid())
                .collect::<Vec<Uuid>>(),
            vec![cb1.uuid(), cb2.uuid()]
        );
        assert_eq!(
            section
                .code_blocks()
                .map(|cb| cb.uuid())
                .collect::<Vec<Uuid>>(),
            vec![cb1.uuid(), cb2.uuid()]
        );
    }

    #[test]
    fn can_calculate_size() {
        let ir = IR::new();
        let module = ir.add_module(Module::new("dummy"));
        assert_eq!(module.size(), None);
        assert_eq!(module.address(), None);

        let text = module.add_section(Section::new(".text"));
        let bytes = text.add_byte_interval(ByteInterval::new());
        bytes.set_address(Some(Addr(200)));
        bytes.set_size(100);

        assert!(module.address().is_some());
        assert_eq!(module.size(), Some(100));
        assert_eq!(module.address(), Some(Addr(200)));

        bytes.set_address(Some(Addr(0)));
        assert_eq!(module.address(), Some(Addr(0)));

        let data = module.add_section(Section::new(".data"));
        let bytes = data.add_byte_interval(ByteInterval::new());
        bytes.set_address(Some(Addr(300)));
        bytes.set_size(100);
        assert_eq!(module.size(), Some(400));
        assert_eq!(module.address(), Some(Addr(0)));

        assert_eq!(module.byte_intervals().count(), 2);
    }
}
