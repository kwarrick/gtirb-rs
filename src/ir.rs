use std::convert::TryFrom;
use std::convert::TryInto;
use std::path::Path;

use anyhow::Result;
use prost::Message;
use uuid::Uuid;

use crate::{Module};
use crate::proto;
use crate::util::parse_uuid;

#[derive(Debug)]
pub struct IR {
    pub uuid: Uuid,
    pub modules: Vec<Module>,
    //TODO: aux_data
    pub version: u32,
    //TODO: cfg
}

impl IR {
    pub fn new() -> Self {
        IR {
            uuid: Uuid::new_v4(),
            modules: Vec::new(),
            version: 0,
        }
    }

    pub fn load_protobuf<P: AsRef<Path>>(path: P) -> Result<Self> {
        let bytes = std::fs::read(path)?;
        Ok(proto::Ir::decode(&*bytes)?.try_into()?)
    }

    // modules_on
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

impl TryFrom<proto::Ir> for IR {
    type Error = anyhow::Error;
    fn try_from(message: proto::Ir) -> Result<Self> {
        let modules: Result<Vec<Module>> =
            message.modules.into_iter().map(|m| m.try_into()).collect();
        Ok(IR {
            uuid: parse_uuid(&message.uuid)?,
            modules: modules?,
            version: message.version,
        })
    }
}

pub fn read<P: AsRef<Path>>(path: P) -> Result<IR> {
    IR::load_protobuf(path)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
