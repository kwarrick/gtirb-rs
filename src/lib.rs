use std::convert::TryFrom;
use std::convert::TryInto;
use std::path::Path;

use anyhow::{anyhow, Context, Result};
use prost::Message;
use uuid::Uuid;

// Lib
// -----------------------------------------------------------------------------
mod proto {
    include!(concat!(env!("OUT_DIR"), "/proto.rs"));
}

use proto::FileFormat;
use proto::Isa as ISA;

pub fn read<P: AsRef<Path>>(path: P) -> Result<IR> {
    IR::load_protobuf(path)
}


// IR
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct IR {
    pub version: u32,
    pub uuid: Uuid,
    pub modules: Vec<Module>,
}

impl IR {
    pub fn load_protobuf<P: AsRef<Path>>(path: P) -> Result<Self> {
        let bytes = std::fs::read(path)?;
        Ok(proto::Ir::decode(&*bytes)?.try_into()?)
    }
}

impl TryFrom<proto::Ir> for IR {
    type Error = anyhow::Error;
    fn try_from(message: proto::Ir) -> Result<Self> {
        let modules: Result<Vec<Module>> = message
            .modules
            .into_iter()
            .map(|m| m.try_into())
            .collect();

        Ok(IR {
            uuid: parse_uuid(&message.uuid)?,
            version: message.version,
            modules: modules?,
        })
    }
}

// Module
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Module {
    pub uuid: Uuid,
    pub binary_path: String,
    pub preferred_addr: u64,
    pub rebase_delta: i64,
    pub file_format: FileFormat,
    pub isa: ISA,
    pub name: String,
}

impl TryFrom<proto::Module> for Module {
    type Error = anyhow::Error;
    fn try_from(message: proto::Module) -> Result<Self> {
        let format = FileFormat::from_i32(message.file_format)
            .ok_or(anyhow!("Invalid FileFormat"))?;
        let isa = ISA::from_i32(message.isa)
            .ok_or(anyhow!("Invalid ISA"))?;
        Ok(Module {
            uuid: parse_uuid(&message.uuid)?,
            binary_path: message.binary_path.clone(),
            preferred_addr: message.preferred_addr,
            rebase_delta: message.rebase_delta,
            file_format: format,
            isa: isa,
            name: message.name.clone(),
            // ...
        })
    }
}

// Util
// -----------------------------------------------------------------------------
fn parse_uuid(bytes: &[u8]) -> Result<Uuid> {
    let bytes: [u8; 16] =
        bytes.try_into().context("Failed to parse 16-byte UUID")?;
    Ok(Uuid::from_bytes(bytes))
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
