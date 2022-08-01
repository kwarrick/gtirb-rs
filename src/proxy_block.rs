use anyhow::Result;

use crate::*;

#[derive(Default, Debug, gtirb_derive::Node)]
pub struct ProxyBlock {
    uuid: Uuid,
    pub(crate) parent: WNodeBox<Module>,
}

impl ProxyBlock {
    pub fn new(context: &mut Context) -> ProxyBlockRef {
        let proxy_block = ProxyBlock {
            uuid: Uuid::new_v4(),
            ..Default::default()
        };
        context.add_proxy_block(proxy_block)
    }

    pub(crate) fn load_protobuf(
        context: &mut Context,
        message: proto::ProxyBlock,
    ) -> Result<ProxyBlockRef> {
        let proxy_block = ProxyBlock {
            parent: WNodeBox::new(),
            uuid: crate::util::parse_uuid(&message.uuid)?,
        };

        let proxy_block = context.add_proxy_block(proxy_block);

        Ok(proxy_block)
    }
}

impl ProxyBlockRef {
}
