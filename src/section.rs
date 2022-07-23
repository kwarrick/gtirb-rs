use std::collections::HashSet;

use anyhow::{anyhow, Result};

use crate::*;

#[derive(Default, Debug, PartialEq)]
pub struct Section {
    uuid: Uuid,
    name: String,
    flags: HashSet<SectionFlag>,
    byte_intervals: Vec<NodeBox<ByteInterval>>,
    pub(crate) parent: Option<*const RefCell<Module>>,
}

impl Section {
    pub fn new(context: &mut Context, name: &str) -> Node<Section> {
        let section = Section {
            uuid: Uuid::new_v4(),
            name: name.to_owned(),
            ..Default::default()
        };
        context.add_node(section)
    }

    pub(crate) fn load_protobuf(
        context: &mut Context,
        message: proto::Section,
    ) -> Result<Node<Section>> {
        let section_flags: Result<HashSet<SectionFlag>> = message
            .section_flags
            .into_iter()
            .map(|i| {
                SectionFlag::from_i32(i).ok_or(anyhow!("Invalid FileFormat"))
            })
            .collect();

        let section = Section {
            parent: None,

            uuid: crate::util::parse_uuid(&message.uuid)?,
            name: message.name,
            flags: section_flags?,
            byte_intervals: Vec::new(),
        };

        let mut section = context.add_node(section);

        // Load ByteInterval protobuf messages.
        for m in message.byte_intervals.into_iter() {
            let byte_interval = ByteInterval::load_protobuf(context, m)?;
            section.add_byte_interval(byte_interval);
        }

        Ok(section)
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

    pub fn set_name<T: AsRef<str>>(&mut self, name: T) {
        self.borrow_mut().name = name.as_ref().to_owned();
    }

    pub fn flags(&self) -> HashSet<SectionFlag> {
        self.borrow().flags.clone()
    }

    pub fn add_flag(&mut self, flag: SectionFlag) {
        self.borrow_mut().flags.insert(flag);
    }

    pub fn remove_flag(&mut self, flag: SectionFlag) {
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

    pub fn add_byte_interval(
        &mut self,
        byte_interval: Node<ByteInterval>,
    ) -> Node<ByteInterval> {
        let ptr = Weak::into_raw(Rc::downgrade(&Rc::clone(&self.inner)));
        byte_interval.inner.borrow_mut().parent = Some(ptr);
        self.borrow_mut()
            .byte_intervals
            .push(Rc::clone(&byte_interval.inner));
        byte_interval
    }

    pub fn remove_byte_interval(
        &self,
        uuid: Uuid,
    ) -> Option<Node<ByteInterval>> {
        let mut section = self.inner.borrow_mut();
        if let Some(pos) = section
            .byte_intervals
            .iter()
            .position(|m| m.borrow().uuid() == uuid)
        {
            let ptr = section.byte_intervals.remove(pos);
            ptr.borrow_mut().parent = None;
            Some(Node::new(&self.context, ptr))
        } else {
            None
        }
    }

    pub fn byte_intervals(&self) -> Iter<ByteInterval> {
        Iter {
            inner: Some(Ref::map(self.borrow(), |section| {
                &section.byte_intervals[..]
            })),
            context: &self.context,
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
    //         context: self.context.clone(),
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
    //         context: self.context.clone(),
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

    fn search(context: &Context, uuid: &Uuid) -> Option<NodeBox<Self>> {
        context
            .index
            .borrow()
            .sections
            .get(uuid)
            .map(|ptr| ptr.upgrade())
            .flatten()
    }

    fn rooted(ptr: NodeBox<Self>) -> bool {
        ptr.borrow().parent.is_some()
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
        let mut section = module.add_section(Section::new(&mut ctx, ".text"));
        assert_eq!(section.name(), ".text");

        section.set_name(".data");
        assert_eq!(section.name(), ".data");
    }

    #[test]
    fn can_set_flags() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        let mut module = ir.add_module(Module::new(&mut ctx, "dummy"));
        let mut section = module.add_section(Section::new(&mut ctx, ".text"));
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
        let mut module = ir.add_module(Module::new(&mut ctx, "dummy"));

        let mut section = module.add_section(Section::new(&mut ctx, ".text"));
        assert_eq!(section.size(), None);
        assert_eq!(section.address(), None);

        let mut byte_interval =
            section.add_byte_interval(ByteInterval::new(&mut ctx));
        byte_interval.set_address(Some(Addr(5)));
        byte_interval.set_size(10);
        assert_eq!(section.size(), Some(10));
        assert_eq!(section.address(), Some(Addr(5)));

        let mut byte_interval =
            section.add_byte_interval(ByteInterval::new(&mut ctx));
        byte_interval.set_address(Some(Addr(15)));
        byte_interval.set_size(10);
        assert_eq!(section.size(), Some(20));
        assert_eq!(section.address(), Some(Addr(5)));

        section.add_byte_interval(ByteInterval::new(&mut ctx));
        assert_eq!(section.size(), None);
        assert_eq!(section.address(), None);
    }
}
