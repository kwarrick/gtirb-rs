use std::collections::HashMap;

use crate::*;

#[derive(Debug, PartialEq)]
pub struct IR {
    uuid: Uuid,
    version: u32,
    modules: HashMap<Uuid, Node<Module>>,
}

impl IR {
    pub fn new(context: &mut Context) -> Node<IR> {
        let uuid = Uuid::new_v4();
        let ir = Box::new(IR {
            uuid: uuid.clone(),
            version: 1,
            modules: HashMap::new(),
        });
        let ir = context.add_ir(uuid, Box::into_raw(ir));
        Node::new(context, ir)
    }

    pub fn load_protobuf(
        context: &mut Context,
        message: proto::Ir,
    ) -> Result<Node<IR>> {
        // Load IR protobuf message.
        let mut ir = Box::new(IR {
            uuid: crate::util::parse_uuid(&message.uuid)?,
            version: message.version,
            modules: HashMap::new(),
        });

        // Load Module protobuf messages.
        for m in message.modules.into_iter() {
            ir.add_module(Module::load_protobuf(context, m)?);
        }

        let ir = Box::into_raw(ir);
        Ok(Node::new(context, ir))
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn set_uuid(&mut self, uuid: Uuid) {
        self.uuid = uuid;
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn set_version(&mut self, version: u32) {
        self.version = version;
    }

    pub fn modules(&self) -> impl Iterator<Item = &Node<Module>> {
        self.modules.values()
    }

    pub fn modules_mut(&mut self) -> impl Iterator<Item = &mut Node<Module>> {
        self.modules.values_mut()
    }

    pub fn add_module(
        &mut self,
        mut module: Node<Module>,
    ) -> Option<Node<Module>> {
        module.parent = Some(Node {
            ptr: self,
            ctx: std::ptr::null_mut(),
            kind: PhantomData,
        });
        self.modules.insert(module.uuid(), module)
    }

    pub fn remove_module(&mut self, uuid: Uuid) -> Option<Node<Module>> {
        if let Some(mut module) = self.modules.remove(&uuid) {
            module.parent = None;
            Some(module)
        } else {
            None
        }
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

        ir.remove_module(uuid);
        assert_eq!(ir.modules().count(), 0);

        // TODO:
        // let node = ir.find_node::<Node<Module>>(uuid);
        // assert!(node.is_none());
    }

    // #[test]
    // fn can_find_node_by_uuid() {
    //     let mut ir = IR::new();
    //     let module = Module::new("foo");
    //     let uuid = module.uuid();
    //     ir.add_module(module);

    //     let node: Option<Node<Module>> = ir.find_node(&uuid);
    //     assert!(node.is_some());
    //     assert_eq!(uuid, node.unwrap().uuid());

    //     let mut node: Node<Module> = ir.find_node_mut(&uuid).unwrap();
    //     node.set_name("bar");

    //     let module = ir.modules().last().unwrap();
    //     assert_eq!(module.name(), "bar");

    //     node.set_name("baz");
    // }

    // #[test]
    // fn can_modify_modules() {
    //     let mut ir = IR::new();
    //     ir.add_module(Module::new("foo"));
    //     ir.add_module(Module::new("bar"));
    //     for mut module in ir.modules_mut() {
    //         module.set_preferred_address(Addr(1));
    //     }
    //     assert!(ir.modules().all(|m| m.preferred_address() == 1.into()));
    // }
}
