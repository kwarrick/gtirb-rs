use anyhow::{anyhow, Result};

use crate::*;

#[derive(Debug, PartialEq)]
pub struct Module {
    uuid: Uuid,
    name: String,
    binary_path: String,
    entry_point: Option<Uuid>,
    byte_order: ByteOrder,
    isa: ISA,
    rebase_delta: i64,
    preferred_address: Addr,
    file_format: FileFormat,
    // sections: Vec<Index>,
    // symbols: Vec<Index>,
    // proxy_blocks: Vec<Index>,
    pub(crate) parent: *mut IR,
}

impl Unique for Module {
    fn uuid(&self) -> Uuid {
        self.uuid
    }

    fn set_uuid(&mut self, uuid: Uuid) {
        self.uuid = uuid;
    }
}

impl Module {
    pub fn new(name: &str) -> Self {
        Module {
            uuid: Uuid::new_v4(),
            name: name.to_owned(),
            binary_path: String::new(),
            entry_point: None,
            byte_order: ByteOrder::Undefined,
            isa: ISA::Undefined,
            rebase_delta: 0,
            preferred_address: Addr(0),
            file_format: FileFormat::FormatUndefined,
            parent: std::ptr::null_mut(),
        }
    }

    pub(crate) fn load_protobuf(message: proto::Module) -> Result<Module> {
        let format = FileFormat::from_i32(message.file_format)
            .ok_or(anyhow!("Invalid FileFormat"))?;

        let isa = ISA::from_i32(message.isa).ok_or(anyhow!("Invalid ISA"))?;

        let byte_order = ByteOrder::from_i32(message.byte_order)
            .ok_or(anyhow!("Invalid ByteOrder"))?;

        // let sections = message
        //     .sections
        //     .into_iter()
        //     .map(|m| Section::load_protobuf(context.clone(), m))
        //     .collect::<Result<Vec<Index>>>()?;

        // let symbols = message
        //     .symbols
        //     .into_iter()
        //     .map(|m| Symbol::load_protobuf(context.clone(), m))
        //     .collect::<Result<Vec<Index>>>()?;

        // let proxy_blocks = message
        //     .proxies
        //     .into_iter()
        //     .map(|m| ProxyBlock::load_protobuf(context.clone(), m))
        //     .collect::<Result<Vec<Index>>>()?;

        let module = Module {
            uuid: crate::util::parse_uuid(&message.uuid)?,
            name: message.name,
            binary_path: message.binary_path,
            entry_point: Some(crate::util::parse_uuid(&message.entry_point)?),
            byte_order: byte_order,
            isa: isa,
            rebase_delta: message.rebase_delta,
            preferred_address: Addr(message.preferred_addr),
            file_format: format,
            // sections: sections,
            // symbols: symbols,
            // proxy_blocks: proxy_blocks,
            parent: std::ptr::null_mut(),
        };

        Ok(module)
    }

    pub fn ir(&self) -> Option<&IR> {
        if self.parent.is_null() {
            None
        } else {
            unsafe { Some(&*self.parent) }
        }
    }

    pub fn name(&self) -> String {
        self.name.to_owned()
    }

    pub fn set_name<T: AsRef<str>>(&mut self, name: T) {
        self.name = name.as_ref().to_owned();
    }

    pub fn binary_path(&self) -> String {
        self.binary_path.to_owned()
    }

    pub fn set_binary_path<T: AsRef<str>>(&mut self, binary_path: T) {
        self.binary_path = binary_path.as_ref().to_owned();
    }

    pub fn file_format(&self) -> FileFormat {
        self.file_format
    }

    pub fn set_file_format(&mut self, file_format: FileFormat) {
        self.file_format = file_format;
    }

    pub fn isa(&self) -> ISA {
        self.isa
    }

    pub fn set_isa(&mut self, isa: ISA) {
        self.isa = isa;
    }

    // pub fn entry_point(&self) -> Option<Node<CodeBlock>> {
    //     self.borrow()
    //         .entry_point
    //         .and_then(|uuid| self.ir().find_node(uuid))
    // }

    // pub fn set_entry_point(&self, block: Node<CodeBlock>) {
    //     self.borrow_mut().entry_point.replace(block.uuid());
    // }

    pub fn byte_order(&self) -> ByteOrder {
        self.byte_order
    }

    pub fn set_byte_order(&mut self, byte_order: ByteOrder) {
        self.byte_order = byte_order;
    }

