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
use context::Context;

mod ir;
use ir::IR;

mod module;
use module::Module;

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
pub struct Node<T> {
    ptr: *mut T,
    ctx: *mut Context,
    kind: PhantomData<T>,
}

impl<T> Node<T> {
    fn new(ctx: *mut Context, ptr: *mut T) -> Self {
        Node {
            ptr,
            ctx,
            kind: PhantomData,
        }
    }
}

// TODO: Use the Context pointer to remove the node and free values.
// impl<T> Drop for Node<T> {
//     fn drop(&mut self) {
//         // let _: Box<T> = unsafe { Box::from_raw(self.ptr) };
//     }
// }

impl<T> PartialEq for Node<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl<T> Deref for Node<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        assert!(!self.ptr.is_null());
        unsafe { &*self.ptr }
    }
}

impl<T> DerefMut for Node<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        assert!(!self.ptr.is_null());
        unsafe { &mut *self.ptr }
    }
}

type Iter<'a, T> = std::slice::Iter<'a, Node<T>>;
type IterMut<'a, T> = std::slice::IterMut<'a, Node<T>>;

pub type GTIRB = Context;
