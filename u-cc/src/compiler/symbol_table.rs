use crate::ast::Type;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Symbol<'src> {
    name: &'src str,
    type_of: Type,
}

impl<'src> Symbol<'src> {
    pub fn new(name: &'src str, type_of: Type) -> Self {
        Symbol { name, type_of }
    }
    pub fn name(&self) -> &'src str {
        self.name
    }
    pub fn type_of(&self) -> &Type {
        &self.type_of
    }
}

#[derive(Debug, Default)]
pub struct SymbolTable<'src> {
    inner: Vec<HashMap<&'src str, Symbol<'src>>>,
}

impl<'src> SymbolTable<'src> {
    // pub fn lookup_symbol(&self, name: &str) -> Option<&Symbol<'src>> {
    //     for idx in 0..self.inner.len() {
    //         let idx = self.inner.len() - idx - 1;
    //         let table = &self.inner[idx];
    //         if let Some(symbol) = table.get(name) {
    //             return Some(symbol);
    //         }
    //     }
    //     return None;
    // }
    pub fn insert_symbol(&mut self, symbol: Symbol<'src>) {
        self.inner.last_mut().unwrap().insert(symbol.name, symbol);
    }
    pub fn push_scope(&mut self) {
        self.inner.push(Default::default());
    }
    pub fn pop_scope(&mut self) {
        self.inner.pop();
    }
}
