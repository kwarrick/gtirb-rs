use std::convert::TryInto;

use anyhow::{Context, Result};
use uuid::Uuid;

// Util
// -----------------------------------------------------------------------------
pub(crate) fn parse_uuid(bytes: &[u8]) -> Result<Uuid> {
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
