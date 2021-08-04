#![allow(dead_code)]

use std::pin::Pin;

use uuid::Uuid;

mod proto {
    include!(concat!(env!("OUT_DIR"), "/gtirb.proto.rs"));
}

pub use proto::{ByteOrder, FileFormat, Isa as ISA, SectionFlag};

mod addr;
use addr::*;

mod ir;
pub use ir::read;
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

type Node<T> = Pin<Box<T>>;

type Iter<'a, T> = std::slice::Iter<'a, Node<T>>;
type IterMut<'a, T> = std::slice::IterMut<'a, Node<T>>;

enum NodeRef {
    Module(*mut Module),
}

pub trait Unique {
    fn uuid(&self) -> Uuid;
    fn set_uuid(&mut self, uuid: Uuid);
}

mod util;
