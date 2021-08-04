use std::path::Path;

use anyhow::Result;
use prost::Message;

use crate::*;

#[derive(Debug, PartialEq)]
pub struct IR {
    uuid: Uuid,
    version: u32,
    modules: Vec<Node<Module>>,
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
        Box::pin(IR {
            uuid: Uuid::new_v4(),
            modules: Vec::with_capacity(1),
            version: 1,
        })
    }

    pub fn read<P: AsRef<Path>>(path: P) -> Result<Node<IR>> {
        let bytes = std::fs::read(path)?;
        IR::load_protobuf(proto::Ir::decode(&*bytes)?)
    }

    fn load_protobuf(message: proto::Ir) -> Result<Node<IR>> {
        // Load IR protobuf message.
        let mut ir = IR {
            uuid: crate::util::parse_uuid(&message.uuid)?,
            version: message.version,
            modules: Vec::with_capacity(message.modules.len()),
        };

        // Load Module protobuf messages.
        for m in message.modules.into_iter() {
            ir.add_module(Module::load_protobuf(m)?);
        }

        Ok(Box::pin(ir))
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn set_version(&mut self, version: u32) {
        self.version = version
    }

    pub fn modules(&self) -> Iter<Module> {
        self.modules.iter()
    }

    pub fn modules_mut(&mut self) -> IterMut<Module> {
        self.modules.iter_mut()
    }

    pub fn add_module(&mut self, mut module: Module) {
        module.parent = self;
        self.modules.push(Box::pin(module));
    }

    pub fn remove_module(&mut self, uuid: Uuid) {
        if let Some(pos) = self.modules.iter().position(|m| m.uuid() == uuid) {
            self.modules.remove(pos);
        }
    }

    pub fn find_node<T: Unique>(&self, _uuid: Uuid) -> Option<Node<T>> {
        unimplemented!();
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
    fn is_pinned() {
        let mut ir = IR::new();
        ir.add_module(Module::new("A"));
    }

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
        assert_eq!(module.unwrap().ir(), Some(&*ir));
    }

    #[test]
    fn can_remove_module() {
        let mut ir = IR::new();
        let module = Module::new("dummy");
        ir.add_module(module);

        let uuid = ir.modules().last().unwrap().uuid();
        ir.remove_module(uuid);
        assert_eq!(ir.modules().count(), 0);

        // TODO:
        // let node: Option<Node<Module>> = ir.find_node(uuid);
        // assert!(node.is_none());
    }

    // TODO:
    // #[test]
    // fn can_find_node_by_uuid() {
    //     let mut ir = IR::new();
    //     let module = Module::new("dummy");
    //     let uuid = module.uuid();
    //     ir.add_module(module);
    //     let node: Option<Node<Module>> = ir.find_node(uuid);
    //     assert!(node.is_some());
    //     assert_eq!(uuid, node.unwrap().uuid());
    // }

    #[test]
    fn can_modify_modules() {
        let mut ir = IR::new();
        ir.add_module(Module::new("foo"));
        ir.add_module(Module::new("bar"));
        for module in ir.modules_mut() {
            module.set_preferred_address(Addr(1));
        }
        assert!(ir.modules().all(|m| m.preferred_address() == 1.into()));
    }
}
