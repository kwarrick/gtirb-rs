use std::marker::PhantomData;
use std::sync::{Arc, RwLock};

use generational_arena::{Arena, Index};
use uuid::Uuid;

mod ir;

use ir::IR;

struct Node<T> {
    index: Index,
    context: Arc<RwLock<Context>>,
    kind: PhantomData<T>,
}

struct NodeIterator<T> {
    index: usize, // XXX: Very bad idea, what if the buffer is modified by a concurrent thread?
    parent: Index,
    context: Arc<RwLock<Context>>,
    kind: PhantomData<T>,
}

struct Module;
struct Section;
struct ByteInterval;
struct DataBlock;
struct CodeBlock;
struct ProxyBlock;
struct Symbol;
struct SymbolicExpression;

#[derive(Default)]
struct Context {
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
}
