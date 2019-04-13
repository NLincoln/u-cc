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

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Return(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Number(i32),
    Ident(String),
    FunctionCall(FunctionCall),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Expr>,
}
