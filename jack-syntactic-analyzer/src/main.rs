use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[allow(dead_code)]
mod parser;
#[allow(dead_code)]
mod token;
mod token_reader;
#[allow(dead_code)]
mod xml_writer;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = if args.len() < 2 {
        println!("Nenhum caminho informado, usando padrão: square-tests/output");
        repository_root().join("square-tests/output")
    } else {
        PathBuf::from(&args[1])
    };

    println!("Executando analisador sintático...");

    if input_path.is_dir() {
        process_directory(&input_path);
    } else {
        process_file(&input_path);
    }

    println!("Arquivos sintáticos gerados! Verifique a pasta 'square-tests/output'.");
}

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("Erro ao encontrar a raiz do repositório")
        .to_path_buf()
}

fn process_directory(dir: &Path) {
    let entries = fs::read_dir(dir).expect("Erro ao ler diretório de tokens");

    for entry in entries {
        let entry = entry.expect("Erro ao acessar arquivo de tokens");
        let path = entry.path();

        if is_token_xml(&path) {
            process_file(&path);
        }
    }
}

fn process_file(token_path: &Path) {
    let tokens = token_reader::read_tokens_from_file(token_path);
    let mut parser = parser::Parser::new(tokens);
    let output = parser.parse();

    let output_path = parser_output_path(token_path);

    fs::write(&output_path, output)
        .expect("Erro ao escrever arquivo XML sintático");

    println!("Arquivo gerado: {}", output_path.display());
}

fn is_token_xml(path: &Path) -> bool {
    let is_xml = path.extension().and_then(|s| s.to_str()) == Some("xml");
    let is_token_file = path
        .file_stem()
        .and_then(|s| s.to_str())
        .map(|name| name.ends_with('T'))
        .unwrap_or(false);

    is_xml && is_token_file
}

fn parser_output_path(token_path: &Path) -> PathBuf {
    let file_stem = token_path
        .file_stem()
        .and_then(|s| s.to_str())
        .expect("Nome inválido para arquivo de tokens");

    let class_name = file_stem
        .strip_suffix('T')
        .expect("Arquivo de tokens deve terminar com T");

    token_path.with_file_name(format!("{}P.xml", class_name))
}
