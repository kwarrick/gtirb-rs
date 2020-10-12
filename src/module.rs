use std::cell::{Ref, RefCell, RefMut};

use anyhow::{anyhow, Result};
use indextree::{Arena, NodeId};
use uuid::Uuid;

use crate::proto;
use crate::{
    Node,
    FileFormat,
    Gtirb,
    ISA,
    // ProxyBlock,
    // Section,
    // Symbol,
};
use crate::util::parse_uuid;

#[derive(Clone, Debug)]
pub struct Module {
    uuid: Uuid,
    binary_path: String,
    preferred_addr: u64,
    rebase_delta: i64,
    file_format: FileFormat,
    isa: ISA,
    name: String,
    //TODO: aux_data
    entry_point: Uuid,
}

impl Module {
    pub(crate) fn load_protobuf(arena: &mut Arena<Gtirb>, message: proto::Module) -> Result<NodeId> {
        let format = FileFormat::from_i32(message.file_format)
            .ok_or(anyhow!("Invalid FileFormat"))?;
        let isa = ISA::from_i32(message.isa).ok_or(anyhow!("Invalid ISA"))?;

        // TODO:
        // let symbols: Result<Vec<Symbol>> =
        //     message.symbols.into_iter().map(|m| m.try_into()).collect();
        // let proxies: Result<Vec<ProxyBlock>> =
        //     message.proxies.into_iter().map(|m| m.try_into()).collect();
        // let sections: Result<Vec<Section>> =
        //     message.sections.into_iter().map(|m| m.try_into()).collect();

        let module = Module {
            uuid: parse_uuid(&message.uuid)?,
            binary_path: message.binary_path,
            preferred_addr: message.preferred_addr,
            rebase_delta: message.rebase_delta,
            file_format: format,
            isa: isa,
            name: message.name,
            entry_point: parse_uuid(&message.entry_point)?,
        };

        let module_node_id = arena.new_node(Gtirb::Module(module));

        Ok(module_node_id)
    }
}


impl Node<Module> {
    fn get(&self) -> Ref<Module> {
        Ref::map(self.arena.borrow(), |a| {
            if let Gtirb::Module(module) = a.get(self.id).expect("Module node").get() {
                module
            } else {
                panic!("Expected Gtirb::Module node")
            }
        })
    }

    fn get_mut(&self) -> RefMut<Module> {
        RefMut::map(self.arena.borrow_mut(), |a| {
            if let Gtirb::Module(module) =
                a.get_mut(self.id).expect("Module node").get_mut()
            {
                module
            } else {
                panic!("Expected Gtirb::Module node")
            }
        })
    }

    pub fn uuid(&self) -> Uuid {
        self.get().uuid
    }
}
