use crate::asm::{Address, IndirectAddress, Instruction, Register::*};
use crate::ast::{self, Expr, FunctionDefinition, Program, Statement, Type};
use crate::compiler::symbol_table::Symbol;
use crate::platform;
use std::collections::HashMap;

mod symbol_table;

pub struct Compiler<'src> {
    instructions: Vec<Instruction>,
    string_literals: Vec<String>,
    symbol_table: symbol_table::SymbolTable<'src>,
}

struct FunctionCtx<'src> {
    // the name of the variable as well as the offset of that var into
    // the stack frame
    local_variables: HashMap<&'src str, (i32, Symbol<'src>)>,
    stack_ptr_offset: i32,
}

impl<'src> FunctionCtx<'src> {
    fn new() -> Self {
        FunctionCtx {
            local_variables: Default::default(),
            stack_ptr_offset: 0,
        }
    }
    fn lookup(&self, name: &str) -> Address {
        assert!(self.local_variables.get(name).is_some());
        let (offset, ref symbol) = *self.local_variables.get(name).unwrap();
        let addr = IndirectAddress::offset(Box::new(Rbp.into()), offset);
        let addr = match symbol.type_of().stack_size() {
            4 => addr.dword(),
            8 => addr.qword(),
            n => panic!("Unknown stack size: {}", n),
        };
        addr.into()
    }
    fn register_local(&mut self, symbol: Symbol<'src>) {
        debug_assert!(self.local_variables.get(symbol.name()).is_none());
        self.stack_ptr_offset -= symbol.type_of().stack_size() as i32;
        self.local_variables
            .insert(symbol.name(), (self.stack_ptr_offset, symbol));
    }
    fn register_temp(&mut self) -> Address {
        self.stack_ptr_offset -= Type::Int.stack_size() as i32;
        IndirectAddress::offset(Box::new(Rbp.into()), self.stack_ptr_offset).into()
    }
}

impl<'src> Compiler<'src> {
    pub fn new() -> Self {
        Compiler {
            instructions: vec![],
            string_literals: vec![],
            symbol_table: Default::default(),
        }
    }

    pub fn string_literals(&self) -> &[String] {
        &self.string_literals
    }

    pub fn instructions(&self) -> &[Instruction] {
        &self.instructions
    }

    pub fn register_literal(&mut self, literal: String) -> String {
        self.string_literals.push(literal);
        return format!("LC{}", self.string_literals.len() - 1);
    }

    pub fn gen(&mut self, instruction: Instruction) -> &mut Self {
        self.instructions.push(instruction);
        self
    }
    pub fn gen_label(&mut self, label: String) -> &mut Self {
        self.gen(Instruction::Label(label))
    }
}

fn func_parameter_register(number: usize) -> Address {
    match number {
        0 => Edi.into(),
        1 => Esi.into(),
        _ => unimplemented!(),
    }
}

