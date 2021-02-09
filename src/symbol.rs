use crate::*;

#[derive(Debug, PartialEq)]
pub(crate) enum Payload {
    Value(Addr),
    Referent(Uuid),
}

#[derive(Debug, Default, PartialEq)]
pub(crate) struct Symbol {
    pub(crate) parent: Option<Index>,

    uuid: Uuid,
    payload: Option<Payload>,
}

impl Symbol {
    pub fn new(name: &str) -> Self {
        Symbol {
            uuid: Uuid::new_v4(),
            ..Default::default()
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
