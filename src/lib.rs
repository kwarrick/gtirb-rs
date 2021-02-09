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

mod addr;
use addr::*;

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

mod symbol;
use symbol::*;

mod symbolic_expression;
use symbolic_expression::*;

#[derive(Clone, Debug)]
struct Node<T> {
    index: Index,
    context: Rc<RefCell<Context>>,
    kind: PhantomData<T>,
}

struct NodeIterator<T, U> {
    position: usize,
    parent: Node<T>,
    kind: PhantomData<U>,
}

trait Child<T> {
    fn parent(&self) -> (Option<Index>, PhantomData<T>);
    fn set_parent(&self, index: (Index, PhantomData<T>));
}

trait Parent<T> {
    fn nodes(&self) -> Ref<Vec<Index>>;
    fn nodes_mut(&self) -> RefMut<Vec<Index>>;

    fn node_arena(&self) -> Ref<Arena<T>>;
    fn node_arena_mut(&self) -> RefMut<Arena<T>>;
}

impl<T> Node<T> {
    pub fn node_iter<U>(&self) -> NodeIterator<T, U> {
        NodeIterator {
            position: 0,
            parent: Node {
                index: self.index,
                context: self.context.clone(),
                kind: PhantomData,
            },
            kind: PhantomData,
        }
    }

    pub fn add_node<U: Unique>(&self, node: U) -> Node<U>
    where
        Node<U>: Child<T>,
        Node<T>: Parent<U>,
    {
        // Add node to Context.
        let uuid = node.uuid();
        let index = self.node_arena_mut().insert(node);
        self.context.borrow_mut().uuid_map.insert(uuid, index);

        // Add node to Parent.
        self.nodes_mut().push(index);

        let node = Node {
            index,
            context: self.context.clone(),
            kind: PhantomData,
        };

        // Update parent
        node.set_parent((self.index, PhantomData));
        node
    }

    pub fn remove_node<U: Unique>(&self, node: Node<U>)
    where
        Node<T>: Parent<U>,
        Node<U>: Child<T>,
        Node<U>: Indexed<U>,
    {
        // Consume node.
        let (index, uuid) = { (node.index, node.uuid()) };

        // Remove Child from Parent.
        self.node_arena_mut().remove(node.index);
        let position = self.nodes().iter().position(|i| *i == index).unwrap();
        self.nodes_mut().remove(position);

        // Remove Child from Context.
        self.node_arena_mut().remove(index);
        self.context.borrow_mut().uuid_map.remove(&uuid);
    }
}

impl<T, U> Iterator for NodeIterator<T, U>
where
    Node<T>: Parent<U>,
{
    type Item = Node<U>;

    fn next(&mut self) -> Option<Self::Item> {
        let child = self.parent.nodes().get(self.position).cloned();
        self.position += 1;
        child.map(|index| Node {
            index,
            context: self.parent.context.clone(),
            kind: PhantomData,
        })
    }
}

trait Indexed<T> {
    fn arena(&self) -> Ref<Arena<T>>;
    fn arena_mut(&self) -> RefMut<Arena<T>>;
}

impl<T> Node<T>
where
    Node<T>: Indexed<T>,
{
    fn borrow(&self) -> Ref<T> {
        Ref::map(self.arena(), |a| a.get(self.index).expect("indexed node"))
    }

    fn borrow_mut(&self) -> RefMut<T> {
        RefMut::map(self.arena_mut(), |a| {
            a.get_mut(self.index).expect("indexed node")
        })
    }
}

trait Unique {
    fn uuid(&self) -> Uuid;
    fn set_uuid(&mut self, uuid: Uuid);
}

impl<T> Node<T>
where
    Node<T>: Indexed<T>,
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
    Node<T>: Indexed<T>,
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self.arena().get(self.index), other.arena().get(other.index)) {
            (Some(a), Some(b)) => *a == *b,
            _ => false,
        }
    }
}

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
}

impl Context {
    fn new() -> Self {
        Default::default()
    }
}
