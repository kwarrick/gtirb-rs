use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;

use generational_arena::{Arena, Index};
use uuid::Uuid;

mod ir;

use ir::IR;

#[derive(Debug, Clone)]
struct Node<T> {
    index: Index,
    context: Rc<RefCell<Context>>,
    kind: PhantomData<T>,
}

trait Container<T> {
    fn get(&self, index: usize) -> (Option<Index>, PhantomData<T>);
}

struct NodeIterator<T,U> {
    index: usize,
    parent: Node<T>,
    kind: PhantomData<U>,
}

impl<T,U> Iterator for NodeIterator<T,U> 
where Node<T>: Container<U>
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

#[derive(Debug)]
struct Module {
    uuid: Uuid,
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
    uuid_map: HashMap<Uuid,Index>,

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
        let uuid = module.uuid.clone();
        let index = self.module.insert(module);
        self.uuid_map.insert(uuid, index);
        index
    }
}
