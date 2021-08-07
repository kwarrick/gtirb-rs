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
pub struct Node<T>
where
    T: Allocate<T> + Deallocate,
{
    ptr: *mut T,
    ctx: *mut Context,
    kind: PhantomData<T>,
}

impl<T> Node<T>
where
    T: Allocate<T> + Deallocate,
{
    fn new(context: &mut Context, ptr: *mut T) -> Self {
        Node {
            ptr,
            ctx: &mut *context,
            kind: PhantomData,
        }
    }
}

pub trait Allocate<T>
where
    T: Allocate<T> + Deallocate,
{
    fn allocate(self, context: &mut Context) -> Node<T>;
}

pub trait Deallocate {
    fn deallocate(self, context: &mut Context);
}

pub trait Index<T> {
    fn find(context: &Context, uuid: &Uuid) -> Option<*mut T>;
}

// impl<T> Deallocate for Node<T>
// {
//     fn deallocate(self, context: &mut Context) {
//         // let node: Box<T> = unsafe { Box::from_raw(self.ptr) };
//     }
// }

impl<T> Drop for Node<T>
where
    T: Allocate<T> + Deallocate,
{
    fn drop(&mut self) {
        assert!(!self.ptr.is_null());
        assert!(!self.ctx.is_null());
        println!("Drop: {:?}", self.ptr);
        let node = unsafe { Box::from_raw(self.ptr) };
        node.deallocate(unsafe { &mut *self.ctx });
    }
}

type Iter<'a, T> = std::slice::Iter<'a, Node<T>>;

type IterMut<'a, T> = std::slice::IterMut<'a, Node<T>>;

impl<T> PartialEq for Node<T>
where
    T: PartialEq + Allocate<T> + Deallocate,
{
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl<T> Deref for Node<T>
where
    T: Allocate<T> + Deallocate,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        assert!(!self.ptr.is_null());
        unsafe { &*self.ptr }
    }
}

impl<T> DerefMut for Node<T>
where
    T: Allocate<T> + Deallocate,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        assert!(!self.ptr.is_null());
        unsafe { &mut *self.ptr }
    }
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
