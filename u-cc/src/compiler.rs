use crate::asm::{Address, Instruction, Register::*};
use crate::ast::{self, FunctionDefinition, Program, Statement};
use crate::platform;
struct Compiler {
    instructions: Vec<Instruction>,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            instructions: vec![],
        }
    }

    pub fn gen(&mut self, instruction: Instruction) -> &mut Self {
        self.instructions.push(instruction);
        self
    }
    pub fn gen_label(&mut self, label: String) -> &mut Self {
        self.gen(Instruction::Label(label))
    }
}

fn compile_statement(compiler: &mut Compiler, stmt: &Statement) {
    use ast::Expr;
    match stmt {
        Statement::Return(expr) => {
            let retval = match expr {
                Expr::Number(val) => *val,
            };
            compiler.gen(Instruction::Mov(Eax.into(), Address::Immediate(retval)));
        }
    }
}

fn compile_func(compiler: &mut Compiler, func: &FunctionDefinition) {
    let name = match func.name.as_str() {
        "main" => platform::main_symbol().to_string(),
        name => name.to_string(),
    };
    compiler
        .gen_label(name)
        .gen(Instruction::Push(Rbp))
        .gen(Instruction::Mov(Rbp.into(), Rsp.into()));
    for stmt in func.body.iter() {
        compile_statement(compiler, &stmt);
    }
    compiler.gen(Instruction::Pop(Rbp)).gen(Instruction::Ret);
}

pub fn compile(program: &Program) -> Vec<Instruction> {
    let mut compiler = Compiler::new();

    for func in program.functions.iter() {
        compile_func(&mut compiler, func);
    }

    compiler.instructions
}
