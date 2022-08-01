use crate::*;

#[derive(Default, Debug, gtirb_derive::Node)]
pub struct CodeBlock {
    uuid: Uuid,
    parent: WNodeBox<ByteInterval>,
}

impl CodeBlock {
    pub fn new(ctx: &mut Context) -> CodeBlockRef {
        let block = Self {
            uuid: Uuid::new_v4(),
            ..Default::default()
        };
        ctx.add_code_block(block)
    }
}

impl CodeBlockRef {
}
