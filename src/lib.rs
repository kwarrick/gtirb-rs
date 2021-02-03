use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;

use generational_arena::{Arena, Index};
use uuid::Uuid;

mod proto {
    include!(concat!(env!("OUT_DIR"), "/gtirb.proto.rs"));
}

pub use proto::{ByteOrder, FileFormat, Isa as ISA, SectionFlag};

mod ir;
use ir::*;

mod module;
use module::*;

mod section;
use section::*;

mod byte_interval;
use byte_interval::*;

mod code_block;
use code_block::*;

mod data_block;
use data_block::*;

mod proxy_block;
use proxy_block::*;

#[derive(Clone, Debug)]
struct Node<T> {
    index: Index,
    context: Rc<RefCell<Context>>,
    kind: PhantomData<T>,
}

struct NodeIterator<T, U> {
    index: usize,
    parent: Node<T>,
    kind: PhantomData<U>,
}

trait Container<T> {
    fn get(&self, position: usize) -> (Option<Index>, PhantomData<T>);
    fn remove(&self, index: (Index, PhantomData<T>));
}

impl<T, U> Iterator for NodeIterator<T, U>
where
    Node<T>: Container<U>,
{
    type Item = Node<U>;

    fn next(&mut self) -> Option<Self::Item> {
        let (child, _) = self.parent.get(self.index);
        self.index += 1;
        child.map(|index| Node {
            index,
            context: self.parent.context.clone(),
            kind: PhantomData,
        })
    }
}

trait Indexed<T> {
    fn get_ref(&self, index: (Index, PhantomData<T>)) -> Option<Ref<T>>;
    fn get_ref_mut(&self, index: (Index, PhantomData<T>)) -> Option<RefMut<T>>;
}

trait Borrow<T> {
    fn borrow(&self) -> Ref<T>;
    fn borrow_mut(&self) -> RefMut<T>;
}

impl<T> Borrow<T> for Node<T>
where
    Node<T>: Indexed<T>,
{
    fn borrow(&self) -> Ref<T> {
        self.get_ref((self.index, PhantomData))
            .expect("indexed node")
    }

    fn borrow_mut(&self) -> RefMut<T> {
        self.get_ref_mut((self.index, PhantomData))
            .expect("indexed node")
    }
}

trait Unique {
    fn uuid(&self) -> Uuid;
    fn set_uuid(&mut self, uuid: Uuid);
}

impl<T> Node<T>
where
    Node<T>: Borrow<T>,
    T: Unique,
{
    pub fn uuid(&self) -> Uuid {
        self.borrow().uuid()
    }

    pub fn set_uuid(&self, uuid: Uuid) {
        let mut context = self.context.borrow_mut();
        let mut node = self.borrow_mut();
        context.uuid_map.remove(&node.uuid());
        context.uuid_map.insert(uuid, self.index);
        node.set_uuid(uuid);
    }
}

impl<T> PartialEq for Node<T>
where
    Node<T>: Indexed<T> + Borrow<T>,
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (
            self.get_ref((self.index, PhantomData)),
            other.get_ref((other.index, PhantomData)),
        ) {
            (Some(a), Some(b)) => *a == *b,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct Symbol;
#[derive(Debug)]
struct SymbolicExpression;

#[derive(Debug, Default)]
struct Context {
    uuid_map: HashMap<Uuid, Index>,

    ir: Arena<IR>,
    module: Arena<Module>,
    section: Arena<Section>,
    byte_interval: Arena<ByteInterval>,
    code_block: Arena<CodeBlock>,
    data_block: Arena<DataBlock>,
    proxy_block: Arena<ProxyBlock>,
    symbol: Arena<Symbol>,
    symbolic_expression: Arena<SymbolicExpression>,
}

impl Context {
    fn new() -> Self {
        Default::default()
    }
}
