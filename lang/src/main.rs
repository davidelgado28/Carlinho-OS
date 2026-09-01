pub mod lexer;
pub mod ast;
pub mod parser;
pub mod codegen;
use lexer::Lexer;
use parser::Parser;
use codegen::CodeGenerator;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Uso: carlinhos-lang <arquivo.car>");
        return;
    }
    let filename = &args[1];
    let source_code = match fs::read_to_string(filename) {
        Ok(code) => code,
        Err(e) => {
            eprintln!("Erro ao ler o arquivo {}: {}", filename, e);
            return;
        }
    };
    let mut lexer = Lexer::new(source_code);
    let mut tokens = Vec::new();
    loop {
        let tok = lexer.next_token();
        if tok == lexer::Token::Eof {
            tokens.push(tok);
            break;
        }
        tokens.push(tok);
    }
    let mut parser = Parser::new(tokens);
    let mut statements = Vec::new();
    while let Some(stmt) = parser.parse_statement() {
        statements.push(stmt);
    }
    let mut codegen = CodeGenerator::new();
    let cpp_code = codegen.generate(&statements);
    let output_filename = "output.cpp";
    fs::write(output_filename, cpp_code).expect("Falha ao escrever o arquivo C++ gerado");
    println!("Compilação bem-sucedida! Código C++ gerado em '{}'.", output_filename);
}
