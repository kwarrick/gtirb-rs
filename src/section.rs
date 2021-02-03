use std::collections::HashSet;

use crate::*;

#[derive(Default, Debug, PartialEq)]
pub(crate) struct Section {
    pub(crate) parent: Option<Index>,

    uuid: Uuid,
    name: String,
    flags: HashSet<SectionFlag>,
    byte_intervals: Vec<Index>,
}

impl Section {
    pub fn new(name: &str) -> Self {
        Section {
            uuid: Uuid::new_v4(),
            ..Default::default()
        }
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

impl Node<Section> {
    pub fn name(&self) -> String {
        self.borrow().name.to_owned()
    }

    pub fn set_name<T: AsRef<str>>(&self, name: T) {
        self.borrow_mut().name = name.as_ref().to_owned();
    }
    pub fn flags(&self) -> HashSet<SectionFlag> {
        self.borrow().flags.clone()
    }

    pub fn add_flag(&self, flag: SectionFlag) {
        self.borrow_mut().flags.insert(flag);
    }

    pub fn remove_flag(&self, flag: SectionFlag) {
        self.borrow_mut().flags.remove(&flag);
    }
}

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
