use std::path::Path;

use anyhow::Result;
use prost::Message;

mod proto {
    include!(concat!(env!("OUT_DIR"), "/proto.rs"));
}

pub struct IR {
    inner: proto::Ir,
}

impl IR {
    pub fn load_protobuf<P: AsRef<Path>>(path: P) -> Result<Self> {
        let bytes = std::fs::read(path)?;
        Ok(IR { inner: proto::Ir::decode(&*bytes)? })
    }

    pub fn version(&self) -> u32 {
        self.inner.version
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
