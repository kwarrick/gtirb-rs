use anyhow::Result;

use crate::*;

#[derive(Debug, PartialEq)]
pub enum Payload {
    Value(Addr),
    Referent(Uuid),
}

#[derive(Debug, Default, PartialEq)]
pub struct Symbol {
    uuid: Uuid,
    name: String,
    payload: Option<Payload>,
    pub(crate) parent: Option<*const RefCell<Module>>,
}

impl Symbol {
    pub fn new(context: &mut Context, name: &str) -> Node<Symbol> {
        let symbol = Symbol {
            uuid: Uuid::new_v4(),
            name: name.to_owned(),
            ..Default::default()
        };
        context.add_node(symbol)
    }

    pub(crate) fn load_protobuf(
        context: &mut Context,
        message: proto::Symbol,
    ) -> Result<Node<Symbol>> {
        use crate::proto::symbol::OptionalPayload;

        let payload = match message.optional_payload {
            Some(OptionalPayload::Value(n)) => Some(Payload::Value(Addr(n))),
            Some(OptionalPayload::ReferentUuid(bytes)) => {
                Some(Payload::Referent(crate::util::parse_uuid(&bytes)?))
            }
            None => None,
        };

        let symbol = Symbol {
            parent: None,

            uuid: crate::util::parse_uuid(&message.uuid)?,
            name: message.name,
            payload: payload,
        };

        let symbol = context.add_node(symbol);

        Ok(symbol)
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

impl Node<Symbol> {}

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

    fn search(context: &Context, uuid: &Uuid) -> Option<NodeBox<Self>> {
        context
            .index
            .borrow()
            .symbols
            .get(uuid)
            .map(|ptr| ptr.upgrade())
            .flatten()
    }

    fn rooted(ptr: NodeBox<Self>) -> bool {
        ptr.borrow().parent.is_some()
    }
}
