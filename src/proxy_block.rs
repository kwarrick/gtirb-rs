use crate::*;

#[derive(Default, Debug, PartialEq)]
pub(crate) struct ProxyBlock {
    pub(crate) parent: Option<Index>,

    uuid: Uuid,
}

impl ProxyBlock {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            ..Default::default()
        }
    }
}

impl Unique for ProxyBlock {
    fn uuid(&self) -> Uuid {
        self.uuid
    }

    fn set_uuid(&mut self, uuid: Uuid) {
        self.uuid = uuid;
    }
}

impl Node<ProxyBlock> {}

impl Indexed<ProxyBlock> for Node<ProxyBlock> {
    fn get_ref(
        &self,
        (index, _): (Index, PhantomData<ProxyBlock>),
    ) -> Option<Ref<ProxyBlock>> {
        let context = self.context.borrow();
        if context.proxy_block.contains(index) {
            Some(Ref::map(context, |ctx| &ctx.proxy_block[index]))
        } else {
            None
        }
    }

    fn get_ref_mut(
        &self,
        (index, _): (Index, PhantomData<ProxyBlock>),
    ) -> Option<RefMut<ProxyBlock>> {
        let context = self.context.borrow_mut();
        if context.proxy_block.contains(index) {
            Some(RefMut::map(context, |ctx| &mut ctx.proxy_block[index]))
        } else {
            None
        }
    }
}
