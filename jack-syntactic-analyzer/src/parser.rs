// Arquivo central do analisador sintático.
// A estrutura Parser será responsável por consumir tokens e gerar a árvore XML.

use crate::token::{Keyword, Token, TokenType};

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

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.current)
    }

    fn previous(&self) -> Option<&Token> {
        if self.current == 0 {
            None
        } else {
            self.tokens.get(self.current - 1)
        }
    }

    fn advance(&mut self) -> Option<Token> {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous().cloned()
    }

    fn check_token_type(&self, token_type: &TokenType) -> bool {
        match self.peek() {
            Some(token) => &token.token_type == token_type,
            None => false,
        }
    }

    fn check_keyword(&self, keyword: Keyword) -> bool {
        self.check_token_type(&TokenType::Keyword(keyword))
    }

    fn check_symbol(&self, symbol: char) -> bool {
        self.check_token_type(&TokenType::Symbol(symbol))
    }

    fn check_identifier(&self) -> bool {
        self.check_token_type(&TokenType::Identifier)
    }

    fn match_keyword(&mut self, keyword: Keyword) -> bool {
        if self.check_keyword(keyword) {
            self.advance();
            return true;
        }

        false
    }

    fn match_symbol(&mut self, symbol: char) -> bool {
        if self.check_symbol(symbol) {
            self.advance();
            return true;
        }

        false
    }

    fn consume_keyword(&mut self, keyword: Keyword) -> Token {
        if self.check_keyword(keyword.clone()) {
            return self.advance().unwrap();
        }

        self.syntax_error(&format!("keyword {:?}", keyword))
    }

    fn consume_symbol(&mut self, symbol: char) -> Token {
        if self.check_symbol(symbol) {
            return self.advance().unwrap();
        }

        self.syntax_error(&format!("symbol '{}'", symbol))
    }

    fn consume_identifier(&mut self) -> Token {
        if self.check_identifier() {
            return self.advance().unwrap();
        }

        self.syntax_error("identifier")
    }

    fn syntax_error(&self, expected: &str) -> ! {
        match self.peek() {
            Some(token) => panic!(
                "Erro sintático na linha {}: esperado {}, encontrado '{}'",
                token.line, expected, token.lexeme
            ),
            None => panic!(
                "Erro sintático no fim do arquivo: esperado {}",
                expected
            ),
        }
    }
}
