// use std::convert::TryFrom;
// use std::convert::TryInto;

// use anyhow::{anyhow, Result};
use uuid::Uuid;

use crate::proto;
use crate::{
    FileFormat,
    Gtirb,
    ISA,
    // ProxyBlock,
    // Section,
    // Symbol,
};
// use crate::util::parse_uuid;

#[derive(Clone, Debug)]
pub struct Module {
    pub uuid: Uuid,
    pub binary_path: String,
    pub preferred_addr: u64,
    pub rebase_delta: i64,
    pub file_format: FileFormat,
    pub isa: ISA,
    pub name: String,
    //TODO: aux_data
}

impl Module {
    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn binary_path(&self) -> &str {
        &self.binary_path
    }

    // ir
    // address
    // size
    //
}

// impl TryFrom<proto::Module> for Module {
//     type Error = anyhow::Error;
//     fn try_from(message: proto::Module) -> Result<Self> {
//         let format = FileFormat::from_i32(message.file_format)
//             .ok_or(anyhow!("Invalid FileFormat"))?;
//         let isa = ISA::from_i32(message.isa).ok_or(anyhow!("Invalid ISA"))?;
//         let symbols: Result<Vec<Symbol>> =
//             message.symbols.into_iter().map(|m| m.try_into()).collect();
//         let proxies: Result<Vec<ProxyBlock>> =
//             message.proxies.into_iter().map(|m| m.try_into()).collect();
//         let sections: Result<Vec<Section>> =
//             message.sections.into_iter().map(|m| m.try_into()).collect();
//         Ok(Module {
//             uuid: parse_uuid(&message.uuid)?,
//             binary_path: message.binary_path,
//             preferred_addr: message.preferred_addr,
//             rebase_delta: message.rebase_delta,
//             file_format: format,
//             isa: isa,
//             name: message.name,
//             symbols: symbols?,
//             proxies: proxies?,
//             sections: sections?,
//             entry_point: parse_uuid(&message.entry_point)?,
//         })
//     }
// }
