use crate::*;

#[derive(Default, Debug, PartialEq)]
pub struct DataBlock {
    uuid: Uuid,
    pub(crate) parent: Option<*const RefCell<ByteInterval>>,
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

impl Index for DataBlock {
    fn insert(context: &mut Context, node: Self) -> NodeBox<Self> {
        let uuid = node.uuid();
        let boxed = Rc::new(RefCell::new(node));
        context
            .index
            .borrow_mut()
            .data_blocks
            .insert(uuid, Rc::clone(&boxed));
        boxed
    }

    fn remove(context: &mut Context, ptr: NodeBox<Self>) -> NodeBox<Self> {
        let uuid = ptr.borrow().uuid();
        context.index.borrow_mut().data_blocks.remove(&uuid);
        ptr
    }

    fn search(context: &Context, uuid: &Uuid) -> Option<NodeBox<Self>> {
        context
            .index
            .borrow()
            .data_blocks
            .get(uuid)
            .map(|ptr| Rc::clone(&ptr))
    }

    fn rooted(ptr: NodeBox<Self>) -> bool {
        ptr.borrow().parent.is_some()
    }
}
