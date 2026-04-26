use std::path::Path;

mod parser;
#[allow(dead_code)]
mod token;
mod token_reader;

fn main() {
    let token_path = Path::new("../square-tests/output/MainT.xml");
    let tokens = token_reader::read_tokens_from_file(token_path);
    let mut parser = parser::Parser::new(tokens);
    let output = parser.parse();

    println!("Tokens carregados: {}", parser.token_count());
    println!("Posição inicial do parser: {}", parser.current_position());
    println!("Indentação inicial do parser: {}", parser.current_indent());
    println!("Tamanho inicial da saída XML: {}", output.len());
}
