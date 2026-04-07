// Arquivo principal responsável pela pipeline do projeto:
// leitura do ".jack" -> análise léxica -> geração do ".XML" -> validação.

use std::env;
use std::fs;
use std::path::Path;

mod token;
mod tokenizer;

use tokenizer::Tokenizer;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = if args.len() < 2 {
        println!("Nenhum caminho informado, usando padrão: testes/Square/Inputs");
        Path::new("tests/Square/Inputs")
    } else {
        Path::new(&args[1])
    };

    if input_path.is_dir() {
        process_directory(input_path);
    } else {
        process_file(input_path);
    }
}

fn process_directory(dir: &Path) {
    let entries = fs::read_dir(dir).expect("Erro ao ler diretório");

    for entry in entries {
        let entry = entry.expect("Erro ao acessar arquivo");
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("jack") {
            process_file(&path);
        }
    }
}

fn process_file(path: &Path) {
    let code = fs::read_to_string(path)
        .expect("Erro ao ler arquivo .jack");

    let mut tokenizer = Tokenizer::new(code);
    let tokens = tokenizer.tokenize();

    let output = generate_xml(tokens);

    // 🔹 Define diretório de saída
    let output_dir = path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("Outputs");

    // 🔹 Garante que a pasta Outputs existe
    fs::create_dir_all(&output_dir)
        .expect("Erro ao criar diretório Outputs");

    // 🔹 Nome do arquivo (Main, Square, etc.)
    let file_name = path.file_stem().unwrap().to_str().unwrap();

    let output_path = output_dir.join(format!("{}T.xml", file_name));

    fs::write(output_path, output)
        .expect("Erro ao escrever arquivo de saída");
}

fn generate_xml(tokens: &Vec<token::Token>) -> String {
    let mut result = String::new();

    result.push_str("<tokens>\n");

    for token in tokens {
        result.push_str(&token.to_xml());
        result.push('\n');
    }

    result.push_str("</tokens>\n");

    result
}