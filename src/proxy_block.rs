use std::convert::TryFrom;

use anyhow::Result;
use uuid::Uuid;

use crate::proto;
use crate::util::parse_uuid;

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
