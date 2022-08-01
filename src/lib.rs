#![allow(dead_code)]
use std::cell::RefCell;
use std::cell::{Ref, RefMut};
use std::marker::PhantomData;
use std::path::Path;
use std::rc::{Rc, Weak};

use anyhow::Result;
use prost::Message;
use uuid::Uuid;

mod proto {
    include!(concat!(env!("OUT_DIR"), "/gtirb.proto.rs"));
}

pub use proto::{ByteOrder, FileFormat, Isa as ISA, SectionFlag};

mod addr;
use addr::*;

mod context;
pub use context::Context;

mod ir;
pub use ir::IR;
pub use ir::IRRef;

mod module;
pub use module::Module;
pub use module::ModuleRef;

mod section;
pub use section::Section;
pub use section::SectionRef;

mod byte_interval;
pub use byte_interval::ByteInterval;
pub use byte_interval::ByteIntervalRef;

mod code_block;
pub use code_block::CodeBlock;
pub use code_block::CodeBlockRef;

mod data_block;
pub use data_block::DataBlock;
pub use data_block::DataBlockRef;

mod proxy_block;
pub use proxy_block::ProxyBlock;
pub use proxy_block::ProxyBlockRef;

mod symbol;
pub use symbol::Symbol;
pub use symbol::SymbolRef;

mod symbolic_expression;
pub use symbolic_expression::SymbolicExpression;

mod util;

type NodeBox<T> = Rc<RefCell<T>>;
type WNodeBox<T> = Weak<RefCell<T>>;

pub trait Node<T>
where
    T: Unique,
{
    fn get_inner(&self) -> &NodeBox<T>;
    fn get_context(&self) -> &Context;

    fn borrow(&self) -> Ref<T> {
        self.get_inner().borrow()
    }

    fn borrow_mut(&mut self) -> RefMut<T> {
        self.get_inner().borrow_mut()
    }

    fn uuid(&self) -> Uuid {
        self.get_inner().borrow().uuid()
    }
}

pub struct Iter<'a, T: 'a, TRef> {
    inner: Option<Ref<'a, [NodeBox<T>]>>,
    context: &'a Context,
    phantom: PhantomData<TRef>,
}

impl<'a, T, TRef> Iterator for Iter<'a, T, TRef>
where
    T: Unique,
    TRef: IsRefFor<T>,
{
    type Item = TRef;

    fn next(&mut self) -> Option<Self::Item> {
        if self.inner.is_none() {
            return None;
        }
        if let Some(borrow) = self.inner.take() {
            if borrow.is_empty() {
                return None;
            }
            let (begin, end) =
                Ref::map_split(borrow, |slice| slice.split_at(1));
            self.inner.replace(end);
            let head = Ref::map(begin, |slice| &slice[0]);
            return Some(TRef::new(self.context, Rc::clone(&head)));
        }
        None
    }
}

pub trait Unique {
    fn uuid(&self) -> Uuid;
}

pub trait IsRefFor<T>
where
    T: Unique,
    Self: Node<T>,
{
    fn new(context: &Context, node: NodeBox<T>) -> Self;
}

pub fn read<P: AsRef<Path>>(path: P) -> Result<(Context, IRRef)> {
    // Read protobuf file.
    let bytes = std::fs::read(path)?;
    let message = proto::Ir::decode(&*bytes)?;

    // Create a context and load the IR.
    let mut context = Context::new();
    let ir = IR::load_protobuf(&mut context, message)?;

    Ok((context, ir))
}
