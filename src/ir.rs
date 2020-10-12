use std::cell::{Ref, RefCell, RefMut};
use std::marker::PhantomData;
use std::path::Path;
use std::rc::Rc;

use anyhow::Result;
use indextree::Arena;
use prost::Message;
use uuid::Uuid;

use crate::{proto, Gtirb, Module, Node};
use crate::util::parse_uuid;

#[derive(Debug, Clone)]
pub struct IR {
    uuid: Uuid,
    version: u32,
}

impl IR {
    pub fn new() -> Node<IR> {
        let ir = IR {
            uuid: Uuid::new_v4(),
            version: 0,
        };

        let mut arena = Arena::new();
        let node_id = arena.new_node(Gtirb::IR(ir));

        Node {
            id: node_id,
            arena: Rc::new(RefCell::new(Box::new(arena))),
            data: PhantomData,
        }
    }

    pub(crate) fn load_protobuf(message: proto::Ir) -> Result<Node<IR>> {

        let ir = IR {
            uuid: parse_uuid(&message.uuid)?,
            version: message.version,
        };

        let mut arena = Arena::new();
        let ir_node_id = arena.new_node(Gtirb::IR(ir));

        for module in message.modules.into_iter() {
            let module_node_id = Module::load_protobuf(&mut arena, module)?;
            ir_node_id.append(module_node_id, &mut arena);
        }

        Ok(Node {
            id: ir_node_id,
            arena: Rc::new(RefCell::new(Box::new(arena))),
            data: PhantomData,
        })
    }
}

impl Node<IR> {
    fn get(&self) -> Ref<IR> {
        Ref::map(self.arena.borrow(), |a| {
            if let Gtirb::IR(ir) = a.get(self.id).expect("IR node").get() {
                ir
            } else {
                panic!("Expected GTIRB::IR node")
            }
        })
    }

    fn get_mut(&self) -> RefMut<IR> {
        RefMut::map(self.arena.borrow_mut(), |a| {
            if let Gtirb::IR(ir) =
                a.get_mut(self.id).expect("IR node").get_mut()
            {
                ir
            } else {
                panic!("Expected GTIRB::IR node")
            }
        })
    }

    pub fn uuid(&self) -> Uuid {
        self.get().uuid
    }

    pub fn version(&self) -> u32 {
        self.get().version
    }

    pub fn set_version(&self, n: u32) {
        self.get_mut().version = n
    }

    pub fn add_module(&self, module: Module) -> Node<Module> {
        let mut arena = self.arena.borrow_mut();
        let module_node_id = arena.new_node(Gtirb::Module(module));
        self.id.append(module_node_id, &mut arena);
        Node {
            id: module_node_id,
            arena: self.arena.clone(),
            data: PhantomData,
        }
    }

    // fn modules_on(&self) -> FilterOn {}
    // modules_at
    // sections_on
    // sections_at
    // byte_intervals_on
    // byte_intervals_at
    // code_blocks_on
    // code_blocks_at
    // data_blocks_on
    // data_blocks_at
    // symbolic_expressions_at

}

pub fn read<P: AsRef<Path>>(path: P) -> Result<Node<IR>> {
    let bytes = std::fs::read(path)?;
    IR::load_protobuf(proto::Ir::decode(&*bytes)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_ir_version() {
        let ir = IR::new();
        assert_eq!(ir.version(), 0);
        ir.set_version(7);
        assert_eq!(ir.version(), 7);
    }

    // #[test]
    // fn add_modules() {
    //    let m1 = Module::new(...);
    // }
}
