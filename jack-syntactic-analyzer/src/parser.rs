// Arquivo central do analisador sintático.
// A estrutura Parser será responsável por consumir tokens e gerar a árvore XML.

use crate::token::{Keyword, Token, TokenType};
use crate::xml_writer::XmlWriter;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    writer: XmlWriter,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            writer: XmlWriter::new(),
        }
    }

    // Ponto de entrada do parser. Nas próximas etapas, este método chamará
    // compile_class(), que representa a regra inicial da gramática Jack.
    pub fn parse(&mut self) -> String {
        self.writer.content().to_string()
    }

    pub fn token_count(&self) -> usize {
        self.tokens.len()
    }

    pub fn current_position(&self) -> usize {
        self.current
    }

    pub fn current_indent(&self) -> usize {
        self.writer.current_indent()
    }

    // Regra inicial da gramática Jack:
    // class: 'class' className '{' classVarDec* subroutineDec* '}'
    fn compile_class(&mut self) {
        self.writer.open_tag("class");

        self.consume_keyword_and_write(Keyword::Class);
        self.consume_identifier_and_write();
        self.consume_symbol_and_write('{');

        while self.check_keyword(Keyword::Static) || self.check_keyword(Keyword::Field) {
            self.compile_class_var_dec();
        }

        while self.is_subroutine_dec() {
            self.compile_subroutine_dec();
        }

        self.consume_symbol_and_write('}');
        self.writer.close_tag("class");
    }

    // classVarDec: ('static' | 'field') type varName (',' varName)* ';'
    fn compile_class_var_dec(&mut self) {
        self.writer.open_tag("classVarDec");

        if self.check_keyword(Keyword::Static) {
            self.consume_keyword_and_write(Keyword::Static);
        } else {
            self.consume_keyword_and_write(Keyword::Field);
        }

        self.compile_type();
        self.consume_identifier_and_write();

        while self.match_symbol_and_write(',') {
            self.consume_identifier_and_write();
        }

        self.consume_symbol_and_write(';');
        self.writer.close_tag("classVarDec");
    }

    // subroutineDec:
    // ('constructor' | 'function' | 'method') ('void' | type)
    // subroutineName '(' parameterList ')' subroutineBody
    fn compile_subroutine_dec(&mut self) {
        self.writer.open_tag("subroutineDec");

        if self.check_keyword(Keyword::Constructor) {
            self.consume_keyword_and_write(Keyword::Constructor);
        } else if self.check_keyword(Keyword::Function) {
            self.consume_keyword_and_write(Keyword::Function);
        } else {
            self.consume_keyword_and_write(Keyword::Method);
        }

        self.compile_return_type();
        self.consume_identifier_and_write();
        self.consume_symbol_and_write('(');
        self.compile_parameter_list();
        self.consume_symbol_and_write(')');
        self.compile_subroutine_body();

        self.writer.close_tag("subroutineDec");
    }

    fn compile_return_type(&mut self) {
        if self.check_keyword(Keyword::Void) {
            self.consume_keyword_and_write(Keyword::Void);
        } else {
            self.compile_type();
        }
    }

    // Esta função será completada na próxima etapa.
    fn compile_parameter_list(&mut self) {
        self.writer.open_tag("parameterList");
        self.writer.close_tag("parameterList");
    }

    // Esta função será completada com varDec e statements na próxima etapa.
    fn compile_subroutine_body(&mut self) {
        self.writer.open_tag("subroutineBody");
        self.consume_symbol_and_write('{');
        self.consume_symbol_and_write('}');
        self.writer.close_tag("subroutineBody");
    }

    // type: 'int' | 'char' | 'boolean' | className
    fn compile_type(&mut self) {
        if self.check_keyword(Keyword::Int) {
            self.consume_keyword_and_write(Keyword::Int);
        } else if self.check_keyword(Keyword::Char) {
            self.consume_keyword_and_write(Keyword::Char);
        } else if self.check_keyword(Keyword::Boolean) {
            self.consume_keyword_and_write(Keyword::Boolean);
        } else {
            self.consume_identifier_and_write();
        }
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

    fn is_subroutine_dec(&self) -> bool {
        self.check_keyword(Keyword::Constructor)
            || self.check_keyword(Keyword::Function)
            || self.check_keyword(Keyword::Method)
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

    fn match_symbol_and_write(&mut self, symbol: char) -> bool {
        if self.check_symbol(symbol) {
            let token = self.advance().unwrap();
            self.writer.write_token(&token);
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

    fn consume_keyword_and_write(&mut self, keyword: Keyword) {
        let token = self.consume_keyword(keyword);
        self.writer.write_token(&token);
    }

    fn consume_symbol_and_write(&mut self, symbol: char) {
        let token = self.consume_symbol(symbol);
        self.writer.write_token(&token);
    }

    fn consume_identifier_and_write(&mut self) {
        let token = self.consume_identifier();
        self.writer.write_token(&token);
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
