use std::path::Path;

#[allow(dead_code)]
mod token;
mod token_reader;

fn main() {
    let token_path = Path::new("../square-tests/output/MainT.xml");
    let tokens = token_reader::read_tokens_from_file(token_path);

    println!("Tokens carregados: {}", tokens.len());
}
