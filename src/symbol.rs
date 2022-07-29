use anyhow::Result;

use crate::*;

#[derive(Debug, PartialEq)]
pub enum Payload {
    Value(Addr),
    Referent(Uuid),
}

#[derive(Debug, Default)]
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
        SymbolRef::new(context.add_node(symbol))
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

        let symbol = SymbolRef::new(context.add_node(symbol));

        Ok(symbol)
    }

    pub(crate) fn set_parent(&mut self, parent: Option<&NodeBox<Module>>) {
        self.parent = match parent {
            Some(ptr) => Rc::downgrade(ptr),
            None => WNodeBox::new(),
        }
    }
}

impl Unique for Symbol {
    fn uuid(&self) -> Uuid {
        self.uuid
    }

    fn set_uuid(&mut self, uuid: Uuid) {
        self.uuid = uuid;
    }
}

pub struct SymbolRef {
    pub(crate) node: Node<Symbol>
}

impl SymbolRef {
    pub(crate) fn new(node: Node<Symbol>) -> Self {
        Self { node: node }
    }
}

impl Index for Symbol {
    fn insert(context: &mut Context, node: Self) -> NodeBox<Self> {
        let uuid = node.uuid();
        let boxed = Rc::new(RefCell::new(node));
        context
            .index
            .borrow_mut()
            .symbols
            .insert(uuid, Rc::downgrade(&boxed));
        boxed
    }

    fn remove(context: &mut Context, ptr: &NodeBox<Self>) {
        let uuid = ptr.borrow().uuid();
        context.index.borrow_mut().symbols.remove(&uuid);
    }

    fn rooted(ptr: NodeBox<Self>) -> bool {
        ptr.borrow().parent.upgrade().is_some()
    }
}

impl IsRefFor<Symbol> for SymbolRef {
    fn new(node: Node<Symbol>) -> Self {
        Self { node: node }
    }
}
