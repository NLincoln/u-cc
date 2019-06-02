#[macro_use]
extern crate lalrpop_util;
mod asm;
mod ast;
mod compiler;
mod platform;
lalrpop_mod!(pub c);

use clap::{App, Arg};
use std::fs;

fn main() {
    let matches = App::new("u-cc")
        .arg(Arg::with_name("input").takes_value(true).required(true))
        .get_matches();

    let input_str = fs::read_to_string(matches.value_of_os("input").unwrap())
        .expect("Failed to open input file");

    let ast = match c::ProgramParser::new().parse(&input_str) {
        Ok(program) => program,
        Err(err) => {
            panic!("{}", err);
        }
    };

    let output = compiler::compile(&ast);
    println!("global {}", platform::main_symbol());
    println!("extern _printf");
    println!("section .data");
    for (i, literal) in output.string_literals().iter().enumerate() {
        println!("LC{}: db \"{}\"", i, literal);
    }
    println!("section .text");
    for instruction in output.instructions().iter() {
        println!("{}", instruction);
    }
}
