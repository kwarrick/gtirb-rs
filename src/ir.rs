use crate::*;

#[derive(Debug, PartialEq)]
pub struct IR {
    uuid: Uuid,
    version: u32,
    modules: Vec<NodeBox<Module>>,
}

impl IR {
    pub fn new(context: &mut Context) -> Node<IR> {
        let ir = IR {
            uuid: Uuid::new_v4(),
            version: 1,
            modules: Vec::new(),
        };
        context.add_node(ir)
    }

    pub fn load_protobuf(
        context: &mut Context,
        message: proto::Ir,
    ) -> Result<Node<IR>> {
        // Load IR protobuf message.
        let ir = IR {
            uuid: crate::util::parse_uuid(&message.uuid)?,
            version: message.version,
            modules: Vec::new(),
        };

        let mut ir = context.add_node(ir);

        // Load Module protobuf messages.
        for m in message.modules.into_iter() {
            let module = Module::load_protobuf(context, m)?;
            ir.add_module(module);
        }

        Ok(ir)
    }
}

impl Node<IR> {
    pub fn version(&self) -> u32 {
        self.borrow().version
    }

    pub fn set_version(&mut self, version: u32) {
        self.borrow_mut().version = version;
    }

    pub fn add_module(&mut self, module: Node<Module>) -> Node<Module> {
        let ptr = Weak::into_raw(Rc::downgrade(&Rc::clone(&self.inner)));
        module.inner.borrow_mut().parent = Some(ptr);
        self.borrow_mut().modules.push(Rc::clone(&module.inner));
        module
    }

    pub fn remove_module(&mut self, uuid: Uuid) -> Option<Node<Module>> {
        let mut ir = self.inner.borrow_mut();
        if let Some(pos) = ir.modules.iter().position(|m| m.borrow().uuid() == uuid)
        {
            let ptr = ir.modules.remove(pos);
            ptr.borrow_mut().parent = None;
            Some(Node::new(&self.context, ptr))
        } else {
            None
        }
    }

    pub fn modules<'a>(&'a self) -> Iter<Module> {
        Iter {
            inner: Some(Ref::map(self.borrow(), |ir| &ir.modules[..])),
            context: &self.context,
        }
    }
}

impl Unique for IR {
    fn uuid(&self) -> Uuid {
        self.uuid
    }

    fn set_uuid(&mut self, uuid: Uuid) {
        self.uuid = uuid;
    }
}

impl Index for IR {
    fn insert(context: &mut Context, node: Self) -> NodeBox<Self> {
        let uuid = node.uuid();
        let boxed = Rc::new(RefCell::new(node));
        context
            .index
            .borrow_mut()
            .ir
            .insert(uuid, Rc::clone(&boxed));
        boxed
    }

    fn remove(context: &mut Context, ptr: NodeBox<Self>) -> NodeBox<Self> {
        // Remove self.
        let uuid = ptr.borrow().uuid();
        context.index.borrow_mut().ir.remove(&uuid);
        // Remove children.
        // TODO:
        // for ptr in ir.modules.values_mut() {
        //     Module::remove(context, *ptr);
        // }
        ptr
    }

    fn search(context: &Context, uuid: &Uuid) -> Option<NodeBox<Self>> {
        context
            .index
            .borrow()
            .ir
            .get(uuid)
            .map(|ptr| Rc::clone(&ptr))
    }

    fn rooted(_: NodeBox<Self>) -> bool {
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_new_ir() {
        let mut ctx = Context::new();
        let ir = IR::new(&mut ctx);
        assert_eq!(ir.version(), 1);
        assert_eq!(ir.modules().count(), 0);
    }

    #[test]
    fn new_ir_is_unique() {
        let mut ctx = Context::new();
        assert_ne!(IR::new(&mut ctx), IR::new(&mut ctx));
    }

    #[test]
    fn can_set_version() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        ir.set_version(42);
        assert_eq!(ir.version(), 42);
    }

    #[test]
    fn can_add_new_module() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        let module = Module::new(&mut ctx, "dummy");
        ir.add_module(module);

        let module = ir.modules().nth(0);
        assert!(module.is_some());
        assert_eq!(module.unwrap().ir().unwrap().uuid(), ir.uuid());
    }

    #[test]
    fn can_remove_module() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);

        let module = Module::new(&mut ctx, "dummy");
        let uuid = module.uuid();
        ir.add_module(module);

        {
            let _module = ir.remove_module(uuid);
            assert_eq!(ir.modules().count(), 0);
        }
        // Module should be dropped after preceding scope.
        let node = ctx.find_node::<Module>(&uuid);
        assert!(node.is_none());
    }

    #[test]
    fn can_modify_modules() {
        let mut ctx = Context::new();
        let mut ir = IR::new(&mut ctx);
        ir.add_module(Module::new(&mut ctx, "foo"));
        ir.add_module(Module::new(&mut ctx, "bar"));
        for mut module in ir.modules() {
            module.set_preferred_address(Addr(1));
        }
        assert!(ir.modules().all(|m| m.preferred_address() == 1.into()));
    }
}
