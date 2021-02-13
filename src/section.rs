use std::collections::HashSet;

use crate::*;

#[derive(Default, Debug, PartialEq)]
pub(crate) struct Section {
    pub(crate) parent: Option<Index>,

    uuid: Uuid,
    name: String,
    flags: HashSet<SectionFlag>,
    byte_intervals: Vec<Index>,
}

impl Section {
    pub fn new(name: &str) -> Self {
        Section {
            uuid: Uuid::new_v4(),
            ..Default::default()
        }
    }
}

impl Unique for Section {
    fn uuid(&self) -> Uuid {
        self.uuid
    }

    fn set_uuid(&mut self, uuid: Uuid) {
        self.uuid = uuid;
    }
}

impl Node<Section> {
    pub fn name(&self) -> String {
        self.borrow().name.to_owned()
    }

    pub fn set_name<T: AsRef<str>>(&self, name: T) {
        self.borrow_mut().name = name.as_ref().to_owned();
    }
    pub fn flags(&self) -> HashSet<SectionFlag> {
        self.borrow().flags.clone()
    }

    pub fn add_flag(&self, flag: SectionFlag) {
        self.borrow_mut().flags.insert(flag);
    }

    pub fn remove_flag(&self, flag: SectionFlag) {
        self.borrow_mut().flags.remove(&flag);
    }

    pub fn byte_intervals(&self) -> NodeIterator<ByteInterval> {
        self.node_iter()
    }

    pub fn add_byte_interval(
        &self,
        byte_interval: ByteInterval,
    ) -> Node<ByteInterval> {
        self.add_node(byte_interval)
    }

    pub fn remove_byte_interval(&self, node: Node<ByteInterval>) {
        self.remove_node(node);
    }

    pub fn code_blocks(&self) -> NodeIterator<CodeBlock> {
        let iter = self.byte_intervals().flat_map(|interval| {
            <Node<ByteInterval> as Parent<CodeBlock>>::nodes(&interval)
                .clone()
                .into_iter()
        });
        NodeIterator {
            iter: Box::new(iter),
            context: self.context.clone(),
            kind: PhantomData,
        }
    }

    pub fn data_blocks(&self) -> NodeIterator<DataBlock> {
        let iter = self.byte_intervals().flat_map(|interval| {
            <Node<ByteInterval> as Parent<DataBlock>>::nodes(&interval)
                .clone()
                .into_iter()
        });
        NodeIterator {
            iter: Box::new(iter),
            context: self.context.clone(),
            kind: PhantomData,
        }
    }
}

impl Indexed<Section> for Node<Section> {
    fn arena(&self) -> Ref<Arena<Section>> {
        Ref::map(self.context.borrow(), |ctx| &ctx.section)
    }

    fn arena_mut(&self) -> RefMut<Arena<Section>> {
        RefMut::map(self.context.borrow_mut(), |ctx| &mut ctx.section)
    }
}

impl Child<Module> for Node<Section> {
    fn parent(&self) -> (Option<Index>, PhantomData<Module>) {
        (self.borrow().parent, PhantomData)
    }

    fn set_parent(&self, (index, _): (Index, PhantomData<Module>)) {
        self.borrow_mut().parent.replace(index);
    }
}

impl Parent<ByteInterval> for Node<Section> {
    fn nodes(&self) -> Ref<Vec<Index>> {
        Ref::map(self.borrow(), |section| &section.byte_intervals)
    }

    fn nodes_mut(&self) -> RefMut<Vec<Index>> {
        RefMut::map(self.borrow_mut(), |section| &mut section.byte_intervals)
    }

    fn node_arena(&self) -> Ref<Arena<ByteInterval>> {
        Ref::map(self.context.borrow(), |ctx| &ctx.byte_interval)
    }
    fn node_arena_mut(&self) -> RefMut<Arena<ByteInterval>> {
        RefMut::map(self.context.borrow_mut(), |ctx| &mut ctx.byte_interval)
    }
}
