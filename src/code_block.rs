use crate::*;

#[derive(Default, Debug, PartialEq)]
pub struct CodeBlock {
    pub(crate) parent: Option<Index>,

    uuid: Uuid,
}

impl CodeBlock {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            ..Default::default()
        }
    }
}

impl Unique for CodeBlock {
    fn uuid(&self) -> Uuid {
        self.uuid
    }

    fn set_uuid(&mut self, uuid: Uuid) {
        self.uuid = uuid;
    }
}

impl Node<CodeBlock> {}

impl Indexed<CodeBlock> for Node<CodeBlock> {
    fn arena(&self) -> Ref<Arena<CodeBlock>> {
        Ref::map(self.context.borrow(), |ctx| &ctx.code_block)
    }

    fn arena_mut(&self) -> RefMut<Arena<CodeBlock>> {
        RefMut::map(self.context.borrow_mut(), |ctx| &mut ctx.code_block)
    }
}

impl Child<ByteInterval> for Node<CodeBlock> {
    fn parent(&self) -> (Option<Index>, PhantomData<ByteInterval>) {
        (self.borrow().parent, PhantomData)
    }

    fn set_parent(&self, (index, _): (Index, PhantomData<ByteInterval>)) {
        self.borrow_mut().parent.replace(index);
    }
}
