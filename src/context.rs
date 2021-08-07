use std::collections::HashMap;

use crate::*;

#[derive(Debug)]
pub struct Context {
    pub(crate) ir: HashMap<Uuid, *mut IR>,
    pub(crate) modules: HashMap<Uuid, *mut Module>,
}

impl Context {
    pub fn new() -> Box<Context> {
        Box::new(Context {
            ir: HashMap::new(),
            modules: HashMap::new(),
        })
    }

    pub fn read<P: AsRef<Path>>(path: P) -> Result<Box<Context>> {
        let bytes = std::fs::read(path)?;

        let mut context = Context::new();
        let message = proto::Ir::decode(&*bytes)?;
        let node = IR::load_protobuf(&mut context, message)?;
        let uuid = node.uuid();
        context.add_ir(uuid, node.ptr);

        Ok(context)
    }

    // TODO:
    // fn add_node<T>(node: Node<T>) {}

    pub(crate) fn add_ir(&mut self, uuid: Uuid, node: *mut IR) -> *mut IR {
        self.ir.insert(uuid, node);
        node
    }

    pub(crate) fn add_module(
        &mut self,
        uuid: Uuid,
        node: *mut Module,
    ) -> *mut Module {
        self.modules.insert(uuid, node);
        node
    }

    // pub fn find_node<T>(&self, uuid: &Uuid) -> Option<&Node<T>> {
    //     self.modules.get(uuid)
    // }

    // pub fn find_node_mut<T>(&mut self, uuid: &Uuid) -> Option<&mut Node<T>> {
    //     self.modules.get_mut(uuid)
    // }
}
