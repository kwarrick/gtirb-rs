use anyhow::Result;

use crate::*;

#[derive(Default, Debug, PartialEq)]
pub struct ProxyBlock {
    uuid: Uuid,
    pub(crate) parent: Option<*const RefCell<Module>>,
}

impl ProxyBlock {
    pub fn new(context: &mut Context) -> Node<ProxyBlock> {
        let proxy_block = ProxyBlock {
            uuid: Uuid::new_v4(),
            ..Default::default()
        };
        context.add_node(proxy_block)
    }

    pub(crate) fn load_protobuf(
        context: &mut Context,
        message: proto::ProxyBlock,
    ) -> Result<Node<ProxyBlock>> {
        let proxy_block = ProxyBlock {
            parent: None,
            uuid: crate::util::parse_uuid(&message.uuid)?,
        };

        let proxy_block = context.add_node(proxy_block);

        Ok(proxy_block)
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

impl Index for ProxyBlock {
    fn insert(context: &mut Context, node: Self) -> NodeBox<Self> {
        let uuid = node.uuid();
        let boxed = Rc::new(RefCell::new(node));
        context
            .index
            .borrow_mut()
            .proxy_blocks
            .insert(uuid, Rc::downgrade(&boxed));
        boxed
    }

    fn remove(context: &mut Context, ptr: &NodeBox<Self>) {
        let uuid = ptr.borrow().uuid();
        context.index.borrow_mut().modules.remove(&uuid);
    }

    fn search(context: &Context, uuid: &Uuid) -> Option<NodeBox<Self>> {
        context
            .index
            .borrow()
            .proxy_blocks
            .get(uuid)
            .map(|ptr| ptr.upgrade())
            .flatten()
    }

    fn rooted(ptr: NodeBox<Self>) -> bool {
        ptr.borrow().parent.is_some()
    }
}
