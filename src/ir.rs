use crate::*;

pub(crate) struct IR {
    uuid: Uuid,
    modules: Vec<Index>,
    version: u32,
    // cfg
    // aux_data
}

impl IR {
    pub fn new() -> Node<IR> {
        let mut context = Context::new();
        let index = context.ir.insert(IR {
            uuid: Uuid::new_v4(),
            modules: Vec::new(),
            version: 1,
        });
        Node {
            index,
            context: Arc::new(RwLock::new(context)),
            kind: PhantomData,
        }
    }
}

impl Node<IR> {
    fn read<T, F: Fn(&IR) -> T>(&self, f: F) -> T {
        f(&self.context.read().expect("read lock").ir[self.index])
    }

    fn write<T, F: Fn(&mut IR) -> T>(&self, f: F) -> T {
        f(&mut self.context.write().expect("write lock").ir[self.index])
    }

    pub fn uuid(&self) -> Uuid {
        self.read(|ir| ir.uuid)
    }

    pub fn set_uuid(&self, uuid: Uuid) {
        self.write(|ir| ir.uuid = uuid);
    }

    pub fn version(&self) -> u32 {
        self.read(|ir| ir.version)
    }

    pub fn set_version(&self, version: u32) {
        self.write(|ir| ir.version = version)
    }

    pub fn modules(&self) -> NodeIterator<Module> {
        NodeIterator {
            index: 0,
            parent: self.index,
            context: self.context.clone(),
            kind: PhantomData,
        }
    }

    pub fn add_module(&self, module: Module) {
        let module = self
            .context
            .write()
            .expect("write lock")
            .module
            .insert(module);
        self.write(|ir| ir.modules.push(module));
    }
}

impl Iterator for NodeIterator<Module> {
    type Item = Node<Module>;

    fn next(&mut self) -> Option<Self::Item> {
        let module = self.context.read().expect("read lock").ir[self.parent]
            .modules
            .get(self.index)
            .cloned();

        self.index = self.index + 1;

        module.map(|index| Node {
            index,
            context: self.context.clone(),
            kind: PhantomData,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::IR;

    #[test]
    fn can_create_new_ir() {
        let ir = IR::new();
        assert_eq!(ir.version(), 1);
        assert_eq!(ir.modules().len(), 0);
    }

    #[test]
    fn can_set_version() {
        let ir = IR::new();
        ir.set_version(42);
        assert_eq!(ir.version(), 42);
    }
}
