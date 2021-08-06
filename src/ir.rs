use crate::*;
use std::collections::HashMap;

#[derive(Debug, Default, PartialEq)]
struct Index {
    modules: HashMap<Uuid, *mut Module>,
}

#[derive(Debug, Default, PartialEq)]
pub struct IR {
    uuid: Uuid,
    version: u32,
    modules: Vec<*mut Module>,
    index: Index,
}

impl IR {
    pub(crate) fn new() -> IR {
        IR {
            uuid: Uuid::new_v4(),
            version: 1,
            ..Default::default()
        }
    }

    pub fn load_protobuf(message: proto::Ir) -> Result<Box<IR>> {
        // Load IR protobuf message.
        let mut ir = Box::new(IR {
            uuid: crate::util::parse_uuid(&message.uuid)?,
            version: message.version,
            modules: Vec::with_capacity(message.modules.len()),
            ..Default::default()
        });

        // Load Module protobuf messages.
        // for m in message.modules.into_iter() {
        //     ir.add_module(Module::load_protobuf(m)?);
        // }

        Ok(ir)
    }

    pub fn read<P: AsRef<Path>>(path: P) -> Result<Box<IR>> {
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

    pub fn modules(&self) -> Iter<Module, Self> {
        Iter {
            iter: self.modules.iter(),
            lender: &self,
        }
    }

    // pub fn modules_mut(&mut self) -> IterMut<Module> {
    //     IterMut {
    //         iter: self.modules.iter_mut(),
    //         lender: &mut self,
    //     }
    // }

    // pub fn add_module(&mut self, mut module: Module) {
    //     module.parent = self;
    //     self.modules.push(module.ptr);
    //     self.index.modules.insert(module.uuid(), module.ptr);
    // }

    // pub fn remove_module(&mut self, module: NodeRef<Module>) {
    //     if let Some(pos) = self
    //         .modules
    //         .iter()
    //         .position(|ptr| *ptr as *const Module == module.ptr)
    //     {
    //         // Remove the indexes for the node.
    //         assert!(self.index.modules.remove(&module.uuid()).is_some());
    //         // Remove the raw pointer from child node list.
    //         self.modules.remove(pos);
    //         // TODO: Free the subtree!
    //     }
    // }

    // pub fn find_node<T>(&self, uuid: Uuid) -> Option<NodeRef<T>> {
    //     if let Some(ptr) = self.index.modules.get(&uuid) {
    //         Some(NodeRef::from_raw(*ptr as *const T))
    //     } else {
    //         None
    //     }
    // }

    // pub fn find_node_mut<T>(&mut self, uuid: Uuid) -> Option<NodeMut<T>> {
    //     if let Some(ptr) = self.index.modules.get(&uuid) {
    //         Some(NodeMut::from_raw(*ptr as *mut T))
    //     } else {
    //         None
    //     }
    // }
}

pub fn read<P: AsRef<Path>>(path: P) -> Result<Box<IR>> {
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
        assert_eq!(module.unwrap().ir().unwrap(), ir);
    }

    #[test]
    fn can_remove_module() {
        let mut ir = IR::new();
        let module = Module::new("dummy");
        ir.add_module(module);

        let module = ir.modules().last().unwrap();
        let uuid = module.uuid();
        ir.remove_module(module);
        assert_eq!(ir.modules().count(), 0);

        let node: Option<NodeRef<Module>> = ir.find_node(uuid);
        assert!(node.is_none());
    }

    #[test]
    fn can_find_node_by_uuid() {
        let mut ir = IR::new();
        let module = Module::new("foo");
        let uuid = module.uuid();
        ir.add_module(module);

        let node: Option<NodeRef<Module>> = ir.find_node(uuid);
        assert!(node.is_some());
        assert_eq!(uuid, node.unwrap().uuid());

        let mut node: NodeMut<Module> = ir.find_node_mut(uuid).unwrap();
        node.set_name("bar");

        let module = ir.modules_mut().last().unwrap();
        assert_eq!(module.name(), "bar");

        // XXX: THIS SHOULDN'T BE POSSIBLE
        node.set_name("baz");
    }

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
