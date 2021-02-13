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

    pub fn set_ir(&self, ir: Node<IR>) {
        self.borrow_mut().parent.replace(ir.index);
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

    // TODO:
    // size
    // address

    // code_blocks()
    // data_blocks()
    // byte_intervals()
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
}
