// Arquivo responsável por ler o XML gerado pelo analisador léxico.
// A saída do scanner possui uma tag <tokens> e, dentro dela, uma linha por token.

use std::fs;
use std::path::Path;

use crate::token::{Token, TokenType};

pub fn read_tokens_from_file(path: &Path) -> Vec<Token> {
    let content = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Erro ao ler arquivo de tokens XML: {}", path.display()));

    read_tokens_from_string(&content)
}

pub fn read_tokens_from_string(content: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for (index, line) in content.lines().enumerate() {
        let line = line.trim();

        if line.is_empty() || line == "<tokens>" || line == "</tokens>" {
            continue;
        }

        let token = parse_token_line(line, index + 1);
        tokens.push(token);
    }

    tokens
}

fn parse_token_line(line: &str, line_number: usize) -> Token {
    if !line.starts_with('<') {
        panic!(
            "Token XML inválido na linha {}: esperado tag inicial, encontrado '{}'",
            line_number, line
        );
    }

    let start_tag_end = line
        .find('>')
        .unwrap_or_else(|| panic!("Token XML inválido na linha {}: '{}'", line_number, line));

    let category = &line[1..start_tag_end];
    let close_tag = format!("</{}>", category);

    let close_tag_start = line
        .rfind(&close_tag)
        .unwrap_or_else(|| {
            panic!(
                "Tag de fechamento inválida na linha {}: esperado '{}'",
                line_number, close_tag
            )
        });

    let raw_lexeme = line[start_tag_end + 1..close_tag_start].trim();
    let lexeme = unescape_xml(raw_lexeme);

    let token_type = TokenType::from_xml_category(category, &lexeme)
        .unwrap_or_else(|| {
            panic!(
                "Categoria de token inválida '{}' na linha {}",
                category, line_number
            )
        });

    Token::new(token_type, lexeme, line_number)
}

fn unescape_xml(text: &str) -> String {
    text.replace("&quot;", "\"")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&amp;", "&")
}
