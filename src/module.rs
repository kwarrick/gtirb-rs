use crate::*;

#[derive(Debug, Default, PartialEq)]
pub(crate) struct Module {
    pub(crate) parent: Option<Index>,

    uuid: Uuid,
    name: String,
    binary_path: String,
    entry_point: Option<Index>,
    byte_order: ByteOrder,
    isa: ISA,
    rebase_delta: i64,
    preferred_address: u64,
    file_format: FileFormat,
    sections: Vec<Index>,
    symbols: Vec<Index>,
    proxies: Vec<Index>,
}

impl Module {
    pub fn new(name: &str) -> Self {
        Module {
            uuid: Uuid::new_v4(),
            name: name.to_owned(),
            ..Default::default()
        }
    }
}

impl Unique for Module {
    fn uuid(&self) -> Uuid {
        self.uuid
    }

    fn set_uuid(&mut self, uuid: Uuid) {
        self.uuid = uuid;
    }
}

impl Node<Module> {
    pub fn ir(&self) -> Node<IR> {
        Node {
            index: self.borrow().parent.expect("parent node"),
            context: self.context.clone(),
            kind: PhantomData,
        }
    }

    pub fn set_ir(&self, ir: Node<IR>) {
        self.borrow_mut().parent.replace(ir.index);
    }

    pub fn name(&self) -> String {
        self.borrow().name.to_owned()
    }

    pub fn set_name<T: AsRef<str>>(&self, name: T) {
        self.borrow_mut().name = name.as_ref().to_owned();
    }

    pub fn binary_path(&self) -> String {
        self.borrow().binary_path.to_owned()
    }

    pub fn set_binary_path<T: AsRef<str>>(&self, binary_path: T) {
        self.borrow_mut().binary_path = binary_path.as_ref().to_owned();
    }

    pub fn file_format(&self) -> FileFormat {
        self.borrow().file_format
    }

    pub fn set_file_format(&mut self, file_format: FileFormat) {
        self.borrow_mut().file_format = file_format;
    }

    pub fn isa(&self) -> ISA {
        self.borrow().isa
    }

    pub fn set_isa(&mut self, isa: ISA) {
        self.borrow_mut().isa = isa;
    }

    pub fn entry_point(&self) -> Option<Node<CodeBlock>> {
        self.borrow().entry_point.map(|index| Node {
            index,
            context: self.context.clone(),
            kind: PhantomData,
        })
    }

    pub fn set_entry_point(&mut self, block: Node<CodeBlock>) {
        self.borrow_mut().entry_point.replace(block.index);
    }

    pub fn byte_order(&self) -> ByteOrder {
        self.borrow().byte_order
    }

    pub fn set_byte_order(&mut self, byte_order: ByteOrder) {
        self.borrow_mut().byte_order = byte_order;
    }

    pub fn preferred_address(&self) -> u64 {
        self.borrow().preferred_address
    }

    pub fn set_preferred_address(&mut self, preferred_address: u64) {
        self.borrow_mut().preferred_address = preferred_address;
    }

    pub fn rebase_delta(&self) -> i64 {
        self.borrow().rebase_delta
    }

    pub fn set_rebase_delta(&mut self, rebase_delta: i64) {
        self.borrow_mut().rebase_delta = rebase_delta;
    }

    pub fn is_relocated(&self) -> bool {
        self.borrow().rebase_delta != 0
    }

    pub fn sections(&self) -> NodeIterator<IR, Section> {
        NodeIterator {
            index: 0,
            parent: Node {
                index: self.index,
                context: self.context.clone(),
                kind: PhantomData,
            },
            kind: PhantomData,
        }
    }

    pub fn add_section(&self, section: Section) {
        let uuid = section.uuid();

        let mut section = section;
        section.parent.replace(self.index);

        let index = {
            let mut context = self.context.borrow_mut();
            let index = context.section.insert(section);
            context.uuid_map.insert(uuid, index);
            index
        };

        self.borrow_mut().sections.push(index);
    }

    pub fn remove_section(&self, node: Node<Section>) {
        let (index, uuid) = { (node.index, node.uuid()) };
        // Remove Section from Module.
        self.remove((node.index, PhantomData));
        // Remove Section from Context.
        {
            let mut context = self.context.borrow_mut();
            context.uuid_map.remove(&uuid);
            context.section.remove(index);
        }
    }

    // TODO:
    // size
    // address

    // code_blocks()
    // data_blocks()
    // byte_intervals()
    // symbolic_expressions()

    // proxy_blocks()
    // add_proxy_block
    // remove_proxy_block

    // symbols()
    // add_symbol()
    // remove_symbol()

    // get_symbol_reference<T>(symbol: Symbol) -> Node<T>
}

impl Indexed<Module> for Node<Module> {
    fn get_ref(
        &self,
        (index, _): (Index, PhantomData<Module>),
    ) -> Option<Ref<Module>> {
        let context = self.context.borrow();
        if context.module.contains(index) {
            Some(Ref::map(context, |ctx| &ctx.module[index]))
        } else {
            None
        }
    }

    fn get_ref_mut(
        &self,
        (index, _): (Index, PhantomData<Module>),
    ) -> Option<RefMut<Module>> {
        let context = self.context.borrow_mut();
        if context.module.contains(index) {
            Some(RefMut::map(context, |ctx| &mut ctx.module[index]))
        } else {
            None
        }
    }
}

impl Container<Section> for Node<Module> {
    fn get(&self, position: usize) -> (Option<Index>, PhantomData<Section>) {
        let index = self
            .get_ref((self.index, PhantomData))
            .unwrap()
            .sections
            .get(position)
            .cloned();
        (index, PhantomData)
    }

    fn remove(&self, (index, _): (Index, PhantomData<Section>)) {
        let mut ir = self.get_ref_mut((self.index, PhantomData)).unwrap();
        if let Some(position) = ir.sections.iter().position(|i| *i == index) {
            ir.sections.remove(position);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_module_is_unique() {
        assert_ne!(Module::new("a"), Module::new("a"));
    }

    #[test]
    fn can_add_new_section() {
        let ir = IR::new();
        let module = Module::new("dummy");
        let module = ir.add_module(module);
        assert_eq!(module.ir(), ir);
    }
}
