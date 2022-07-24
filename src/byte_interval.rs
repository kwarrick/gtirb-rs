use anyhow::Result;

use std::collections::HashMap;

use crate::*;

#[derive(Default, Debug)]
pub struct ByteInterval {
    uuid: Uuid,
    size: u64,
    address: Option<Addr>,
    bytes: Vec<u8>,
    code_blocks: Vec<NodeBox<CodeBlock>>,
    data_blocks: Vec<NodeBox<DataBlock>>,
    symbolic_expressions: HashMap<u64, SymbolicExpression>,
    parent: WNodeBox<Section>,
}

impl ByteInterval {
    pub fn new(context: &mut Context) -> Node<ByteInterval> {
        let byte_interval = ByteInterval {
            uuid: Uuid::new_v4(),
            ..Default::default()
        };
        context.add_node(byte_interval)
    }

    pub(crate) fn load_protobuf(
        context: &mut Context,
        message: proto::ByteInterval,
    ) -> Result<Node<ByteInterval>> {
        let byte_interval = ByteInterval {
            uuid: crate::util::parse_uuid(&message.uuid)?,
            size: message.size,
            address: message.has_address.then(|| Addr(message.address)),
            bytes: message.contents,
            code_blocks: Vec::new(),
            data_blocks: Vec::new(),
            symbolic_expressions: HashMap::new(),
            parent: WNodeBox::new(),
        };

        let byte_interval = context.add_node(byte_interval);

        Ok(byte_interval)
    }

    pub(crate) fn set_parent(&mut self, parent: Option<&NodeBox<Section>>) {
        self.parent = match parent {
            Some(ptr) => Rc::downgrade(ptr),
            None => WNodeBox::new(),
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
    pub fn size(&self) -> u64 {
        return self.borrow().size;
    }

    pub fn set_size(&mut self, n: u64) {
        self.borrow_mut().size = n;
    }

    pub fn address(&self) -> Option<Addr> {
        return self.borrow().address;
    }

    pub fn set_address(&mut self, address: Option<Addr>) {
        self.borrow_mut().address = address;
    }

    pub fn initialized_size(&self) -> u64 {
        self.borrow().bytes.len() as u64
    }

    pub fn set_initialized_size(&mut self, n: u64) {
        self.borrow_mut().bytes.resize(n as usize, 0);
        if n > self.size() {
            self.set_size(n);
        }
    }

    pub fn bytes(&self) -> Ref<[u8]> {
        Ref::map(self.borrow(), |i| &i.bytes[..])
    }

    pub fn set_bytes<T: AsRef<[u8]>>(&mut self, bytes: T) {
        self.borrow_mut().bytes = bytes.as_ref().to_vec();
    }

    pub fn add_code_block(
        &mut self,
        code_block: Node<CodeBlock>,
    ) -> Node<CodeBlock> {
        code_block.inner.borrow_mut().set_parent(Some(&self.inner));
        self.borrow_mut()
            .code_blocks
            .push(Rc::clone(&code_block.inner));
        code_block
    }

    pub fn remove_code_block(&self, uuid: Uuid) -> Option<Node<CodeBlock>> {
        let mut byte_interval = self.inner.borrow_mut();
        if let Some(pos) = byte_interval
            .code_blocks
            .iter()
            .position(|m| m.borrow().uuid() == uuid)
        {
            let ptr = byte_interval.code_blocks.remove(pos);
            ptr.borrow_mut().set_parent(None);
            Some(Node::new(&self.context, ptr))
        } else {
            None
        }
    }

    pub fn code_blocks(&self) -> Iter<CodeBlock> {
        Iter {
            inner: Some(Ref::map(self.borrow(), |section| {
                &section.code_blocks[..]
            })),
            context: &self.context,
        }
    }

    pub fn data_blocks(&self) -> Iter<DataBlock> {
        Iter {
            inner: Some(Ref::map(self.borrow(), |section| {
                &section.data_blocks[..]
            })),
            context: &self.context,
        }
    }

    pub fn add_data_block(
        &mut self,
        data_block: Node<DataBlock>,
    ) -> Node<DataBlock> {
        data_block.inner.borrow_mut().set_parent(Some(&self.inner));
        self.borrow_mut()
            .data_blocks
            .push(Rc::clone(&data_block.inner));
        data_block
    }

    pub fn remove_data_block(&self, uuid: Uuid) -> Option<Node<DataBlock>> {
        let mut byte_interval = self.inner.borrow_mut();
        if let Some(pos) = byte_interval
            .data_blocks
            .iter()
            .position(|m| m.borrow().uuid() == uuid)
        {
            let ptr = byte_interval.data_blocks.remove(pos);
            ptr.borrow_mut().set_parent(None);
            Some(Node::new(&self.context, ptr))
        } else {
            None
        }
    }
}

impl Index for ByteInterval {
    fn insert(context: &mut Context, node: Self) -> NodeBox<Self> {
        let uuid = node.uuid();
        let boxed = Rc::new(RefCell::new(node));
        context
            .index
            .borrow_mut()
            .byte_intervals
            .insert(uuid, Rc::downgrade(&boxed));
        boxed
    }

    fn remove(context: &mut Context, ptr: &NodeBox<Self>) {
        let uuid = ptr.borrow().uuid();
        context.index.borrow_mut().byte_intervals.remove(&uuid);
    }

    fn search(context: &Context, uuid: &Uuid) -> Option<NodeBox<Self>> {
        context
            .index
            .borrow()
            .byte_intervals
            .get(uuid)
            .map(|ptr| ptr.upgrade())
            .flatten()
    }

    fn rooted(ptr: NodeBox<Self>) -> bool {
        ptr.borrow().parent.upgrade().is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_set_attributes() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        let mut module = ir.add_module(Module::new(&mut ctx, "dummy"));
        let mut section = module.add_section(Section::new(&mut ctx, ".dummy"));
        let mut interval =
            section.add_byte_interval(ByteInterval::new(&mut ctx));
        interval.set_size(0xDEAD);
        interval.set_address(Some(Addr(0xBEEF)));
        assert_eq!(interval.size(), 0xDEAD);
        assert_eq!(interval.address(), Some(Addr(0xBEEF)));
    }

    #[test]
    fn can_iterate_code_blocks() {
        let mut ctx = Context::new();
        let mut bi = ByteInterval::new(&mut ctx);
        let cb1 = CodeBlock::new(&mut ctx);
        let uuid1 = cb1.uuid();
        let cb2 = CodeBlock::new(&mut ctx);
        let uuid2 = cb2.uuid();
        bi.add_code_block(cb1);
        bi.add_code_block(cb2);
        assert_eq!(
            bi.code_blocks().map(|x| x.uuid()).collect::<Vec<Uuid>>(),
            vec![uuid1, uuid2]
        );
    }
}
