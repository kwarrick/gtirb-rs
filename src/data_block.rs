use crate::*;

#[derive(Default, Debug, PartialEq)]
pub(crate) struct DataBlock {
    pub(crate) parent: Option<Index>,

    uuid: Uuid,
}

impl DataBlock {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            ..Default::default()
        }
    }
}

impl Unique for DataBlock {
    fn uuid(&self) -> Uuid {
        self.uuid
    }

    fn set_uuid(&mut self, uuid: Uuid) {
        self.uuid = uuid;
    }
}

impl Node<DataBlock> {}

impl Indexed<DataBlock> for Node<DataBlock> {
    fn get_ref(
        &self,
        (index, _): (Index, PhantomData<DataBlock>),
    ) -> Option<Ref<DataBlock>> {
        let context = self.context.borrow();
        if context.data_block.contains(index) {
            Some(Ref::map(context, |ctx| &ctx.data_block[index]))
        } else {
            None
        }
    }

    fn get_ref_mut(
        &self,
        (index, _): (Index, PhantomData<DataBlock>),
    ) -> Option<RefMut<DataBlock>> {
        let context = self.context.borrow_mut();
        if context.data_block.contains(index) {
            Some(RefMut::map(context, |ctx| &mut ctx.data_block[index]))
        } else {
            None
        }
    }
}
