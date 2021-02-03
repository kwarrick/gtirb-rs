use crate::*;

#[derive(Default, Debug, PartialEq)]
pub(crate) struct Section {
    uuid: Uuid,
    module: Option<Index>,
}

impl Section {
    pub fn new(name: &str) -> Self {
        Section {
            uuid: Uuid::new_v4(),
            ..Default::default()
        }
    }

    pub fn module(&self) -> Option<Index> {
        self.module
    }

    pub fn set_module(&mut self, index: Index) {
        self.module.replace(index);
    }
}

impl Unique for Section {
    fn uuid(&self) -> Uuid {
        self.uuid
    }

    fn set_uuid(&mut self, uuid: Uuid) {
        self.uuid = uuid;
    }
}

impl Node<Module> {}

impl Indexed<Section> for Node<Section> {
    fn get_ref(
        &self,
        (index, _): (Index, PhantomData<Section>),
    ) -> Option<Ref<Section>> {
        let context = self.context.borrow();
        if context.section.contains(index) {
            Some(Ref::map(context, |ctx| &ctx.section[index]))
        } else {
            None
        }
    }

    fn get_ref_mut(
        &self,
        (index, _): (Index, PhantomData<Section>),
    ) -> Option<RefMut<Section>> {
        let context = self.context.borrow_mut();
        if context.section.contains(index) {
            Some(RefMut::map(context, |ctx| &mut ctx.section[index]))
        } else {
            None
        }
    }
}
