use crate::*;

#[derive(Default, Debug, gtirb_derive::Node)]
pub struct DataBlock {
    uuid: Uuid,
    parent: WNodeBox<ByteInterval>,
}

impl DataBlock {
    pub fn new(ctx: &mut Context) -> DataBlockRef {
        let block = Self {
            uuid: Uuid::new_v4(),
            ..Default::default()
        };
        ctx.add_data_block(block)
    }
}

impl DataBlockRef {
}
