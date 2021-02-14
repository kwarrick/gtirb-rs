use std::collections::HashSet;

use crate::*;

#[derive(Default, Debug, PartialEq)]
pub struct Section {
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
            name: name.to_owned(),
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

    pub fn is_flag_set(&self, flag: SectionFlag) -> bool {
        self.borrow().flags.contains(&flag)
    }

    pub fn size(&self) -> Option<u64> {
        let min: Option<Addr> =
            self.byte_intervals().map(|i| i.address()).min().flatten();
        let max: Option<Addr> = self
            .byte_intervals()
            .map(|i| i.address().map(|a| a + i.size().into()))
            .max()
            .flatten();
        if let (Some(min), Some(max)) = (min, max) {
            Some(u64::from(max - min))
        } else {
            None
        }
    }

    pub fn address(&self) -> Option<Addr> {
        self.byte_intervals().map(|i| i.address()).min().flatten()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_set_attributes() {
        let ir = IR::new();
        let module = ir.add_module(Module::new("dummy"));
        let section = module.add_section(Section::new(".text"));
        assert_eq!(section.name(), ".text");

        section.set_name(".data");
        assert_eq!(section.name(), ".data");
    }

    #[test]
    fn can_set_flags() {
        let ir = IR::new();
        let module = ir.add_module(Module::new("dummy"));
        let section = module.add_section(Section::new(".text"));
        assert_eq!(section.name(), ".text");

        assert!(section.flags().is_empty());
        section.add_flag(SectionFlag::Readable);
        section.add_flag(SectionFlag::Writable);
        assert!(section.is_flag_set(SectionFlag::Readable));
        assert!(section.is_flag_set(SectionFlag::Writable));

        section.remove_flag(SectionFlag::Writable);
        assert!(!section.is_flag_set(SectionFlag::Writable));
    }

    #[test]
    fn can_calculate_size() {
        let ir = IR::new();
        let module = ir.add_module(Module::new("dummy"));

        let section = module.add_section(Section::new(".text"));
        assert_eq!(section.size(), None);
        assert_eq!(section.address(), None);

        let byte_interval = section.add_byte_interval(ByteInterval::new());
        byte_interval.set_address(Some(Addr(5)));
        byte_interval.set_size(10);
        assert_eq!(section.size(), Some(10));
        assert_eq!(section.address(), Some(Addr(5)));

        let byte_interval = section.add_byte_interval(ByteInterval::new());
        byte_interval.set_address(Some(Addr(15)));
        byte_interval.set_size(10);
        assert_eq!(section.size(), Some(20));
        assert_eq!(section.address(), Some(Addr(5)));

        section.add_byte_interval(ByteInterval::new());
        assert_eq!(section.size(), None);
        assert_eq!(section.address(), None);
    }
}
