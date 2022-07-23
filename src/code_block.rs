use crate::*;

#[derive(Default, Debug, PartialEq)]
pub struct CodeBlock {
    uuid: Uuid,
    pub(crate) parent: Option<*const RefCell<ByteInterval>>,
}

impl CodeBlock {
    pub fn new(ctx: &mut Context) -> Node<Self> {
        let block = Self {
            uuid: Uuid::new_v4(),
            ..Default::default()
        };
        ctx.add_node(block)
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

impl Index for CodeBlock {
    fn insert(context: &mut Context, node: Self) -> NodeBox<Self> {
        let uuid = node.uuid();
        let boxed = Rc::new(RefCell::new(node));
        context
            .index
            .borrow_mut()
            .code_blocks
            .insert(uuid, Rc::downgrade(&boxed));
        boxed
    }

    fn remove(context: &mut Context, ptr: &NodeBox<Self>) {
        let uuid = ptr.borrow().uuid();
        context.index.borrow_mut().code_blocks.remove(&uuid);
    }

    fn search(context: &Context, uuid: &Uuid) -> Option<NodeBox<Self>> {
        context
            .index
            .borrow()
            .code_blocks
            .get(uuid)
            .map(|ptr| ptr.upgrade())
            .flatten()
    }

    fn rooted(ptr: NodeBox<Self>) -> bool {
        ptr.borrow().parent.is_some()
    }
}