    pub fn preferred_address(&self) -> Addr {
        self.preferred_address
    }

    pub fn set_preferred_address(&mut self, preferred_address: Addr) {
        self.preferred_address = preferred_address;
    }

    pub fn rebase_delta(&self) -> i64 {
        self.rebase_delta
    }

    pub fn set_rebase_delta(&mut self, rebase_delta: i64) {
        self.rebase_delta = rebase_delta;
    }

    pub fn is_relocated(&self) -> bool {
        self.rebase_delta != 0
    }

    // pub fn sections(&self) -> Iter<Section> {
    //     self.sections.iter()
    // }

    // pub fn add_section(&self, section: Section) -> Node<Section> {
    //     self.add_node(section)
    // }

    // pub fn remove_section(&self, node: Node<Section>) {
    //     self.remove_node(node);
    // }

    // pub fn proxy_blocks(&self) -> NodeIterator<ProxyBlock> {
    //     self.node_iter()
    // }

    // pub fn add_proxy_block(&self, proxy_block: ProxyBlock) -> Node<ProxyBlock> {
    //     self.add_node(proxy_block)
    // }

    // pub fn remove_proxy_block(&self, node: Node<ProxyBlock>) {
    //     self.remove_node(node);
    // }

    // pub fn symbols(&self) -> NodeIterator<Symbol> {
    //     self.node_iter()
    // }

    // pub fn add_symbol(&self, symbol: Symbol) -> Node<Symbol> {
    //     self.add_node(symbol)
    // }

    // pub fn remove_symbol(&self, node: Node<Symbol>) {
    //     self.remove_node(node);
    // }

    // pub fn size(&self) -> Option<u64> {
    //     let min: Option<Addr> =
    //         self.sections().map(|i| i.address()).min().flatten();
    //     let max: Option<Addr> = self
    //         .sections()
    //         .map(|i| {
    //             i.address()
    //                 .zip(i.size())
    //                 .map(|(addr, size)| addr + size.into())
    //         })
    //         .max()
    //         .flatten();
    //     if let (Some(min), Some(max)) = (min, max) {
    //         Some(u64::from(max - min))
    //     } else {
    //         None
    //     }
    // }

    // pub fn address(&self) -> Option<Addr> {
    //     self.sections().map(|s| s.address()).min().flatten()
    // }

    // pub fn byte_intervals(&self) -> NodeIterator<ByteInterval> {
    //     let iter = self.sections().flat_map(|interval| {
    //         <Node<Section> as Parent<ByteInterval>>::nodes(&interval)
    //             .clone()
    //             .into_iter()
    //     });
    //     NodeIterator {
    //         iter: Box::new(iter),
    //         context: self.context.clone(),
    //         kind: PhantomData,
    //     }
    // }

    // pub fn code_blocks(&self) -> NodeIterator<CodeBlock> {
    //     let iter = self.sections().flat_map(|section| {
    //         section.byte_intervals().flat_map(|interval| {
    //             <Node<ByteInterval> as Parent<CodeBlock>>::nodes(&interval)
    //                 .clone()
    //                 .into_iter()
    //         })
    //     });
    //     NodeIterator {
    //         iter: Box::new(iter),
    //         context: self.context.clone(),
    //         kind: PhantomData,
    //     }
    // }

    // pub fn data_blocks(&self) -> NodeIterator<DataBlock> {
    //     let iter = self.sections().flat_map(|section| {
    //         section.byte_intervals().flat_map(|interval| {
    //             <Node<ByteInterval> as Parent<DataBlock>>::nodes(&interval)
    //                 .clone()
    //                 .into_iter()
    //         })
    //     });
    //     NodeIterator {
    //         iter: Box::new(iter),
    //         context: self.context.clone(),
    //         kind: PhantomData,
    //     }
    // }

