use std::convert::TryFrom;

use anyhow::{anyhow, Result};
use uuid::Uuid;

use crate::proto;
use crate::util::parse_uuid;

// CodeBlock
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
