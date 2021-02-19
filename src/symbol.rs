use anyhow::Result;

use crate::*;

#[derive(Debug, PartialEq)]
pub enum Payload {
    Value(Addr),
    Referent(Uuid),
}

#[derive(Debug, Default, PartialEq)]
pub struct Symbol {
    pub(crate) parent: Option<Index>,

    uuid: Uuid,
    name: String,
    payload: Option<Payload>,
}

impl Symbol {
    pub fn new(name: &str) -> Self {
        Symbol {
            uuid: Uuid::new_v4(),
            name: name.to_owned(),
            ..Default::default()
        }
    }

    pub(crate) fn load_protobuf(
        context: Rc<RefCell<Context>>,
        message: proto::Symbol,
    ) -> Result<Index> {
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

        Ok(context.borrow_mut().symbol.insert(symbol))
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

impl Indexed<Symbol> for Node<Symbol> {
    fn arena(&self) -> Ref<Arena<Symbol>> {
        Ref::map(self.context.borrow(), |ctx| &ctx.symbol)
    }

    fn arena_mut(&self) -> RefMut<Arena<Symbol>> {
        RefMut::map(self.context.borrow_mut(), |ctx| &mut ctx.symbol)
    }
}

impl Child<Module> for Node<Symbol> {
    fn parent(&self) -> (Option<Index>, PhantomData<Module>) {
        (self.borrow().parent, PhantomData)
    }

    fn set_parent(&self, (index, _): (Index, PhantomData<Module>)) {
        self.borrow_mut().parent.replace(index);
    }
}