    // symbolic_expressions()
    // get_symbol_reference<T>(symbol: Symbol) -> Node<T>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_module_is_unique() {
        assert_ne!(Module::new("a"), Module::new("a"));
    }

    //     #[test]
    //     fn new_module_is_empty() {
    //         let ir = IR::new();
    //         let module = ir.add_module(Module::new("dummy"));

    //         assert_eq!(module.symbols().count(), 0);
    //         assert_eq!(module.sections().count(), 0);
    //         assert_eq!(module.proxy_blocks().count(), 0);
    //     }

    //     #[test]
    //     fn can_set_binary_path() {
    //         let ir = IR::new();
    //         let path = "/home/gt/irb/foo";
    //         let module = ir.add_module(Module::new("dummy"));
    //         module.set_binary_path(path);
    //         assert_eq!(module.binary_path(), path);
    //     }

    //     #[test]
    //     fn can_get_file_format_default() {
    //         let ir = IR::new();
    //         let module = ir.add_module(Module::new("dummy"));
    //         assert_eq!(module.file_format(), FileFormat::FormatUndefined);
    //     }

    //     #[test]
    //     fn can_set_file_format() {
    //         let ir = IR::new();
    //         let module = ir.add_module(Module::new("dummy"));
    //         module.set_file_format(FileFormat::Coff);
    //         assert_eq!(module.file_format(), FileFormat::Coff);

    //         module.set_file_format(FileFormat::Macho);
    //         assert_eq!(module.file_format(), FileFormat::Macho);
    //     }

    //     #[test]
    //     fn can_set_name() {
    //         let ir = IR::new();
    //         let module = ir.add_module(Module::new("dummy"));
    //         module.set_name("example");
    //         assert_eq!(module.name(), "example");
    //     }

    //     #[test]
    //     fn can_relocate_module() {
    //         let ir = IR::new();
    //         let module = ir.add_module(Module::new("dummy"));
    //         assert!(!module.is_relocated());
    //         assert_eq!(module.rebase_delta(), 0);

    //         module.set_rebase_delta(0x1000);
    //         assert!(module.is_relocated());
    //         assert_eq!(module.rebase_delta(), 0x1000);
    //     }

    //     #[test]
    //     fn can_add_new_section() {
    //         let ir = IR::new();
    //         let module = Module::new("dummy");
    //         let module = ir.add_module(module);
    //         assert_eq!(module.ir(), ir);
    //     }

    //     #[test]
    //     fn can_remove_section() {
    //         let ir = IR::new();
    //         let module = ir.add_module(Module::new("foo"));
    //         let section = module.add_section(Section::new("bar"));
    //         module.remove_section(section);
    //         assert_eq!(module.sections().count(), 0);
    //     }

    //     #[test]
    //     fn can_iterate_over_code_blocks() {
    //         let ir = IR::new();
    //         let module = ir.add_module(Module::new("dummy"));
    //         let section = module.add_section(Section::new(".dummy"));
    //         let b1 = section.add_byte_interval(ByteInterval::new());
    //         let b2 = section.add_byte_interval(ByteInterval::new());
    //         let cb1 = b1.add_code_block(CodeBlock::new());
    //         let cb2 = b2.add_code_block(CodeBlock::new());
    //         assert_eq!(
    //             module
    //                 .code_blocks()
    //                 .map(|cb| cb.uuid())
    //                 .collect::<Vec<Uuid>>(),
    //             vec![cb1.uuid(), cb2.uuid()]
    //         );
    //         assert_eq!(
    //             section
    //                 .code_blocks()
    //                 .map(|cb| cb.uuid())
    //                 .collect::<Vec<Uuid>>(),
    //             vec![cb1.uuid(), cb2.uuid()]
    //         );
    //     }

    //     #[test]
    //     fn can_calculate_size() {
    //         let ir = IR::new();
    //         let module = ir.add_module(Module::new("dummy"));
    //         assert_eq!(module.size(), None);
    //         assert_eq!(module.address(), None);

    //         let text = module.add_section(Section::new(".text"));
    //         let bytes = text.add_byte_interval(ByteInterval::new());
    //         bytes.set_address(Some(Addr(200)));
    //         bytes.set_size(100);

    //         assert!(module.address().is_some());
    //         assert_eq!(module.size(), Some(100));
    //         assert_eq!(module.address(), Some(Addr(200)));

    //         bytes.set_address(Some(Addr(0)));
    //         assert_eq!(module.address(), Some(Addr(0)));

    //         let data = module.add_section(Section::new(".data"));
    //         let bytes = data.add_byte_interval(ByteInterval::new());
    //         bytes.set_address(Some(Addr(300)));
    //         bytes.set_size(100);
    //         assert_eq!(module.size(), Some(400));
    //         assert_eq!(module.address(), Some(Addr(0)));

    //         assert_eq!(module.byte_intervals().count(), 2);
    //     }
}
