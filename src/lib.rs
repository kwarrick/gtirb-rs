use std::cell::{Ref, RefCell, RefMut};
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;

use generational_arena::{Arena, Index};
use uuid::Uuid;

mod ir;

use ir::IR;

struct Node<T> {
    index: Index,
    context: Rc<RefCell<Context>>,
    kind: PhantomData<T>,
}

struct NodeIterator<T> {
    index: usize,
    parent: Index,
    context: Rc<RefCell<Context>>,
    kind: PhantomData<T>,
}

struct Module {
    uuid: Uuid,
}

struct Section;
struct ByteInterval;
struct DataBlock;
struct CodeBlock;
struct ProxyBlock;
struct Symbol;
struct SymbolicExpression;

#[derive(Default)]
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
