use std::ops::Deref;
use std::collections::HashMap;
use std::sync::{Arc,RwLock};

use indextree::{Arena,NodeId};
use uuid::Uuid;

mod proto {
    include!(concat!(env!("OUT_DIR"), "/proto.rs"));
}

pub use proto::FileFormat;
pub use proto::Isa as ISA;
pub use proto::SectionFlag;

mod addr;
pub use addr::Addr;

mod ir;
pub use crate::ir::IR;

mod module;
pub use crate::module::Module;

enum GTIRB {
    IR(IR),
    Module(Module),
}

#[derive(Default)]
struct Context {
    index: HashMap<Uuid,NodeId>,
    arena: Arena<GTIRB>,
}

impl Context {
    fn append_node<T>(&mut self, parent: NodeId, child: T) -> NodeId
    where T: Into<Uuid> + Into<GTIRB> + Clone
    {
        // Add node to arena and index.
        let node = self.arena.new_node(child.clone().into());
        self.index.insert(child.into(), node);

        // Append node to parent's children.
        parent.append(node, &mut self.arena);

        node
    }
}

pub struct Node<T> {
    node: NodeId,
    ctx: Arc<RwLock<Context>>,
    inner: T,
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}


// struct FilterOn<T> {
//     node: Node<T>
// }

// pub use crate::ir::read;

// mod section;
// pub use crate::section::Section;

// mod byte_interval;
// pub use crate::byte_interval::Block;
// pub use crate::byte_interval::ByteInterval;

// mod code_block;
// pub use crate::code_block::CodeBlock;

// mod data_block;
// pub use crate::data_block::DataBlock;

// mod proxy_block;
// pub use crate::proxy_block::ProxyBlock;

// mod symbol;
// pub use crate::symbol::Symbol;

// mod util;
