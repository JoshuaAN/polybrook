use std::collections::HashMap;

use index_vec::IndexVec;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum DType {
    Void,
    I32,
    Bool,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum BinaryOp {
    Add,
    Mul,
    Max,
    Div,
    Mod,
    Xor,
    Shl,
    Cmplt,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Constant {
    I32(i32),
    Bool(bool),
}

index_vec::define_index_type! { pub struct Value = u32; }

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ValueData {
    Const(Constant),
    Param { slot: u32, dtype: DType, shape: Value },
    Stack(Vec<Value>),
    Binary { op: BinaryOp, a: Value, b: Value }, // a op b
    Where { cond: Value, a: Value, b: Value },   // cond ? a : b
    Cast { src: Value, dtype: DType },
    Call { f: Value, args: Vec<Value> },
    Index { src: Value, indices: Vec<Value> },
    Assert { actual: Value, expected: Value },
}

pub struct Program<T = ()> {
    /// Expression graph nodes.
    nodes: IndexVec<Value, ValueData>,

    /// Node interning table to hash cons added nodes.
    interned: HashMap<ValueData, Value>,

    /// Annotations per node.
    ann: IndexVec<Value, T>,
}

impl<T> Program<T> {
    pub fn intern(&mut self, data: ValueData) -> Value {
        if let Some(&v) = self.interned.get(&data) {
            return v;
        }
        let v = self.nodes.push(data.clone());
        self.interned.insert(data, v);
        v
    }

    pub fn get(&self, v: Value) -> &ValueData {
        &self.nodes[v]
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (Value, &ValueData)> {
        self.nodes.iter_enumerated()
    }

    fn reannotate<U>(self, ann: IndexVec<Value, U>) -> Program<U> {
        assert_eq!(self.ann.len(), ann.len());
        Program { nodes: self.nodes, interned: self.interned, ann: ann }
    }
}
