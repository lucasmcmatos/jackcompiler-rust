// Arquivo "core" do projeto, realiza o scan do código ".jack". Navega, controla fluxo, 
// ignora comentários/espaços e lê os tokens.

use crate::token::{Keyword, Token, TokenType};

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

    pub fn tokenize(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.skip_whitespace();
            if self.is_at_end() {
                break;
            }

            let ch = self.peek(0);

            if ch == '/' {
                if self.peek(1) == '/' {
                    self.advance();
                    self.advance();
                    self.skip_line_comment();
                    continue;
                } else if self.peek(1) == '*' {
                    self.advance();
                    self.skip_block_comment();
                    continue;
                } else {
                    let lexeme = self.advance().to_string();
                    self.tokens.push(
                        Token::new(TokenType::Symbol('/'), lexeme, self.line)
                    );
                }
            }

            else if ch.is_ascii_alphabetic() || ch == '_' {
                let token = self.read_identifier();
                self.tokens.push(token);
            }

            else if ch.is_ascii_digit() {
                let token = self.read_number();
                self.tokens.push(token);
            }

            else if ch == '"' {
                let token = self.read_string();
                self.tokens.push(token);
            }

            else if Self::is_symbol(ch) {
                let lexeme = self.advance().to_string();
                self.tokens.push(Token::new(TokenType::Symbol(ch),lexeme, self.line));
            }
            
            else {
                panic!("Caractere ilegal '{}' na linha [{}]", ch, self.line);
            }
        }

        &self.tokens
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.code.len()
    }
    
    // Em rust não tem como passar um valor padrão de entrada.
    fn peek(&self, offset: usize) -> char {
        let pos = self.current + offset;

        if pos >= self.code.len() {
            return '\0';
        }

        self.code.as_bytes()[pos] as char
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

    fn skip_whitespace(&mut self) {
        while !self.is_at_end() {
            let c = self.peek(0);

            if c == ' ' || c == '\t' || c == '\r' || c == '\n' {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn skip_line_comment(&mut self) {
        while self.peek(0) != '\n' && !self.is_at_end() {
            self.advance();
        }
    }

    fn skip_block_comment(&mut self) {
        self.advance();

        while !self.is_at_end() {
            let c = self.peek(0);

            if c == '*' && self.peek(1) == '/' {
                self.advance();
                self.advance();
                return;
            }

            self.advance();
        }

        panic!("Comentário não fechado na linha: [{}]", self.line);
    }

    fn is_symbol(c: char) -> bool {
        matches!(
            c, 
            '(' | ')' | '{' | '}' | '[' | ']' |
            ',' | ';' | '.' | '+' | '-' | '*' |
            '/' | '&' | '|' | '<' | '>' | '=' | '~'
        )
    }

    fn read_number(&mut self) -> Token {
        let start = self.current;

        while self.peek(0).is_ascii_digit() {
            self.advance();
        }

        // Isso só é seguro se current representar posições válidas de byte. Como 
        // você está usando chars().nth(pos) em peek, mas incrementando current 
        // por 1, sua lógica mistura índice de caractere com índice de byte. Isso 
        // pode quebrar em Unicode.
        let lexeme = self.code[start..self.current].to_string();

        Token::new(TokenType::IntegerConstant, lexeme, self.line)
    }

    fn read_identifier(&mut self) -> Token {
        let start = self.current;

        while self.peek(0).is_ascii_alphanumeric() || self.peek(0) == '_' {
            self.advance();
        }

        let lexeme = self.code[start..self.current].to_string();
        
        if let Some(keyword) = Keyword::from_str(&lexeme) {
            Token::new(TokenType::Keyword(keyword), lexeme, self.line)
        } else {
            Token::new(TokenType::Identifier, lexeme, self.line)
        }
        
    }

    fn read_string(&mut self) -> Token {
        self.advance();

        let start = self.current;

        while self.peek(0) != '"' && !self.is_at_end() {
            if self.peek(0) == '\n' {
                panic!("Erro na linha [{}]! String constante não pode conter quebra de linha.", self.line);
            }

            self.advance();
        }

        if self.is_at_end() {
            panic!("Erro na linha [{}]! String não fechada (esperado '\"').", self.line);
        }

        let lexeme = self.code[start..self.current].to_string();

        self.advance();

        Token::new(TokenType::StringConstant, lexeme, self.line)
    }


}