fn compile_expr(compiler: &mut Compiler, func_ctx: &mut FunctionCtx, expr: &Expr) -> Address {
    match expr {
        Expr::Number(val) => Address::Immediate(*val),
        Expr::Ident(ident) => func_ctx.lookup(ident),
        Expr::AddressOf(ident) => {
            let (offset, _) = func_ctx.local_variables.get(ident.as_str()).unwrap();
            compiler.gen(Instruction::Lea(
                Rax.into(),
                IndirectAddress::offset(Box::new(Rbp.into()), *offset).into(),
            ));
            Rax.into()
        }
        Expr::FunctionCall(call) => {
            for (i, arg) in call.arguments.iter().enumerate() {
                let addr = compile_expr(compiler, func_ctx, arg);
                let register = func_parameter_register(i);
                compiler.gen(Instruction::Mov(register, addr));
            }
            compiler.gen(Instruction::Call(call.name.to_string()));
            Eax.into()
        }
        Expr::Op(lhs, op, rhs) => {
            use ast::BinaryOp;
            match op {
                BinaryOp::Add => {
                    let lhs = compile_expr(compiler, func_ctx, lhs);
                    let rhs = compile_expr(compiler, func_ctx, rhs);
                    let temp = func_ctx.register_temp();
                    compiler
                        .gen(Instruction::Mov(Eax.into(), lhs))
                        .gen(Instruction::Add(Eax.into(), rhs))
                        .gen(Instruction::Mov(temp.clone(), Eax.into()));
                    temp
                }
                _ => unimplemented!(),
            }
        }
        Expr::Dereference(expr) => {
            let addr = compile_expr(compiler, func_ctx, expr);
            compiler.gen(Instruction::Mov(Rax.into(), addr));
            IndirectAddress::indirect(Box::new(Rax.into()))
                .dword()
                .into()
        }
        Expr::Assignment { lhs, op, value } => {
            assert_eq!(op.clone(), ast::AssignmentOp::Assign);
            let value = compile_expr(compiler, func_ctx, value);
            let lhs = compile_expr(compiler, func_ctx, lhs);
            compiler.gen(Instruction::Mov(lhs.clone(), value));
            lhs
        }
        Expr::RawString(literal) => {
            let label = compiler.register_literal(literal.clone());
            IndirectAddress::indirect(Address::Label(label).into())
                .is_rip_relative(true)
                .into()
        }
        other => {
            eprintln!("Not implemented: {:?}", other);
            unimplemented!()
        }
    }
}

fn compile_statement<'src>(
    compiler: &mut Compiler<'src>,
    func_ctx: &mut FunctionCtx<'src>,
    stmt: &'src Statement,
) {
    match stmt {
        Statement::Return(expr) => {
            let ret_address = compile_expr(compiler, func_ctx, expr);
            compiler.gen(Instruction::Mov(Eax.into(), ret_address));
        }
        Statement::VariableDefinition { ty, name, value } => {
            let value = compile_expr(compiler, func_ctx, value);
            let symbol = Symbol::new(name, ty.clone());
            func_ctx.register_local(symbol);
            compiler.gen(Instruction::Mov(func_ctx.lookup(name), value));
        }
        Statement::Expr(expr) => {
            compile_expr(compiler, func_ctx, expr);
        }
    }
}

fn compile_func<'src>(compiler: &mut Compiler<'src>, func: &'src FunctionDefinition) {
    let body = match func.body {
        Some(ref statements) => statements,
        None => return,
    };
    let name = match func.name.as_str() {
        "main" => platform::main_symbol().to_string(),
        name => name.to_string(),
    };
    let symbol = Symbol::new(func.name.as_str(), func.type_of());
    compiler.symbol_table.insert_symbol(symbol);
    compiler.symbol_table.push_scope();
    let mut func_ctx = FunctionCtx::new();

    compiler
        // name the function
        .gen_label(name)
        // save stack pointer
        .gen(Instruction::Push(Rbp))
        // set frame pointer to stack pointer (so we can alloc stack space)
        .gen(Instruction::Mov(Rbp.into(), Rsp.into()));

    for (i, param) in func.parameters.iter().enumerate() {
        let symbol = Symbol::new(param.name.as_ref(), param.ty.clone());
        func_ctx.register_local(symbol.clone());
        let register = func_parameter_register(i);
        compiler.gen(Instruction::Mov(func_ctx.lookup(symbol.name()), register));
    }
    for stmt in body.iter() {
        compile_statement(compiler, &mut func_ctx, &stmt);
    }
    compiler.gen(Instruction::Pop(Rbp)).gen(Instruction::Ret);
    compiler.symbol_table.pop_scope();
}

pub fn compile(program: &Program) -> Compiler {
    let mut compiler = Compiler::new();
    compiler.symbol_table.push_scope();
    for func in program.functions.iter() {
        compile_func(&mut compiler, func);
    }
    compiler.symbol_table.pop_scope();

    compiler
}
