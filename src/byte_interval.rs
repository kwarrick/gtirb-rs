use crate::*;

#[derive(Default, Debug, PartialEq)]
pub(crate) struct ByteInterval {
    pub(crate) parent: Option<Index>,

    uuid: Uuid,
    code_blocks: Vec<Index>,
    data_blocks: Vec<Index>,
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
    // code_blocks()
    // add_code_block
    // remove_code_block

    // data_blocks()
    // add_data_block
    // remove_data_block
}

impl Indexed<ByteInterval> for Node<ByteInterval> {
    fn get_ref(
        &self,
        (index, _): (Index, PhantomData<ByteInterval>),
    ) -> Option<Ref<ByteInterval>> {
        let context = self.context.borrow();
        if context.byte_interval.contains(index) {
            Some(Ref::map(context, |ctx| &ctx.byte_interval[index]))
        } else {
            None
        }
    }

    fn get_ref_mut(
        &self,
        (index, _): (Index, PhantomData<ByteInterval>),
    ) -> Option<RefMut<ByteInterval>> {
        let context = self.context.borrow_mut();
        if context.byte_interval.contains(index) {
            Some(RefMut::map(context, |ctx| &mut ctx.byte_interval[index]))
        } else {
            None
        }
    }
}
