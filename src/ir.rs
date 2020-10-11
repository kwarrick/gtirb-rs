use std::convert::TryFrom;
use std::convert::TryInto;
use std::path::Path;
use std::collections::HashMap;
use std::sync::{Arc,RwLock};
use std::marker::PhantomData;

use anyhow::Result;
use prost::Message;
use uuid::Uuid;
use indextree::Arena;

use crate::{proto,Module,Context,GTIRB,Node};
// use crate::util::parse_uuid;

#[derive(Debug,Clone)]
pub struct IR {
    uuid: Uuid,
    version: u32,
}

impl IR {
    pub fn new<'a>() -> Node<Self> {
        // Create IR.
        let uuid = Uuid::new_v4();
        let ir = IR { uuid, version: 0 };

        // Create GTIRB arena.
        let mut arena = Arena::new();
        let node = arena.new_node(GTIRB::IR(ir.clone()));

        // Create Node table and insert IR.
        let mut index = HashMap::new();
        index.insert(uuid, node);

        let ctx = Arc::new(RwLock::new(Context { index, arena }));

        Node { node, ctx, data: PhantomData }
    }

    fn version(&self) -> u32 {
        self.version
    }

    fn uuid(&self) -> Uuid {
        self.uuid
    }
}

impl Node<IR> {

    pub fn version(&self) -> u32 {
        match self.ctx.read().unwrap().arena.get(self.node).unwrap().get()
        {
            GTIRB::IR(ir) => ir.version(),
            _ => unreachable!(),
        }
    }

    pub fn add_module(&self, module: Module) -> Node<Module> {
        let node = self.ctx.write().unwrap().append_node(self.node, module.clone());
        Node { node, ctx: self.ctx.clone(), data: PhantomData}
    }

    // pub fn load_protobuf<P: AsRef<Path>>(path: P) -> Result<Self> {
    //     let bytes = std::fs::read(path)?;
    //     Ok(proto::Ir::decode(&*bytes)?.try_into()?)
    // }

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

// impl TryFrom<proto::Ir> for IR {
//     type Error = anyhow::Error;
//     fn try_from(message: proto::Ir) -> Result<Self> {
//         let modules: Result<Vec<Module>> =
//             message.modules.into_iter().map(|m| m.try_into()).collect();
//         Ok(IR {
//             uuid: parse_uuid(&message.uuid)?,
//             modules: modules?,
//             version: message.version,
//         })
//     }
// }

// pub fn read<P: AsRef<Path>>(path: P) -> Result<IR> {
//     IR::load_protobuf(path)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_ir() {
        let ir = IR::new();
        assert_eq!(ir.version(), 0);
    }
}
