#![allow(dead_code)]
use std::cell::RefCell;
use std::cell::{Ref, RefMut};
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

mod module;
pub use module::Module;

mod section;
use section::Section;

mod byte_interval;
use byte_interval::ByteInterval;

// mod code_block;
// use code_block::CodeBlock;

// mod data_block;
// use data_block::DataBlock;

mod proxy_block;
use proxy_block::ProxyBlock;

mod symbol;
use symbol::Symbol;

// mod symbolic_expression;
// use symbolic_expression::SymbolicExpression;

mod util;

type NodeBox<T> = Rc<RefCell<T>>;

#[derive(Debug)]
pub struct Node<T>
where
    T: Index + Unique,
{
    inner: NodeBox<T>,
    context: Context,
}

impl<T> Node<T>
where
    T: Index + Unique,
{
    fn new(context: &Context, ptr: NodeBox<T>) -> Self {
        Node {
            inner: NodeBox::clone(&ptr),
            context: context.clone(),
        }
    }

    fn borrow(&self) -> Ref<T> {
        self.inner.borrow()
    }

    fn borrow_mut(&mut self) -> RefMut<T> {
        self.inner.borrow_mut()
    }

    fn uuid(&self) -> Uuid {
        self.inner.borrow().uuid()
    }

    fn set_uuid(&mut self, uuid: Uuid) {
        self.inner.borrow_mut().set_uuid(uuid);
    }
}

impl<T> Drop for Node<T>
where
    T: Index + Unique,
{
    fn drop(&mut self) {
        if !T::rooted(Rc::clone(&self.inner)) {
            eprintln!("dropped: {:?}", self.uuid());
            T::remove(&mut self.context, Rc::clone(&self.inner));
        }
    }
}

impl<T> PartialEq for Node<T>
where
    T: PartialEq + Index + Unique,
{
    fn eq(&self, other: &Self) -> bool {
        *self.inner.borrow() == *other.inner.borrow()
    }
}

pub struct Iter<'a, T: 'a> {
    inner: Option<Ref<'a, [NodeBox<T>]>>,
    context: &'a Context,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Index + Unique,
{
    type Item = Node<T>;

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
            return Some(Node::new(self.context, Rc::clone(&head)));
        }
        None
    }
}

pub trait Unique {
    fn uuid(&self) -> Uuid;
    fn set_uuid(&mut self, uuid: Uuid);
}

pub trait Index {
    // Consumes a `T`,  attaches it to a `Context`, and returns a boxed reference.
    fn insert(context: &mut Context, node: Self) -> NodeBox<Self>;

    // Receives a boxed `T`, removes it from a `Context`, and returns the boxed reference.
    fn remove(context: &mut Context, ptr: NodeBox<Self>) -> NodeBox<Self>;

    // Locates a `T` as it was indexed and returns the boxed reference.
    fn search(context: &Context, uuid: &Uuid) -> Option<NodeBox<Self>>;

    // Determine if node is unrooted, i.e. has no parent node.
    fn rooted(ptr: NodeBox<Self>) -> bool;
}

pub fn read<P: AsRef<Path>>(path: P) -> Result<(Context, Node<IR>)> {
    // Read protobuf file.
    let bytes = std::fs::read(path)?;
    let message = proto::Ir::decode(&*bytes)?;

    // Create a context and load the IR.
    let mut context = Context::new();
    let ir = IR::load_protobuf(&mut context, message)?;

    Ok((context, ir))
}
