use crate::*;
use std::collections::HashMap;

#[derive(Debug, Default, PartialEq)]
pub struct IR {
    uuid: Uuid,
    version: u32,
    modules: HashMap<Uuid, Node<Module>>,
}

impl IR {
    pub(crate) fn new() -> Node<IR> {
        let node = Box::new(IR {
            uuid: Uuid::new_v4(),
            version: 1,
            ..Default::default()
        });
        Node {
            ptr: Box::into_raw(node),
            kind: PhantomData,
        }
    }

    pub fn load_protobuf(message: proto::Ir) -> Result<Node<IR>> {
        // Load IR protobuf message.
        let mut ir = Box::new(IR {
            uuid: crate::util::parse_uuid(&message.uuid)?,
            version: message.version,
            ..Default::default()
        });

        // Load Module protobuf messages.
        for m in message.modules.into_iter() {
            ir.add_module(Module::load_protobuf(m)?);
        }

        Ok(Node {
            ptr: Box::into_raw(ir),
            kind: PhantomData,
        })
    }

    pub fn read<P: AsRef<Path>>(path: P) -> Result<Node<IR>> {
        let bytes = std::fs::read(path)?;
        IR::load_protobuf(proto::Ir::decode(&*bytes)?)
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

    pub fn add_module(&mut self, mut module: Node<Module>) {
        module.parent = Some(Node {
            ptr: self,
            kind: PhantomData,
        });
        self.modules.insert(module.uuid(), module);
    }

    pub fn remove_module(&mut self, uuid: Uuid) -> Option<Node<Module>> {
        if let Some(mut module) = self.modules.remove(&uuid) {
            module.parent.take();
            Some(module)
        } else {
            None
        }
        // TODO: Dangling pointer
    }

    pub fn find_node(&self, uuid: &Uuid) -> Option<&Node<Module>> {
        self.modules.get(uuid)
    }

    pub fn find_node_mut(&mut self, uuid: &Uuid) -> Option<&mut Node<Module>> {
        self.modules.get_mut(uuid)
    }
}

pub fn read<P: AsRef<Path>>(path: P) -> Result<Node<IR>> {
    let bytes = std::fs::read(path)?;
    IR::load_protobuf(proto::Ir::decode(&*bytes)?)
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
        let mut ir = IR::new();
        ir.set_version(42);
        assert_eq!(ir.version(), 42);
    }

    #[test]
    fn can_add_new_module() {
        let mut ir = IR::new();
        let module = Module::new("dummy");
        ir.add_module(module);

        let module = ir.modules().nth(0);
        assert!(module.is_some());
        // TODO: Try removing .uuid()
        assert_eq!(module.unwrap().ir().unwrap().uuid(), ir.uuid());
    }

    #[test]
    fn can_remove_module() {
        let mut ir = IR::new();

        let module = Module::new("dummy");
        let uuid = module.uuid();
        ir.add_module(module);

        let module = ir.modules().last().unwrap();

        ir.remove_module(uuid);
        assert_eq!(ir.modules().count(), 0);

        // let node = ir.find_node::<Node<Module>>(uuid);
        // assert!(node.is_none());
    }

    // #[test]
    // fn can_find_node_by_uuid() {
    //     let mut ir = IR::new();
    //     let module = Module::new("foo");
    //     let uuid = module.uuid();
    //     ir.add_module(module);

    //     let node: Option<Node<Module>> = ir.find_node(uuid);
    //     assert!(node.is_some());
    //     assert_eq!(uuid, node.unwrap().uuid());

    //     let mut node: Node<Module> = ir.find_node_mut(uuid).unwrap();
    //     node.set_name("bar");

    //     let module = ir.modules().last().unwrap();
    //     assert_eq!(module.name(), "bar");

    //     node.set_name("baz");
    // }

    #[test]
    fn can_modify_modules() {
        let mut ir = IR::new();
        ir.add_module(Module::new("foo"));
        ir.add_module(Module::new("bar"));
        for mut module in ir.modules_mut() {
            module.set_preferred_address(Addr(1));
        }
        assert!(ir.modules().all(|m| m.preferred_address() == 1.into()));
    }
}
