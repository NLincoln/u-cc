#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub functions: Vec<FunctionDefinition>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionParameter {
    pub ty: Type,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinition {
    pub return_type: Type,
    pub name: String,
    pub parameters: Vec<FunctionParameter>,
    pub body: Vec<Statement>,
}

impl FunctionDefinition {
    pub fn type_of(&self) -> Type {
        Type::Function {
            return_type: Box::new(self.return_type.clone()),
            arguments: self.parameters.iter().map(|arg| arg.ty.clone()).collect(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Function {
        return_type: Box<Type>,
        arguments: Vec<Type>,
    },
}

impl Type {
    pub fn stack_size(&self) -> usize {
        use std::usize;
        match self {
            // ok I mean this is probably the worst way to do this but whatever.
            Type::Int => 4,
            Type::Function { .. } => 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Return(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(i32),
    Ident(String),
    FunctionCall(FunctionCall),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Box<Expr>>,
}
