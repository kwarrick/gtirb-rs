use crate::*;

#[derive(Default, Debug, PartialEq)]
pub struct SymAddrAddr {
    pub scale: i64,
    pub offset: i64,
    pub symbol1: Uuid,
    pub symbol2: Uuid,
}

#[derive(Default, Debug, PartialEq)]
pub struct SymAddrConst {
    pub offset: i64,
    pub symbol: Uuid,
}

#[derive(Default, Debug, PartialEq)]
pub struct SymStackConst {
    pub offset: i64,
    pub symbol: Uuid,
}

#[derive(Debug, PartialEq)]
pub enum SymbolicExpression {
    SymAddrAddr(SymAddrAddr),
    SymAddrConst(SymAddrConst),
    SymStackConst(SymStackConst),
}
