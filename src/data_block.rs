use crate::*;

#[derive(Default, Debug)]
pub struct DataBlock {
    uuid: Uuid,
    parent: WNodeBox<ByteInterval>,
}

impl DataBlock {
    pub fn new(ctx: &mut Context) -> Node<Self> {
        let block = Self {
            uuid: Uuid::new_v4(),
            ..Default::default()
        };
        ctx.add_node(block)
    }

    pub(crate) fn set_parent(&mut self, parent: Option<&NodeBox<ByteInterval>>) {
        self.parent = match parent {
            Some(ptr) => Rc::downgrade(ptr),
            None => WNodeBox::new(),
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

impl Index for DataBlock {
    fn insert(context: &mut Context, node: Self) -> NodeBox<Self> {
        let uuid = node.uuid();
        let boxed = Rc::new(RefCell::new(node));
        context
            .index
            .borrow_mut()
            .data_blocks
            .insert(uuid, Rc::downgrade(&boxed));
        boxed
    }

    fn remove(context: &mut Context, ptr: &NodeBox<Self>) {
        let uuid = ptr.borrow().uuid();
        context.index.borrow_mut().data_blocks.remove(&uuid);
    }

    fn search(context: &Context, uuid: &Uuid) -> Option<NodeBox<Self>> {
        context
            .index
            .borrow()
            .data_blocks
            .get(uuid)
            .map(|ptr| ptr.upgrade())
            .flatten()
    }

    fn rooted(ptr: NodeBox<Self>) -> bool {
        ptr.borrow().parent.upgrade().is_some()
    }
}
