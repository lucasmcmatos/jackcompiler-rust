// Arquivo "core" do projeto, realiza o scan do código ".jack". Navega, controla fluxo, 
// ignora comentários/espaços e lê os tokens.

use create::token::{Keyword, Token, TokenType};

pub struct Tokenizer {
    code: String,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new(code: String) -> Self {
        Self {
            code,
            current: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.code.len()
    }
    
    // Em rust não tem como passar um valor padrão de entrada.
    fn peek(&self, offset: usize) -> char {
        let pos = self.current + offset;

        match self.code.chars().nth(pos) {
            Some(c) => c,
            None => '\0'
        }
    }

    fn advance(&mut self) -> char {
        let ch = self.peek(0);

        if ch != '\0' {
            self.current += 1;
        }

        if ch == '\n' {
            self.line += 1;
        }

        ch
    }

    fn skip_whitespace() {

    }

    fn skip_line_comment() {

    }

    fn skip_block_comment() {

    }

    fn is_symbol(c: char) -> bool {

    }

    fn read_number(&mut self) -> Token {

    }

    fn read_identifier(&mut self) -> Token {

    }

    fn read_string(&mut self) -> Token {

    }

    
}