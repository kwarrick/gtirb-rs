use std::convert::TryFrom;
use std::convert::TryInto;

use anyhow::{anyhow, Result};
use uuid::Uuid;

use crate::{
    ByteInterval,
    SectionFlag,
};
use crate::proto;
use crate::util::parse_uuid;

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
