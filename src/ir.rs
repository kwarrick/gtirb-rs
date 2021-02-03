use crate::*;

#[derive(Debug, Default, PartialEq)]
pub(crate) struct IR {
    uuid: Uuid,
    modules: Vec<Index>,
    version: u32,
}

impl Unique for IR {
    fn uuid(&self) -> Uuid {
        self.uuid
    }

    fn set_uuid(&mut self, uuid: Uuid) {
        self.uuid = uuid;
    }
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
    fn find_node<U>(&self, uuid: Uuid) -> Option<Node<U>>
    where
        Node<U>: Indexed<U>,
        U: Unique,
    {
        self.context
            .borrow()
            .uuid_map
            .get(&uuid)
            .map(|index| Node {
                index: *index,
                context: self.context.clone(),
                kind: PhantomData,
            })
            .filter(|node| {
                node.get_ref((node.index, PhantomData))
                    .map(|inner| inner.uuid() == uuid)
                    .unwrap_or(false)
            })
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
                kind: PhantomData,
            },
            kind: PhantomData,
        }
    }

    pub fn add_module(&self, module: Module) {
        let uuid = module.uuid();

        let mut module = module;
        module.set_ir(self.index);

        let index = {
            let mut context = self.context.borrow_mut();
            let index = context.module.insert(module);
            context.uuid_map.insert(uuid, index);
            index
        };

        self.borrow_mut().modules.push(index);
    }

    pub fn remove_module(&self, node: Node<Module>) {
        let (index, uuid) = {
            (node.index, node.uuid())
        };
        // Remove Module from IR.
        {
            let mut ir = self.borrow_mut();
            if let Some(index) = ir.modules.iter().position(|i| *i == index) {
                ir.modules.remove(index);
            }
        }
        // Remove Module from Context.
        {
            let mut context = self.context.borrow_mut();
            context.uuid_map.remove(&uuid);
            context.module.remove(index);
        }
    }
}

impl Indexed<IR> for Node<IR> {
    fn get_ref(&self, (index, _): (Index, PhantomData<IR>)) -> Option<Ref<IR>> {
        let context = self.context.borrow();
        if context.ir.contains(index) {
            Some(Ref::map(context, |ctx| &ctx.ir[index]))
        } else {
            None
        }
    }

    fn get_ref_mut(
        &self,
        (index, _): (Index, PhantomData<IR>),
    ) -> Option<RefMut<IR>> {
        let context = self.context.borrow_mut();
        if context.ir.contains(index) {
            Some(RefMut::map(context, |ctx| &mut ctx.ir[index]))
        } else {
            None
        }
    }
}

impl Container<Module> for Node<IR> {
    fn get(&self, index: usize) -> (Option<Index>, PhantomData<Module>) {
        let module_index = self.context.borrow().ir[self.index]
            .modules
            .get(index)
            .cloned();
        (module_index, PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_new_ir() {
        let ir = IR::new();
        assert_eq!(ir.version(), 1);
        assert_eq!(ir.modules().count(), 0);
    }

    #[test]
    fn new_ir_is_unique() {
        assert_ne!(IR::new(), IR::new());
    }

    #[test]
    fn can_set_version() {
        let ir = IR::new();
        ir.set_version(42);
        assert_eq!(ir.version(), 42);
    }

    #[test]
    fn can_add_new_module() {
        let ir = IR::new();
        let module = Module::new("dummy");
        ir.add_module(module);
        let module = ir.modules().nth(0);
        assert!(module.is_some());
        assert_eq!(module.unwrap().ir(), ir);
    }

    #[test]
    fn can_remove_module() {
        let ir = IR::new();
        let module = Module::new("dummy");
        let uuid = module.uuid();
        ir.add_module(module);
        let module = ir.modules().nth(0);
        assert!(module.is_some());
        ir.remove_module(module.unwrap());
        assert_eq!(ir.modules().count(), 0);
        let node: Option<Node<Module>> = ir.find_node(uuid);
        assert!(node.is_none());
    }

    #[test]
    fn can_find_node_by_uuid() {
        let ir = IR::new();
        let module = Module::new("dummy");
        let uuid = module.uuid();
        ir.add_module(module);
        let node: Option<Node<Module>> = ir.find_node(uuid);
        assert!(node.is_some());
        assert_eq!(uuid, node.unwrap().uuid());
    }
}
