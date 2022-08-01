use std::collections::HashSet;

use anyhow::{anyhow, Result};

use crate::*;

#[derive(Default, Debug, gtirb_derive::Node)]
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
        context.add_section(section)
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

        let mut section = context.add_section(section);

        // Load ByteInterval protobuf messages.
        for m in message.byte_intervals.into_iter() {
            let mut byte_interval = ByteInterval::load_protobuf(context, m)?;
            section.add_byte_interval(&mut byte_interval);
        }

        Ok(section)
    }
}

impl SectionRef {
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

    pub fn add_byte_interval(&mut self, byte_interval: &mut ByteIntervalRef) {
        byte_interval
            .borrow_mut()
            .set_parent(Some(&self.inner));
        self.borrow_mut()
            .byte_intervals
            .push(Rc::clone(&byte_interval.get_inner()));
    }

    pub fn remove_byte_interval(
        &self,
        uuid: Uuid,
    ) -> Option<ByteIntervalRef> {
        let mut section = self.inner.borrow_mut();
        if let Some(pos) = section
            .byte_intervals
            .iter()
            .position(|m| m.borrow().uuid() == uuid)
        {
            let ptr = section.byte_intervals.remove(pos);
            ptr.borrow_mut().set_parent(None);
            Some(ByteIntervalRef::new(&self.context, ptr))
        } else {
            None
        }
    }

    pub fn byte_intervals(&self) -> Iter<ByteInterval, ByteIntervalRef> {
        Iter {
            inner: Some(Ref::map(self.borrow(), |section| {
                &section.byte_intervals[..]
            })),
            context: &self.context,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_set_attributes() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        let mut module = Module::new(&mut ctx, "dummy");
        ir.add_module(&mut module);
        let mut section = Section::new(&mut ctx, ".text");
        module.add_section(&mut section);
        assert_eq!(section.name(), ".text");

        section.set_name(".data");
        assert_eq!(section.name(), ".data");
    }

    #[test]
    fn can_set_flags() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        let mut module = Module::new(&mut ctx, "dummy");
        ir.add_module(&mut module);
        let mut section = Section::new(&mut ctx, ".text");
        module.add_section(&mut section);
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
        ir.add_module(&mut module);

        let mut section = Section::new(&mut ctx, ".text");
        module.add_section(&mut section);
        assert_eq!(section.size(), None);
        assert_eq!(section.address(), None);

        let mut byte_interval = ByteInterval::new(&mut ctx);
        section.add_byte_interval(&mut byte_interval);
        byte_interval.set_address(Some(Addr(5)));
        byte_interval.set_size(10);
        assert_eq!(section.size(), Some(10));
        assert_eq!(section.address(), Some(Addr(5)));

        let mut byte_interval = ByteInterval::new(&mut ctx);
        section.add_byte_interval(&mut byte_interval);
        byte_interval.set_address(Some(Addr(15)));
        byte_interval.set_size(10);
        assert_eq!(section.size(), Some(20));
        assert_eq!(section.address(), Some(Addr(5)));

        section.add_byte_interval(&mut ByteInterval::new(&mut ctx));
        assert_eq!(section.size(), None);
        assert_eq!(section.address(), None);
    }
}
