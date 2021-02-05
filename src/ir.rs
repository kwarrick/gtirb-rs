use crate::*;

#[derive(Debug, Default, PartialEq)]
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
            .filter(|node| node.arena().contains(node.index))
            .filter(|node| node.borrow().uuid() == uuid)
    }

    pub fn version(&self) -> u32 {
        self.borrow().version
    }

    pub fn set_version(&self, version: u32) {
        self.borrow_mut().version = version
    }

    pub fn modules(&self) -> NodeIterator<IR, Module> {
        self.node_iter()
    }

    pub fn add_module(&self, module: Module) -> Node<Module> {
        self.add_node(module)
    }

    pub fn remove_module(&self, node: Node<Module>) {
        self.remove_node(node);
    }
}

impl Indexed<IR> for Node<IR> {
    fn arena(&self) -> Ref<Arena<IR>> {
        Ref::map(self.context.borrow(), |ctx| &ctx.ir)
    }
    fn arena_mut(&self) -> RefMut<Arena<IR>> {
        RefMut::map(self.context.borrow_mut(), |ctx| &mut ctx.ir)
    }
}

impl Parent<Module> for Node<IR> {
    fn nodes(&self) -> Ref<Vec<Index>> {
        Ref::map(self.borrow(), |ir| &ir.modules)
    }

    fn nodes_mut(&self) -> RefMut<Vec<Index>> {
        RefMut::map(self.borrow_mut(), |ir| &mut ir.modules)
    }

    fn node_arena(&self) -> Ref<Arena<Module>> {
        Ref::map(self.context.borrow(), |ctx| &ctx.module)
    }
    fn node_arena_mut(&self) -> RefMut<Arena<Module>> {
        RefMut::map(self.context.borrow_mut(), |ctx| &mut ctx.module)
    }
}

use std::slice::Iter;

pub struct NodeIterator_<'a, T: 'a> {
    index: Ref<'a, [Index]>,
    arena: Ref<'a, Arena<T>>,
}

impl<'a, T: 'a> Iterator for NodeIterator_<'a, T> {
    type Item = Ref<'a, T>;

    fn next(&mut self) -> Option<Ref<'a, T>> {
        if self.index.len() == 0 {
            return None;
        }
        let (head, tail) =
            Ref::map_split(Ref::clone(&self.index), |slice| slice.split_at(1));
        self.index = tail;
        Some(Ref::map(Ref::clone(&self.arena), |arena| &arena[head[0]]))
    }
}

impl Node<IR> {
    pub fn modules_(&self) -> NodeIterator_<Module> {
        NodeIterator_ {
            index: Ref::map(self.nodes(), |nodes| &nodes[..]),
            arena: self.node_arena(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_new_ir() {
        let ir = IR::new();
        assert_eq!(ir.version(), 1);
        assert_eq!(ir.modules().count(), 0);
    }

    #[test]
    fn new_ir_is_unique() {
        assert_ne!(IR::new(), IR::new());
    }

    #[test]
    fn can_set_version() {
        let ir = IR::new();
        ir.set_version(42);
        assert_eq!(ir.version(), 42);
    }

    #[test]
    fn can_add_new_module() {
        let ir = IR::new();
        let module = Module::new("dummy");
        ir.add_module(module);
        let module = ir.modules().nth(0);
        assert!(module.is_some());
        assert_eq!(module.unwrap().ir(), ir);
    }

    #[test]
    fn can_remove_module() {
        let ir = IR::new();
        let module = Module::new("dummy");
        let module = ir.add_module(module);
        let uuid = module.uuid();

        ir.remove_module(module);
        assert_eq!(ir.modules().count(), 0);

        let node: Option<Node<Module>> = ir.find_node(uuid);
        assert!(node.is_none());
    }

    #[test]
    fn can_find_node_by_uuid() {
        let ir = IR::new();
        let module = Module::new("dummy");
        let uuid = module.uuid();
        ir.add_module(module);
        let node: Option<Node<Module>> = ir.find_node(uuid);
        assert!(node.is_some());
        assert_eq!(uuid, node.unwrap().uuid());
    }
}
