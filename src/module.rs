use crate::*;

#[derive(Debug, Default)]
pub(crate) struct Module {
    uuid: Uuid,

    name: String,
    binary_path: String,

    entry_point: Option<Index>,
    rebase_delta: i64,
    preferred_address: u64,


    sections: Vec<Index>,
    symbols: Vec<Index>,
    proxies: Vec<Index>,

}

impl Module {
    pub fn new() -> Self {
        Module {
            uuid: Uuid::new_v4(),
            ..Default::default()
        }
    }
}

impl Unique for Module {
    fn uuid(&self) -> Uuid {
        self.uuid
    }

    fn set_uuid(&mut self, uuid: Uuid) {
        self.uuid = uuid;
    }
}

impl Node<Module> {

}

impl Indexed<Module> for Node<Module> {
    fn get_ref(
        &self,
        (index, _): (Index, PhantomData<Module>)
    ) -> Option<Ref<Module>> {
        let context = self.context.borrow();
        if context.module.contains(index) {
            Some(Ref::map(context, |ctx| &ctx.module[index]))
        } else {
            None
        }
    }

    fn get_ref_mut(
        &self,
        (index, _): (Index, PhantomData<Module>),
    ) -> Option<RefMut<Module>> {
        let context = self.context.borrow_mut();
        if context.module.contains(index) {
            Some(RefMut::map(context, |ctx| &mut ctx.module[index]))
        } else {
            None
        }
    }
}
