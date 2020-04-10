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
use proto::SectionFlag;

pub fn read<P: AsRef<Path>>(path: P) -> Result<IR> {
    IR::load_protobuf(path)
}

// IR
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct IR {
    pub uuid: Uuid,
    pub modules: Vec<Module>,
    //TODO: aux_data
    pub version: u32,
    //TODO: cfg
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
        let modules: Result<Vec<Module>> =
            message.modules.into_iter().map(|m| m.try_into()).collect();
        Ok(IR {
            uuid: parse_uuid(&message.uuid)?,
            modules: modules?,
            version: message.version,
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
    pub symbols: Vec<Symbol>,
    pub proxies: Vec<ProxyBlock>,
    pub sections: Vec<Section>,
    //TODO: aux_data
    pub entry_point: Uuid,
}

impl TryFrom<proto::Module> for Module {
    type Error = anyhow::Error;
    fn try_from(message: proto::Module) -> Result<Self> {
        let format = FileFormat::from_i32(message.file_format)
            .ok_or(anyhow!("Invalid FileFormat"))?;
        let isa = ISA::from_i32(message.isa).ok_or(anyhow!("Invalid ISA"))?;
        let symbols: Result<Vec<Symbol>> =
            message.symbols.into_iter().map(|m| m.try_into()).collect();
        let proxies: Result<Vec<ProxyBlock>> =
            message.proxies.into_iter().map(|m| m.try_into()).collect();
        let sections: Result<Vec<Section>> =
            message.sections.into_iter().map(|m| m.try_into()).collect();
        Ok(Module {
            uuid: parse_uuid(&message.uuid)?,
            binary_path: message.binary_path,
            preferred_addr: message.preferred_addr,
            rebase_delta: message.rebase_delta,
            file_format: format,
            isa: isa,
            name: message.name,
            symbols: symbols?,
            proxies: proxies?,
            sections: sections?,
            entry_point: parse_uuid(&message.entry_point)?,
        })
    }
}

// Section
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Section {
    pub uuid: Uuid,
    pub name: String,
    pub byte_intervals: Vec<ByteInterval>,
    pub section_flags: Vec<SectionFlag>,
}

impl TryFrom<proto::Section> for Section {
    type Error = anyhow::Error;
    fn try_from(message: proto::Section) -> Result<Self> {
        let byte_intervals: Result<Vec<ByteInterval>> = message
            .byte_intervals
            .into_iter()
            .map(|m| m.try_into())
            .collect();
        let section_flags: Result<Vec<SectionFlag>> = message
            .section_flags
            .into_iter()
            .map(|i| {
                SectionFlag::from_i32(i).ok_or(anyhow!("Invalid FileFormat"))
            })
            .collect();
        Ok(Section {
            uuid: parse_uuid(&message.uuid)?,
            name: message.name,
            byte_intervals: byte_intervals?,
            section_flags: section_flags?,
        })
    }
}

// ByteInterval
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct ByteInterval {
    pub uuid: Uuid,
    pub blocks: Vec<Block>,
    // pub symbolic_expressions: Vec<SymbolicExpression>,
    pub has_address: bool,
    pub address: u64,
    pub size: u64,
    pub contents: Vec<u8>,
}

impl TryFrom<proto::ByteInterval> for ByteInterval {
    type Error = anyhow::Error;
    fn try_from(message: proto::ByteInterval) -> Result<Self> {
        let mut blocks = Vec::new();
        for block in message.blocks.into_iter() {
            match block.value {
                Some(proto::block::Value::Code(_)) =>
                    blocks.push(Block::CodeBlock(block.try_into()?)),
                Some(proto::block::Value::Data(_)) =>
                    blocks.push(Block::DataBlock(block.try_into()?)),
                _ => unreachable!()
            }
        }
        Ok(ByteInterval {
            uuid: parse_uuid(&message.uuid)?,
            blocks: blocks,
            has_address: message.has_address,
            address: message.address,
            size: message.size,
            contents: message.contents,
        })
        //
    }
}

// Block
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct CodeBlock {
    pub offset: u64,
    pub uuid: Uuid,
    pub size: u64,
    pub decode_mode: u64,
}

impl TryFrom<proto::Block> for CodeBlock {
    type Error = anyhow::Error;
    fn try_from(message: proto::Block) -> Result<Self> {
        match message.value {
            Some(proto::block::Value::Code(block)) => Ok(CodeBlock {
                offset: message.offset,
                uuid: parse_uuid(&block.uuid)?,
                size: block.size,
                decode_mode: block.decode_mode,
            }),
            _ => Err(anyhow!("Failed to convert CodeBlock")),
        }
    }
}

#[derive(Debug)]
pub struct DataBlock {
    pub offset: u64,
    pub uuid: Uuid,
    pub size: u64,
}

impl TryFrom<proto::Block> for DataBlock {
    type Error = anyhow::Error;
    fn try_from(message: proto::Block) -> Result<Self> {
        match message.value {
            Some(proto::block::Value::Data(block)) => Ok(DataBlock {
                offset: message.offset,
                uuid: parse_uuid(&block.uuid)?,
                size: block.size,
            }),
            _ => Err(anyhow!("Failed to convert DataBlock")),
        }
    }
}

#[derive(Debug)]
pub enum Block {
    CodeBlock(CodeBlock),
    DataBlock(DataBlock),
}

// ProxyBlock
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct ProxyBlock {
    pub uuid: Uuid,
}

impl TryFrom<proto::ProxyBlock> for ProxyBlock {
    type Error = anyhow::Error;
    fn try_from(message: proto::ProxyBlock) -> Result<Self> {
        Ok(ProxyBlock {
            uuid: parse_uuid(&message.uuid)?,
        })
    }
}

// Symbol
// -----------------------------------------------------------------------------

#[derive(Debug)]
pub struct Symbol {
    pub uuid: Uuid,
    pub name: String,
    //TODO: optional_payload
}

impl TryFrom<proto::Symbol> for Symbol {
    type Error = anyhow::Error;
    fn try_from(message: proto::Symbol) -> Result<Self> {
        Ok(Symbol {
            uuid: parse_uuid(&message.uuid)?,
            name: message.name,
        })
        //
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
