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
pub struct NodeRef<'a, T, U> {
    ptr: *const T,
    kind: PhantomData<T>,
    lender: &'a U,
}

#[derive(Debug)]
pub struct NodeMut<'a, T, U> {
    ptr: *mut T,
    kind: PhantomData<T>,
    lender: &'a mut U,
}

// impl<T> PartialEq for NodeMut<T>
// where
//     T: PartialEq,
// {
//     fn eq(&self, other: &Self) -> bool {
//         self.deref() == other.deref()
//     }
// }

// impl<T> PartialEq<NodeRef<T>> for NodeMut<T>
// where
//     T: PartialEq,
// {
//     fn eq(&self, other: &NodeRef<T>) -> bool {
//         self.deref() == other.deref()
//     }
// }

// impl<T> PartialEq for NodeRef<T>
// where
//     T: PartialEq,
// {
//     fn eq(&self, other: &Self) -> bool {
//         self.deref() == other.deref()
//     }
// }

// impl<T> PartialEq<NodeMut<T>> for NodeRef<T>
// where
//     T: PartialEq,
// {
//     fn eq(&self, other: &NodeMut<T>) -> bool {
//         self.deref() == other.deref()
//     }
// }

impl<'a, T, U> Deref for NodeRef<'a, T, U> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<'a, T, U> Deref for NodeMut<'a, T, U> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.ptr }
    }
}

impl<'a, T, U> DerefMut for NodeMut<'a, T, U> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.ptr }
    }
}

pub struct Iter<'a, T, U> {
    iter: std::slice::Iter<'a, *mut T>,
    lender: &'a U,
}

impl<'a, T, U> Iterator for Iter<'a, T, U> {
    type Item = NodeRef<'a, T, U>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|ptr| NodeRef {
            ptr: *ptr as *const T,
            kind: PhantomData,
            lender: self.lender,
        })
    }
}

pub struct IterMut<'t, T, U> {
    iter: std::slice::IterMut<'t, *mut T>,
    lender: &'t mut U,
}

impl<'t, T, U> Iterator for IterMut<'t, T, U> {
    type Item = NodeMut<'t, T, U>;

    fn next<'a>(&'a mut self) -> Option<Self::Item> {
        self.iter.next().map(|ptr| NodeMut {
            ptr: *ptr,
            kind: PhantomData,
            lender: self.lender,
        })
    }
}

// TODO: Free the tree!
// impl<T> Drop for NodeMut<T> {
//     fn drop(&mut self) {
//         // let _: Box<T> = unsafe { Box::from_raw(self.ptr) };
//     }
// }
