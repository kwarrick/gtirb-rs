use std::convert::TryFrom;

use anyhow::Result;
use uuid::Uuid;

use crate::proto;
use crate::util::parse_uuid;

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
    }
}
