mod proto {
    include!(concat!(env!("OUT_DIR"), "/proto.rs"));
}

pub use proto::FileFormat;
pub use proto::Isa as ISA;
pub use proto::SectionFlag;

mod addr;
pub use addr::Addr;

mod ir;
pub use crate::ir::IR;
pub use crate::ir::read;

mod module;
pub use crate::module::Module;

mod section;
pub use crate::section::Section;

mod byte_interval;
pub use crate::byte_interval::Block;
pub use crate::byte_interval::ByteInterval;

mod code_block;
pub use crate::code_block::CodeBlock;

mod data_block;
pub use crate::data_block::DataBlock;

mod proxy_block;
pub use crate::proxy_block::ProxyBlock;

mod symbol;
pub use crate::symbol::Symbol;

mod util;
