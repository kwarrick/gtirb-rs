use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;

use generational_arena::{Arena, Index};
use uuid::Uuid;

mod ir;
mod module;

use ir::IR;
use module::Module;

#[derive(Debug, Clone)]
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
    fn get(&self, index: usize) -> (Option<Index>, PhantomData<T>);
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
where Node<T>: Borrow<T>, T: Unique
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

#[derive(Debug)]
struct Section;
#[derive(Debug)]
struct ByteInterval;
#[derive(Debug)]
struct DataBlock;
#[derive(Debug)]
struct CodeBlock;
#[derive(Debug)]
struct ProxyBlock;
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
    code_block: Arena<DataBlock>,
    data_block: Arena<CodeBlock>,
    proxy_block: Arena<ProxyBlock>,
    symbol: Arena<Symbol>,
    symbolic_expression: Arena<SymbolicExpression>,
}

impl Context {
    fn new() -> Self {
        Default::default()
    }

    fn add_module(&mut self, module: Module) -> Index {
        let uuid = module.uuid();
        let index = self.module.insert(module);
        self.uuid_map.insert(uuid, index);
        index
    }
}
