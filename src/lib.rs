#![allow(dead_code)]
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::path::Path;

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
    T: Allocate + Deallocate,
{
    inner: *mut T,
    context: *mut Context,
    kind: PhantomData<T>,
}

impl<T> Node<T>
where
    T: Allocate + Deallocate,
{
    fn new(context: &mut Context, ptr: *mut T) -> Self {
        Node {
            inner: ptr,
            context: &mut *context,
            kind: PhantomData,
        }
    }
}

pub trait Allocate {
    fn allocate(self, context: &mut Context) -> Node<Self>
    where
        Self: Deallocate,
        Self: Sized;
}

pub trait Deallocate {
    fn deallocate(self, context: &mut Context);
}

pub trait Index<T> {
    fn find(context: &Context, uuid: &Uuid) -> Option<*mut T>;
}

impl<T> Drop for Node<T>
where
    T: Allocate + Deallocate,
{
    fn drop(&mut self) {
        assert!(!self.inner.is_null());
        assert!(!self.context.is_null());
        let node = unsafe { Box::from_raw(self.inner) };
        node.deallocate(unsafe { &mut *self.context });
    }
}

impl<T> PartialEq for Node<T>
where
    T: PartialEq + Allocate + Deallocate,
{
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl<T> Deref for Node<T>
where
    T: Allocate + Deallocate,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        assert!(!self.inner.is_null());
        unsafe { &*self.inner }
    }
}

impl<T> DerefMut for Node<T>
where
    T: Allocate + Deallocate,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        assert!(!self.inner.is_null());
        unsafe { &mut *self.inner }
    }
}

pub fn read<P: AsRef<Path>>(path: P) -> Result<(Box<Context>, Node<IR>)> {
    // Read protobuf file.
    let bytes = std::fs::read(path)?;
    let message = proto::Ir::decode(&*bytes)?;

    // Create a context and load the IR.
    let mut context = Context::new();
    let ir = IR::load_protobuf(&mut context, message)?;

    Ok((context, ir))
}
