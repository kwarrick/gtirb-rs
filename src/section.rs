use std::collections::HashSet;

use anyhow::{anyhow, Result};

use crate::*;

#[derive(Default, Debug, PartialEq)]
pub struct Section {
    uuid: Uuid,
    name: String,
    flags: HashSet<SectionFlag>,
    // byte_intervals: Vec<Index>,
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

        // TODO:
        // let byte_intervals = message
        //     .byte_intervals
        //     .into_iter()
        //     .map(|m| ByteInterval::load_protobuf(context.clone(), m))
        //     .collect::<Result<Vec<Index>>>()?;

        let section = Section {
            parent: None,

            uuid: crate::util::parse_uuid(&message.uuid)?,
            name: message.name,
            flags: section_flags?,
            // byte_intervals: byte_intervals,
        };

        let section = context.add_node(section);

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
        // TODO:
        // let min: Option<Addr> =
        //     self.byte_intervals().map(|i| i.address()).min().flatten();
        // let max: Option<Addr> = self
        //     .byte_intervals()
        //     .map(|i| i.address().map(|a| a + i.size().into()))
        //     .max()
        //     .flatten();
        // if let (Some(min), Some(max)) = (min, max) {
        //     Some(u64::from(max - min))
        // } else {
        //     None
        // }
        None
    }

    pub fn address(&self) -> Option<Addr> {
        // TODO:
        // self.byte_intervals().map(|i| i.address()).min().flatten()
        None
    }

    // pub fn byte_intervals(&self) -> NodeIterator<ByteInterval> {
    //     self.node_iter()
    // }

    // pub fn add_byte_interval(
    //     &self,
    //     byte_interval: ByteInterval,
    // ) -> Node<ByteInterval> {
    //     self.add_node(byte_interval)
    // }

    // pub fn remove_byte_interval(&self, node: Node<ByteInterval>) {
    //     self.remove_node(node);
    // }

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
            .insert(uuid, Rc::clone(&boxed));
        boxed
    }

    fn remove(context: &mut Context, ptr: NodeBox<Self>) -> NodeBox<Self> {
        let uuid = ptr.borrow().uuid();
        context.index.borrow_mut().modules.remove(&uuid);
        ptr
    }

    fn search(context: &Context, uuid: &Uuid) -> Option<NodeBox<Self>> {
        context
            .index
            .borrow()
            .sections
            .get(uuid)
            .map(|ptr| Rc::clone(&ptr))
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
