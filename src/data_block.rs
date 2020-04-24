use std::convert::TryFrom;

use anyhow::{anyhow, Result};
use uuid::Uuid;

use crate::proto;
use crate::util::parse_uuid;

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
