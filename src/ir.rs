use crate::*;

#[derive(Debug, PartialEq)]
pub(crate) struct IR {
    uuid: Uuid,
    modules: Vec<Index>,
    version: u32,
}

impl Unique for IR {
    fn uuid(&self) -> Uuid {
        self.uuid
    }

    fn set_uuid(&mut self, uuid: Uuid) {
        self.uuid = uuid;
    }
}

impl IR {
    pub fn new() -> Node<IR> {
        let mut context = Context::new();
        let index = context.ir.insert(IR {
            uuid: Uuid::new_v4(),
            modules: Vec::new(),
            version: 1,
        });
        Node {
            index,
            context: Rc::new(RefCell::new(context)),
            kind: PhantomData,
        }
    }
}

impl Node<IR> {
    fn find_node<U>(&self, uuid: Uuid) -> Option<Node<U>>
    where
        Node<U>: Indexed<U>,
        U: Unique,
    {
        self.context
            .borrow()
            .uuid_map
            .get(&uuid)
            .map(|index| Node {
                index: *index,
                context: self.context.clone(),
                kind: PhantomData,
            })
            .filter(|node| {
                node.get_ref((node.index, PhantomData))
                    .map(|inner| inner.uuid() != uuid)
                    .unwrap_or(true)
            })
    }

    pub fn version(&self) -> u32 {
        self.borrow().version
    }

    pub fn set_version(&self, version: u32) {
        self.borrow_mut().version = version
    }

    pub fn modules(&self) -> NodeIterator<IR, Module> {
        NodeIterator {
            index: 0,
            parent: Node {
                index: self.index,
                context: self.context.clone(),
                kind: PhantomData,
            },
            kind: PhantomData,
        }
    }

    pub fn add_module(&self, module: Module) {
        let index = self.context.borrow_mut().add_module(module);
        self.borrow_mut().modules.push(index);
    }
}

impl Indexed<IR> for Node<IR> {
    fn get_ref(
        &self,
        (index, _): (Index, PhantomData<IR>)
    ) -> Option<Ref<IR>> {
        let context = self.context.borrow();
        if context.ir.contains(index) {
            Some(Ref::map(context, |ctx| &ctx.ir[index]))
        } else {
            None
        }
    }

    fn get_ref_mut(
        &self,
        (index, _): (Index, PhantomData<IR>),
    ) -> Option<RefMut<IR>> {
        let context = self.context.borrow_mut();
        if context.ir.contains(index) {
            Some(RefMut::map(context, |ctx| &mut ctx.ir[index]))
        } else {
            None
        }
    }
}

impl Container<Module> for Node<IR> {
    fn get(&self, index: usize) -> (Option<Index>, PhantomData<Module>) {
        let module_index = self.context.borrow().ir[self.index]
            .modules
            .get(index)
            .cloned();
        (module_index, PhantomData)
    }
}

#[cfg(test)]
mod tests {
    use super::IR;

    #[test]
    fn can_create_new_ir() {
        let ir = IR::new();
        assert_eq!(ir.version(), 1);
        assert_eq!(ir.modules().count(), 0);
    }

    #[test]
    fn can_set_version() {
        let ir = IR::new();
        ir.set_version(42);
        assert_eq!(ir.version(), 42);
    }
}
