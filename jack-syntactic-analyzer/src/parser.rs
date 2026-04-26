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

    // Ponto de entrada do parser.
    // A regra inicial da linguagem Jack sempre é uma classe.
    pub fn parse(&mut self) -> String {
        self.compile_class();

        if !self.is_at_end() {
            self.syntax_error("fim do arquivo");
        }

        self.writer.content().to_string()
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

    // parameterList: ((type varName) (',' type varName)*)?
    fn compile_parameter_list(&mut self) {
        self.writer.open_tag("parameterList");

        if !self.check_symbol(')') {
            self.compile_type();
            self.consume_identifier_and_write();

            while self.match_symbol_and_write(',') {
                self.compile_type();
                self.consume_identifier_and_write();
            }
        }

        self.writer.close_tag("parameterList");
    }

    // subroutineBody: '{' varDec* statements '}'
    fn compile_subroutine_body(&mut self) {
        self.writer.open_tag("subroutineBody");
        self.consume_symbol_and_write('{');

        while self.check_keyword(Keyword::Var) {
            self.compile_var_dec();
        }

        self.compile_statements();

        self.consume_symbol_and_write('}');
        self.writer.close_tag("subroutineBody");
    }

    // varDec: 'var' type varName (',' varName)* ';'
    fn compile_var_dec(&mut self) {
        self.writer.open_tag("varDec");

        self.consume_keyword_and_write(Keyword::Var);
        self.compile_type();
        self.consume_identifier_and_write();

        while self.match_symbol_and_write(',') {
            self.consume_identifier_and_write();
        }

        self.consume_symbol_and_write(';');
        self.writer.close_tag("varDec");
    }

    // statements: statement*
    fn compile_statements(&mut self) {
        self.writer.open_tag("statements");

        while self.is_statement() {
            self.compile_statement();
        }

        self.writer.close_tag("statements");
    }

    fn compile_statement(&mut self) {
        if self.check_keyword(Keyword::Let) {
            self.compile_let();
        } else if self.check_keyword(Keyword::If) {
            self.compile_if();
        } else if self.check_keyword(Keyword::While) {
            self.compile_while();
        } else if self.check_keyword(Keyword::Do) {
            self.compile_do();
        } else if self.check_keyword(Keyword::Return) {
            self.compile_return();
        } else {
            self.syntax_error("statement");
        }
    }

    // letStatement: 'let' varName ('[' expression ']')? '=' expression ';'
    fn compile_let(&mut self) {
        self.writer.open_tag("letStatement");

        self.consume_keyword_and_write(Keyword::Let);
        self.consume_identifier_and_write();

        if self.match_symbol_and_write('[') {
            self.compile_expression();
            self.consume_symbol_and_write(']');
        }

        self.consume_symbol_and_write('=');
        self.compile_expression();
        self.consume_symbol_and_write(';');

        self.writer.close_tag("letStatement");
    }

    fn compile_if(&mut self) {
        self.writer.open_tag("ifStatement");

        self.consume_keyword_and_write(Keyword::If);
        self.consume_symbol_and_write('(');
        self.compile_expression();
        self.consume_symbol_and_write(')');
        self.consume_symbol_and_write('{');
        self.compile_statements();
        self.consume_symbol_and_write('}');

        if self.check_keyword(Keyword::Else) {
            self.consume_keyword_and_write(Keyword::Else);
            self.consume_symbol_and_write('{');
            self.compile_statements();
            self.consume_symbol_and_write('}');
        }

        self.writer.close_tag("ifStatement");
    }

    // whileStatement: 'while' '(' expression ')' '{' statements '}'
    fn compile_while(&mut self) {
        self.writer.open_tag("whileStatement");

        self.consume_keyword_and_write(Keyword::While);
        self.consume_symbol_and_write('(');
        self.compile_expression();
        self.consume_symbol_and_write(')');
        self.consume_symbol_and_write('{');
        self.compile_statements();
        self.consume_symbol_and_write('}');

        self.writer.close_tag("whileStatement");
    }

    // doStatement: 'do' subroutineCall ';'
    fn compile_do(&mut self) {
        self.writer.open_tag("doStatement");

        self.consume_keyword_and_write(Keyword::Do);
        self.compile_subroutine_call();
        self.consume_symbol_and_write(';');

        self.writer.close_tag("doStatement");
    }

    // returnStatement: 'return' expression? ';'
    fn compile_return(&mut self) {
        self.writer.open_tag("returnStatement");

        self.consume_keyword_and_write(Keyword::Return);

        if !self.check_symbol(';') {
            self.compile_expression();
        }

        self.consume_symbol_and_write(';');

        self.writer.close_tag("returnStatement");
    }

    fn compile_subroutine_call(&mut self) {
        self.consume_identifier_and_write();

        if self.match_symbol_and_write('.') {
            self.consume_identifier_and_write();
        }

        self.consume_symbol_and_write('(');
        self.compile_expression_list();
        self.consume_symbol_and_write(')');
    }

    // expression: term (op term)*
    fn compile_expression(&mut self) {
        self.writer.open_tag("expression");

        self.compile_term();

        while self.is_op() {
            let token = self.consume_current("operator");
            self.writer.write_token(&token);
            self.compile_term();
        }

        self.writer.close_tag("expression");
    }

    // expressionList: (expression (',' expression)*)?
    fn compile_expression_list(&mut self) {
        self.writer.open_tag("expressionList");

        if !self.check_symbol(')') {
            self.compile_expression();

            while self.match_symbol_and_write(',') {
                self.compile_expression();
            }
        }

        self.writer.close_tag("expressionList");
    }

    // term cobre constantes, variáveis, chamadas, expressões entre parênteses e operadores unários.
    fn compile_term(&mut self) {
        self.writer.open_tag("term");

        if self.check_token_type(&TokenType::IntegerConstant) {
            self.consume_token_type_and_write(TokenType::IntegerConstant);
        } else if self.check_token_type(&TokenType::StringConstant) {
            self.consume_token_type_and_write(TokenType::StringConstant);
        } else if self.is_keyword_constant() {
            self.consume_current_and_write();
        } else if self.check_symbol('(') {
            self.consume_symbol_and_write('(');
            self.compile_expression();
            self.consume_symbol_and_write(')');
        } else if self.is_unary_op() {
            self.consume_current_and_write();
            self.compile_term();
        } else if self.check_identifier() {
            if self.check_next_symbol('[') {
                self.consume_identifier_and_write();
                self.consume_symbol_and_write('[');
                self.compile_expression();
                self.consume_symbol_and_write(']');
            } else if self.check_next_symbol('(') || self.check_next_symbol('.') {
                self.compile_subroutine_call();
            } else {
                self.consume_identifier_and_write();
            }
        } else {
            self.syntax_error("term");
        }

        self.writer.close_tag("term");
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

    fn check_next_symbol(&self, symbol: char) -> bool {
        match self.tokens.get(self.current + 1) {
            Some(token) => token.token_type == TokenType::Symbol(symbol),
            None => false,
        }
    }

    fn check_identifier(&self) -> bool {
        self.check_token_type(&TokenType::Identifier)
    }

    fn is_subroutine_dec(&self) -> bool {
        self.check_keyword(Keyword::Constructor)
            || self.check_keyword(Keyword::Function)
            || self.check_keyword(Keyword::Method)
    }

    fn is_statement(&self) -> bool {
        self.check_keyword(Keyword::Let)
            || self.check_keyword(Keyword::If)
            || self.check_keyword(Keyword::While)
            || self.check_keyword(Keyword::Do)
            || self.check_keyword(Keyword::Return)
    }

    fn is_op(&self) -> bool {
        match self.peek() {
            Some(token) => matches!(
                token.token_type,
                TokenType::Symbol('+')
                    | TokenType::Symbol('-')
                    | TokenType::Symbol('*')
                    | TokenType::Symbol('/')
                    | TokenType::Symbol('&')
                    | TokenType::Symbol('|')
                    | TokenType::Symbol('<')
                    | TokenType::Symbol('>')
                    | TokenType::Symbol('=')
            ),
            None => false,
        }
    }

    fn is_unary_op(&self) -> bool {
        self.check_symbol('-') || self.check_symbol('~')
    }

    fn is_keyword_constant(&self) -> bool {
        self.check_keyword(Keyword::True)
            || self.check_keyword(Keyword::False)
            || self.check_keyword(Keyword::Null)
            || self.check_keyword(Keyword::This)
    }

    fn match_symbol_and_write(&mut self, symbol: char) -> bool {
        if self.check_symbol(symbol) {
            let token = self.consume_current(&format!("symbol '{}'", symbol));
            self.writer.write_token(&token);
            return true;
        }

        false
    }

    fn consume_keyword(&mut self, keyword: Keyword) -> Token {
        if self.check_keyword(keyword.clone()) {
            return self.consume_current(&format!("keyword '{}'", keyword.as_str()));
        }

        self.syntax_error(&format!("keyword '{}'", keyword.as_str()))
    }

    fn consume_symbol(&mut self, symbol: char) -> Token {
        if self.check_symbol(symbol) {
            return self.consume_current(&format!("symbol '{}'", symbol));
        }

        self.syntax_error(&format!("symbol '{}'", symbol))
    }

    fn consume_identifier(&mut self) -> Token {
        if self.check_identifier() {
            return self.consume_current("identifier");
        }

        self.syntax_error("identifier")
    }

    fn consume_token_type_and_write(&mut self, token_type: TokenType) {
        if self.check_token_type(&token_type) {
            self.consume_current_and_write();
            return;
        }

        self.syntax_error(token_type.xml_category())
    }

    fn consume_current_and_write(&mut self) {
        let token = self.consume_current("token");
        self.writer.write_token(&token);
    }

    fn consume_current(&mut self, expected: &str) -> Token {
        self.advance()
            .unwrap_or_else(|| self.syntax_error(expected))
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
                "Erro sintático na linha {}: esperado {}, encontrado {}.",
                token.line,
                expected,
                token.description()
            ),
            None => panic!(
                "Erro sintático no fim do arquivo: esperado {}",
                expected
            ),
        }
    }
}
