use crate::*;

#[derive(Default, Debug, PartialEq)]
pub(crate) struct CodeBlock {
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
    fn get_ref(
        &self,
        (index, _): (Index, PhantomData<CodeBlock>),
    ) -> Option<Ref<CodeBlock>> {
        let context = self.context.borrow();
        if context.code_block.contains(index) {
            Some(Ref::map(context, |ctx| &ctx.code_block[index]))
        } else {
            None
        }
    }

    fn get_ref_mut(
        &self,
        (index, _): (Index, PhantomData<CodeBlock>),
    ) -> Option<RefMut<CodeBlock>> {
        let context = self.context.borrow_mut();
        if context.code_block.contains(index) {
            Some(RefMut::map(context, |ctx| &mut ctx.code_block[index]))
        } else {
            None
        }
    }
}
