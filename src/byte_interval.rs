use std::convert::TryFrom;
use std::convert::TryInto;

use anyhow::Result;
use uuid::Uuid;

use crate::{Block};
use crate::proto;
use crate::util::parse_uuid;

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
