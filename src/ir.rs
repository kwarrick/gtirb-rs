use crate::*;

#[derive(Debug)]
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
            context: Rc::new(RefCell::new(context)),
            kind: PhantomData,
        }
    }
}

impl Node<IR> {

    fn borrow(&self) -> Ref<IR> {
        Ref::map(self.context.borrow(), |ctx| {
            &ctx.ir[self.index]
        })
    }

    fn borrow_mut(&self) -> RefMut<IR> {
        RefMut::map(self.context.borrow_mut(), |ctx| {
            &mut ctx.ir[self.index]
        })
    }

    pub fn uuid(&self) -> Uuid {
        self.borrow().uuid
    }

    pub fn set_uuid(&self, uuid: Uuid) {
        let mut context = self.context.borrow_mut();
        let mut ir = self.borrow_mut();
        context.uuid_map.remove(&ir.uuid);
        context.uuid_map.insert(uuid, self.index);
        ir.uuid = uuid;
    }

    pub fn version(&self) -> u32 {
        self.borrow().version
    }

    pub fn set_version(&self, version: u32) {
        self.borrow_mut().version = version
    }

    pub fn modules(&self) -> NodeIterator<IR, Module> {
        NodeIterator {
            index: 0,
            parent: Node {
                index: self.index,
                context: self.context.clone(),
                kind: PhantomData
            },
            kind: PhantomData,
        }
     }

    pub fn add_module(&self, module: Module) {
        let index = self
            .context
            .borrow_mut()
            .add_module(module);
        self.borrow_mut().modules.push(index);
    }

}

impl Container<Module> for Node<IR> {
    fn get(&self, index: usize) -> (Option<Index>, PhantomData<Module>) {
        let module_index = self.context
            .borrow()
            .ir[self.index]
            .modules
            .get(index)
            .cloned();
        (module_index, PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use super::IR;

    #[test]
    fn can_create_new_ir() {
        let ir = IR::new();
        assert_eq!(ir.version(), 1);
        assert_eq!(ir.modules().count(), 0);
    }

    #[test]
    fn can_set_version() {
        let ir = IR::new();
        ir.set_version(42);
        assert_eq!(ir.version(), 42);
    }
}
