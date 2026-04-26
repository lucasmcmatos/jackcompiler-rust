// Arquivo estrutural do analisador sintático.
// Define como os tokens vindos do XML léxico serão representados dentro do parser.

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Var,
    Int,
    Char,
    Boolean,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
}

impl Keyword {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "class" => Some(Keyword::Class),
            "constructor" => Some(Keyword::Constructor),
            "function" => Some(Keyword::Function),
            "method" => Some(Keyword::Method),
            "field" => Some(Keyword::Field),
            "static" => Some(Keyword::Static),
            "var" => Some(Keyword::Var),
            "int" => Some(Keyword::Int),
            "char" => Some(Keyword::Char),
            "boolean" => Some(Keyword::Boolean),
            "void" => Some(Keyword::Void),
            "true" => Some(Keyword::True),
            "false" => Some(Keyword::False),
            "null" => Some(Keyword::Null),
            "this" => Some(Keyword::This),
            "let" => Some(Keyword::Let),
            "do" => Some(Keyword::Do),
            "if" => Some(Keyword::If),
            "else" => Some(Keyword::Else),
            "while" => Some(Keyword::While),
            "return" => Some(Keyword::Return),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Keyword(Keyword),
    Symbol(char),
    Identifier,
    IntegerConstant,
    StringConstant,
}

impl TokenType {
    // Converte a categoria do XML léxico para o tipo interno usado pelo parser.
    pub fn from_xml_category(category: &str, lexeme: &str) -> Option<Self> {
        match category {
            "keyword" => Keyword::from_str(lexeme).map(TokenType::Keyword),
            "symbol" => lexeme.chars().next().map(TokenType::Symbol),
            "identifier" => Some(TokenType::Identifier),
            "integerConstant" => Some(TokenType::IntegerConstant),
            "stringConstant" => Some(TokenType::StringConstant),
            _ => None,
        }
    }

    pub fn xml_category(&self) -> &str {
        match self {
            TokenType::Keyword(_) => "keyword",
            TokenType::Symbol(_) => "symbol",
            TokenType::Identifier => "identifier",
            TokenType::IntegerConstant => "integerConstant",
            TokenType::StringConstant => "stringConstant",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            token_type,
            lexeme,
            line,
        }
    }

    pub fn is_keyword(&self, keyword: Keyword) -> bool {
        self.token_type == TokenType::Keyword(keyword)
    }

    pub fn is_symbol(&self, symbol: char) -> bool {
        self.token_type == TokenType::Symbol(symbol)
    }

    pub fn is_identifier(&self) -> bool {
        self.token_type == TokenType::Identifier
    }
}
