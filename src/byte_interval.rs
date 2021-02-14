use std::collections::HashMap;

use crate::*;

#[derive(Default, Debug, PartialEq)]
pub(crate) struct ByteInterval {
    pub(crate) parent: Option<Index>,

    uuid: Uuid,
    size: u64,
    address: Option<Addr>,
    bytes: Vec<u8>,
    code_blocks: Vec<Index>,
    data_blocks: Vec<Index>,
    symbolic_expressions: HashMap<u64, SymbolicExpression>,
}

impl ByteInterval {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            ..Default::default()
        }
    }
}

impl Unique for ByteInterval {
    fn uuid(&self) -> Uuid {
        self.uuid
    }

    fn set_uuid(&mut self, uuid: Uuid) {
        self.uuid = uuid;
    }
}

impl Node<ByteInterval> {
    pub fn size(&self) -> u64 {
        return self.borrow().size
    }

    pub fn set_size(&self, n: u64) {
        self.borrow_mut().size = n;
    }

    pub fn address(&self) -> Option<Addr> {
        return self.borrow().address
    }

    pub fn set_address(&self, address: Option<Addr>) {
        self.borrow_mut().address = address;
    }

    pub fn initialized_size(&self) -> u64 {
        self.borrow().bytes.len() as u64
    }

    pub fn set_initialized_size(&self, n: u64) {
        self.borrow_mut().bytes.resize(n as usize, 0);
        if n > self.size() {
            self.set_size(n);
        }
    }

    pub fn bytes(&self) -> Ref<[u8]> {
        Ref::map(self.borrow(), |i| &i.bytes[..] )
    }

    pub fn set_bytes<T: AsRef<[u8]>>(&self, bytes: T) {
        self.borrow_mut().bytes = bytes.as_ref().to_vec();
    }

    pub fn code_blocks(&self) -> NodeIterator<CodeBlock> {
        self.node_iter()
    }

    pub fn add_code_block(&self, code_block: CodeBlock) -> Node<CodeBlock> {
        self.add_node(code_block)
    }

    pub fn remove_code_block(&self, node: Node<CodeBlock>) {
        self.remove_node(node);
    }

    pub fn data_blocks(&self) -> NodeIterator<DataBlock> {
        self.node_iter()
    }

    pub fn add_data_block(&self, data_block: DataBlock) -> Node<DataBlock> {
        self.add_node(data_block)
    }

    pub fn remove_data_block(&self, node: Node<DataBlock>) {
        self.remove_node(node);
    }
}

impl Indexed<ByteInterval> for Node<ByteInterval> {
    fn arena(&self) -> Ref<Arena<ByteInterval>> {
        Ref::map(self.context.borrow(), |ctx| &ctx.byte_interval)
    }

    fn arena_mut(&self) -> RefMut<Arena<ByteInterval>> {
        RefMut::map(self.context.borrow_mut(), |ctx| &mut ctx.byte_interval)
    }
}

impl Child<Section> for Node<ByteInterval> {
    fn parent(&self) -> (Option<Index>, PhantomData<Section>) {
        (self.borrow().parent, PhantomData)
    }

    fn set_parent(&self, (index, _): (Index, PhantomData<Section>)) {
        self.borrow_mut().parent.replace(index);
    }
}

impl Parent<CodeBlock> for Node<ByteInterval> {
    fn nodes(&self) -> Ref<Vec<Index>> {
        Ref::map(self.borrow(), |interval| &interval.code_blocks)
    }

    fn nodes_mut(&self) -> RefMut<Vec<Index>> {
        RefMut::map(self.borrow_mut(), |interval| &mut interval.code_blocks)
    }

    fn node_arena(&self) -> Ref<Arena<CodeBlock>> {
        Ref::map(self.context.borrow(), |ctx| &ctx.code_block)
    }

    fn node_arena_mut(&self) -> RefMut<Arena<CodeBlock>> {
        RefMut::map(self.context.borrow_mut(), |ctx| &mut ctx.code_block)
    }
}

impl Parent<DataBlock> for Node<ByteInterval> {
    fn nodes(&self) -> Ref<Vec<Index>> {
        Ref::map(self.borrow(), |interval| &interval.data_blocks)
    }

    fn nodes_mut(&self) -> RefMut<Vec<Index>> {
        RefMut::map(self.borrow_mut(), |interval| &mut interval.data_blocks)
    }

    fn node_arena(&self) -> Ref<Arena<DataBlock>> {
        Ref::map(self.context.borrow(), |ctx| &ctx.data_block)
    }

    fn node_arena_mut(&self) -> RefMut<Arena<DataBlock>> {
        RefMut::map(self.context.borrow_mut(), |ctx| &mut ctx.data_block)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_set_attributes() {
        let ir = IR::new();
        let module = ir.add_module(Module::new("dummy"));
        let section = module.add_section(Section::new(".dummy"));
        let interval = section.add_byte_interval(ByteInterval::new());
        interval.set_size(0xDEAD);
        interval.set_address(Some(Addr(0xBEEF)));
        assert_eq!(interval.size(), 0xDEAD);
        assert_eq!(interval.address(), Some(Addr(0xBEEF)));
    }
}
