use anyhow::Result;

use crate::*;

#[derive(Default, Debug, PartialEq)]
pub struct ProxyBlock {
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
    pub(crate) fn load_protobuf(
        context: Rc<RefCell<Context>>,
        message: proto::ProxyBlock,
    ) -> Result<Index> {
        let proxy_block = ProxyBlock {
            parent: None,
            uuid: crate::util::parse_uuid(&message.uuid)?,
        };
        Ok(context.borrow_mut().proxy_block.insert(proxy_block))
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
    fn arena(&self) -> Ref<Arena<ProxyBlock>> {
        Ref::map(self.context.borrow(), |ctx| &ctx.proxy_block)
    }

    fn arena_mut(&self) -> RefMut<Arena<ProxyBlock>> {
        RefMut::map(self.context.borrow_mut(), |ctx| &mut ctx.proxy_block)
    }
}

impl Child<Module> for Node<ProxyBlock> {
    fn parent(&self) -> (Option<Index>, PhantomData<Module>) {
        (self.borrow().parent, PhantomData)
    }

    fn set_parent(&self, (index, _): (Index, PhantomData<Module>)) {
        self.borrow_mut().parent.replace(index);
    }
}
