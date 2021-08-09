#![allow(dead_code)]
use std::cell::RefCell;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::path::Path;
use std::rc::Rc;

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

// mod section;
// use section::Section;

// mod byte_interval;
// use byte_interval::ByteInterval;

// mod code_block;
// use code_block::CodeBlock;

// mod data_block;
// use data_block::DataBlock;

// mod proxy_block;
// use proxy_block::ProxyBlock;

// mod symbol;
// use symbol::Symbol;

// mod symbolic_expression;
// use symbolic_expression::SymbolicExpression;

mod util;

#[derive(Debug)]
pub struct Node<T: Sized>
where
    T: Allocate + Deallocate + Index + Unique + Sized,
{
    inner: *mut T,
    kind: PhantomData<T>,
    context: Context,
}

impl<T> Node<T>
where
    T: Allocate + Deallocate + Index + Unique + Sized,
{
    fn new(context: &Context, ptr: *mut T) -> Self {
        Node {
            inner: ptr,
            kind: PhantomData,
            context: context.clone(),
        }
    }
}

impl<T> PartialEq for Node<T>
where
    T: PartialEq + Allocate + Deallocate + Index + Unique,
{
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl<T> Deref for Node<T>
where
    T: Allocate + Deallocate + Index + Unique,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        assert!(!self.inner.is_null());
        unsafe { &*self.inner }
    }
}

impl<T> DerefMut for Node<T>
where
    T: Allocate + Deallocate + Index + Unique,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        assert!(!self.inner.is_null());
        unsafe { &mut *self.inner }
    }
}

pub struct Iter<'a, T> {
    iter: std::collections::hash_map::Iter<'a, Uuid, *mut T>,
    context: &'a Context,
}

impl<'a, T> Iterator for Iter<'a, T>
where
    T: Allocate + Deallocate + Index + Unique,
{
    type Item = Node<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter
            .next()
            .map(|(_, ptr)| Node::new(self.context, *ptr))
    }
}

pub trait Unique {
    fn uuid(&self) -> Uuid;
    fn set_uuid(&mut self, uuid: Uuid);
}

pub trait Index {
    // Consumes a `T` to produce a raw pointer..
    fn insert(context: &mut Context, node: Self) -> *mut Self;

    // Receives a raw pointer to produce an owned `Box<T>` for deallocation.
    fn remove(context: &mut Context, ptr: *mut Self) -> Option<Box<Self>>;

    // Locates a node data as it was indexed and return a raw pointer.
    fn search(context: &Context, uuid: &Uuid) -> Option<*mut Self>;

    // Determine if node is unrooted, i.e. has no parent node.
    fn rooted(node: &Self) -> bool;
}

pub trait Allocate {
    fn allocate(context: &mut Context, node: Self) -> Node<Self>
    where
        Self: Index,
        Self: Unique,
        Self: Sized;
}

impl<T> Allocate for T
where
    T: Index + Unique,
{
    fn allocate(context: &mut Context, node: Self) -> Node<T> {
        let ptr = T::insert(context, node);
        Node::new(context, ptr)
    }
}

pub trait Deallocate {
    fn deallocate(node: &mut Node<Self>)
    where
        Self: Index,
        Self: Unique,
        Self: Sized;
}

impl<T> Deallocate for T
where
    T: Index + Unique,
{
    fn deallocate(node: &mut Node<Self>) {
        if !T::rooted(node) {
            T::remove(&mut node.context, node.inner);
        }
        node.inner = std::ptr::null_mut();
    }
}

impl<T> Drop for Node<T>
where
    T: Allocate + Deallocate + Index + Unique,
{
    fn drop(&mut self) {
        assert!(!self.inner.is_null());
        T::deallocate(self);
    }
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
