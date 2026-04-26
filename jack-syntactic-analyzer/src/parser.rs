// Arquivo central do analisador sintático.
// A estrutura Parser será responsável por consumir tokens e gerar a árvore XML.

use crate::token::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    output: String,
    indent: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            output: String::new(),
            indent: 0,
        }
    }

    // Ponto de entrada do parser. Nas próximas etapas, este método chamará
    // compile_class(), que representa a regra inicial da gramática Jack.
    pub fn parse(&mut self) -> String {
        self.output.clone()
    }

    pub fn token_count(&self) -> usize {
        self.tokens.len()
    }

    pub fn current_position(&self) -> usize {
        self.current
    }

    pub fn current_indent(&self) -> usize {
        self.indent
    }
}
