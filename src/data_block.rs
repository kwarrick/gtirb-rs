use crate::*;

#[derive(Default, Debug, PartialEq)]
pub struct DataBlock {
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
    fn arena(&self) -> Ref<Arena<DataBlock>> {
        Ref::map(self.context.borrow(), |ctx| &ctx.data_block)
    }

    fn arena_mut(&self) -> RefMut<Arena<DataBlock>> {
        RefMut::map(self.context.borrow_mut(), |ctx| &mut ctx.data_block)
    }
}

impl Child<ByteInterval> for Node<DataBlock> {
    fn parent(&self) -> (Option<Index>, PhantomData<ByteInterval>) {
        (self.borrow().parent, PhantomData)
    }

    fn set_parent(&self, (index, _): (Index, PhantomData<ByteInterval>)) {
        self.borrow_mut().parent.replace(index);
    }
}
