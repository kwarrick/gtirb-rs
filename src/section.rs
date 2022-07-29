use std::collections::HashSet;

use anyhow::{anyhow, Result};

use crate::*;

#[derive(Default, Debug)]
pub struct Section {
    uuid: Uuid,
    name: String,
    flags: HashSet<SectionFlag>,
    byte_intervals: Vec<NodeBox<ByteInterval>>,
    parent: WNodeBox<Module>,
}

impl Section {
    pub fn new(context: &mut Context, name: &str) -> SectionRef {
        let section = Section {
            uuid: Uuid::new_v4(),
            name: name.to_owned(),
            ..Default::default()
        };
        SectionRef::new(context.add_node(section))
    }

    pub(crate) fn load_protobuf(
        context: &mut Context,
        message: proto::Section,
    ) -> Result<SectionRef> {
        let section_flags: Result<HashSet<SectionFlag>> = message
            .section_flags
            .into_iter()
            .map(|i| {
                SectionFlag::from_i32(i).ok_or(anyhow!("Invalid FileFormat"))
            })
            .collect();

        let section = Section {
            parent: WNodeBox::new(),

            uuid: crate::util::parse_uuid(&message.uuid)?,
            name: message.name,
            flags: section_flags?,
            byte_intervals: Vec::new(),
        };

        let mut section = SectionRef::new(context.add_node(section));

        // Load ByteInterval protobuf messages.
        for m in message.byte_intervals.into_iter() {
            let byte_interval = ByteInterval::load_protobuf(context, m)?;
            section.add_byte_interval(&byte_interval);
        }

        Ok(section)
    }

    pub(crate) fn set_parent(&mut self, parent: Option<&NodeBox<Module>>) {
        self.parent = match parent {
            Some(ptr) => Rc::downgrade(ptr),
            None => WNodeBox::new(),
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

pub struct SectionRef {
    pub(crate) node: Node<Section>,
}

impl SectionRef {
    pub fn uuid(&self) -> Uuid {
        self.node.borrow().uuid
    }

    pub fn name(&self) -> String {
        self.node.borrow().name.to_owned()
    }

    pub fn set_name<T: AsRef<str>>(&mut self, name: T) {
        self.node.borrow_mut().name = name.as_ref().to_owned();
    }

    pub fn flags(&self) -> HashSet<SectionFlag> {
        self.node.borrow().flags.clone()
    }

    pub fn add_flag(&mut self, flag: SectionFlag) {
        self.node.borrow_mut().flags.insert(flag);
    }

    pub fn remove_flag(&mut self, flag: SectionFlag) {
        self.node.borrow_mut().flags.remove(&flag);
    }

    pub fn is_flag_set(&self, flag: SectionFlag) -> bool {
        self.node.borrow().flags.contains(&flag)
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

    pub fn add_byte_interval(&mut self, byte_interval: &ByteIntervalRef) {
        byte_interval
            .node
            .inner
            .borrow_mut()
            .set_parent(Some(&self.node.inner));
        self.node
            .borrow_mut()
            .byte_intervals
            .push(Rc::clone(&byte_interval.node.inner));
    }

    pub fn remove_byte_interval(
        &self,
        uuid: Uuid,
    ) -> Option<ByteIntervalRef> {
        let mut section = self.node.inner.borrow_mut();
        if let Some(pos) = section
            .byte_intervals
            .iter()
            .position(|m| m.borrow().uuid() == uuid)
        {
            let ptr = section.byte_intervals.remove(pos);
            ptr.borrow_mut().set_parent(None);
            Some(ByteIntervalRef::new(Node::new(&self.node.context, ptr)))
        } else {
            None
        }
    }

    pub fn byte_intervals(&self) -> Iter<ByteInterval, ByteIntervalRef> {
        Iter {
            inner: Some(Ref::map(self.node.borrow(), |section| {
                &section.byte_intervals[..]
            })),
            context: &self.node.context,
            phantom: PhantomData,
        }
    }

    // pub fn code_blocks(&self) -> NodeIterator<CodeBlock> {
    //     let iter = self.byte_intervals().flat_map(|interval| {
    //         <Node<ByteInterval> as Parent<CodeBlock>>::nodes(&interval)
    //             .clone()
    //             .into_iter()
    //     });
    //     NodeIterator {
    //         iter: Box::new(iter),
    //         context: self.node.context.clone(),
    //         kind: PhantomData,
    //     }
    // }

    // pub fn data_blocks(&self) -> NodeIterator<DataBlock> {
    //     let iter = self.byte_intervals().flat_map(|interval| {
    //         <Node<ByteInterval> as Parent<DataBlock>>::nodes(&interval)
    //             .clone()
    //             .into_iter()
    //     });
    //     NodeIterator {
    //         iter: Box::new(iter),
    //         context: self.node.context.clone(),
    //         kind: PhantomData,
    //     }
    // }
}

impl Index for Section {
    fn insert(context: &mut Context, node: Self) -> NodeBox<Self> {
        let uuid = node.uuid();
        let boxed = Rc::new(RefCell::new(node));
        context
            .index
            .borrow_mut()
            .sections
            .insert(uuid, Rc::downgrade(&boxed));
        boxed
    }

    fn remove(context: &mut Context, ptr: &NodeBox<Self>) {
        let uuid = ptr.borrow().uuid();
        context.index.borrow_mut().modules.remove(&uuid);
    }

    fn rooted(ptr: NodeBox<Self>) -> bool {
        ptr.borrow().parent.upgrade().is_some()
    }
}

impl IsRefFor<Section> for SectionRef {
    fn new(node: Node<Section>) -> Self {
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
        let mut section = Section::new(&mut ctx, ".text");
        module.add_section(&section);
        assert_eq!(section.name(), ".text");

        section.set_name(".data");
        assert_eq!(section.name(), ".data");
    }

    #[test]
    fn can_set_flags() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        let mut module = Module::new(&mut ctx, "dummy");
        ir.add_module(&module);
        let mut section = Section::new(&mut ctx, ".text");
        module.add_section(&section);
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
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        let mut module = Module::new(&mut ctx, "dummy");
        ir.add_module(&module);

        let mut section = Section::new(&mut ctx, ".text");
        module.add_section(&section);
        assert_eq!(section.size(), None);
        assert_eq!(section.address(), None);

        let mut byte_interval = ByteInterval::new(&mut ctx);
        section.add_byte_interval(&byte_interval);
        byte_interval.set_address(Some(Addr(5)));
        byte_interval.set_size(10);
        assert_eq!(section.size(), Some(10));
        assert_eq!(section.address(), Some(Addr(5)));

        let mut byte_interval = ByteInterval::new(&mut ctx);
        section.add_byte_interval(&byte_interval);
        byte_interval.set_address(Some(Addr(15)));
        byte_interval.set_size(10);
        assert_eq!(section.size(), Some(20));
        assert_eq!(section.address(), Some(Addr(5)));

        section.add_byte_interval(&ByteInterval::new(&mut ctx));
        assert_eq!(section.size(), None);
        assert_eq!(section.address(), None);
    }
}
