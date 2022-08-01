use anyhow::Result;

use crate::*;
use gtirb_derive::*;

#[derive(Debug, PartialEq)]
pub enum Payload {
    Value(Addr),
    Referent(Uuid),
}

#[derive(Debug, Default, Node)]
pub struct Symbol {
    uuid: Uuid,
    name: String,
    payload: Option<Payload>,
    parent: WNodeBox<Module>,
}

impl Symbol {
    pub fn new(context: &mut Context, name: &str) -> SymbolRef {
        let symbol = Symbol {
            uuid: Uuid::new_v4(),
            name: name.to_owned(),
            ..Default::default()
        };
        context.add_symbol(symbol)
    }

    pub(crate) fn load_protobuf(
        context: &mut Context,
        message: proto::Symbol,
    ) -> Result<SymbolRef> {
        use crate::proto::symbol::OptionalPayload;

        let payload = match message.optional_payload {
            Some(OptionalPayload::Value(n)) => Some(Payload::Value(Addr(n))),
            Some(OptionalPayload::ReferentUuid(bytes)) => {
                Some(Payload::Referent(crate::util::parse_uuid(&bytes)?))
            }
            None => None,
        };

        let symbol = Symbol {
            parent: WNodeBox::new(),

            uuid: crate::util::parse_uuid(&message.uuid)?,
            name: message.name,
            payload: payload,
        };

        let symbol = context.add_symbol(symbol);

        Ok(symbol)
    }
}

impl SymbolRef {
}
