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
    pub fn new(context: &mut Context) -> ByteIntervalRef {
        let byte_interval = ByteInterval {
            uuid: Uuid::new_v4(),
            ..Default::default()
        };
        ByteIntervalRef::new(context.add_node(byte_interval))
    }

    pub(crate) fn load_protobuf(
        context: &mut Context,
        message: proto::ByteInterval,
    ) -> Result<ByteIntervalRef> {
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

        let byte_interval = ByteIntervalRef::new(context.add_node(byte_interval));

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

pub struct ByteIntervalRef {
    pub(crate) node: Node<ByteInterval>,
}

impl ByteIntervalRef {
    pub fn uuid(&self) -> Uuid {
        self.node.borrow().uuid
    }

    pub fn size(&self) -> u64 {
        return self.node.borrow().size;
    }

    pub fn set_size(&mut self, n: u64) {
        self.node.borrow_mut().size = n;
    }

    pub fn address(&self) -> Option<Addr> {
        return self.node.borrow().address;
    }

    pub fn set_address(&mut self, address: Option<Addr>) {
        self.node.borrow_mut().address = address;
    }

    pub fn initialized_size(&self) -> u64 {
        self.node.borrow().bytes.len() as u64
    }

    pub fn set_initialized_size(&mut self, n: u64) {
        self.node.borrow_mut().bytes.resize(n as usize, 0);
        if n > self.size() {
            self.set_size(n);
        }
    }

    pub fn bytes(&self) -> Ref<[u8]> {
        Ref::map(self.node.borrow(), |i| &i.bytes[..])
    }

    pub fn set_bytes<T: AsRef<[u8]>>(&mut self, bytes: T) {
        self.node.borrow_mut().bytes = bytes.as_ref().to_vec();
    }

    pub fn add_code_block(
        &mut self,
        code_block: &CodeBlockRef,
    ) {
        code_block.node.inner.borrow_mut().set_parent(Some(&self.node.inner));
        self.node.borrow_mut()
            .code_blocks
            .push(Rc::clone(&code_block.node.inner));
    }

    pub fn remove_code_block(&self, uuid: Uuid) -> Option<CodeBlockRef> {
        let mut byte_interval = self.node.inner.borrow_mut();
        if let Some(pos) = byte_interval
            .code_blocks
            .iter()
            .position(|m| m.borrow().uuid() == uuid)
        {
            let ptr = byte_interval.code_blocks.remove(pos);
            ptr.borrow_mut().set_parent(None);
            Some(CodeBlockRef::new(Node::new(&self.node.context, ptr)))
        } else {
            None
        }
    }

    pub fn code_blocks(&self) -> Iter<CodeBlock, CodeBlockRef> {
        Iter {
            inner: Some(Ref::map(self.node.borrow(), |section| {
                &section.code_blocks[..]
            })),
            context: &self.node.context,
            phantom: PhantomData,
        }
    }

    pub fn data_blocks(&self) -> Iter<DataBlock, DataBlockRef> {
        Iter {
            inner: Some(Ref::map(self.node.borrow(), |section| {
                &section.data_blocks[..]
            })),
            context: &self.node.context,
            phantom: PhantomData,
        }
    }

    pub fn add_data_block(
        &mut self,
        data_block: &DataBlockRef,
    ) {
        data_block.node.inner.borrow_mut().set_parent(Some(&self.node.inner));
        self.node.borrow_mut()
            .data_blocks
            .push(Rc::clone(&data_block.node.inner));
    }

    pub fn remove_data_block(&self, uuid: Uuid) -> Option<DataBlockRef> {
        let mut byte_interval = self.node.inner.borrow_mut();
        if let Some(pos) = byte_interval
            .data_blocks
            .iter()
            .position(|m| m.borrow().uuid() == uuid)
        {
            let ptr = byte_interval.data_blocks.remove(pos);
            ptr.borrow_mut().set_parent(None);
            Some(DataBlockRef::new(Node::new(&self.node.context, ptr)))
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

    fn rooted(ptr: NodeBox<Self>) -> bool {
        ptr.borrow().parent.upgrade().is_some()
    }
}

impl IsRefFor<ByteInterval> for ByteIntervalRef {
    fn new(node: Node<ByteInterval>) -> Self {
        Self { node: node }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_set_attributes() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        let mut module = Module::new(&mut ctx, "dummy");
        ir.add_module(&module);
        let mut section = Section::new(&mut ctx, ".dummy");
        module.add_section(&section);
        let mut interval = ByteInterval::new(&mut ctx);
        section.add_byte_interval(&interval);
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
        bi.add_code_block(&cb1);
        bi.add_code_block(&cb2);
        assert_eq!(
            bi.code_blocks().map(|x| x.uuid()).collect::<Vec<Uuid>>(),
            vec![uuid1, uuid2]
        );
    }
}
