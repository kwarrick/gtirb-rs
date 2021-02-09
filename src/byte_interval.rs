use crate::*;

#[derive(Default, Debug, PartialEq)]
pub(crate) struct ByteInterval {
    pub(crate) parent: Option<Index>,

    uuid: Uuid,
    code_blocks: Vec<Index>,
    data_blocks: Vec<Index>,
    symbolic_expressions: HashMap<u64, Index>,
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
    pub fn code_blocks(&self) -> NodeIterator<ByteInterval, CodeBlock> {
        self.node_iter()
    }

    pub fn add_code_block(&self, code_block: CodeBlock) -> Node<CodeBlock> {
        self.add_node(code_block)
    }

    pub fn remove_code_block(&self, node: Node<CodeBlock>) {
        self.remove_node(node);
    }

    pub fn data_blocks(&self) -> NodeIterator<ByteInterval, DataBlock> {
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
