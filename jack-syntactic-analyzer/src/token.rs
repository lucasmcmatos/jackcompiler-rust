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

    pub fn as_str(&self) -> &str {
        match self {
            Keyword::Class => "class",
            Keyword::Constructor => "constructor",
            Keyword::Function => "function",
            Keyword::Method => "method",
            Keyword::Field => "field",
            Keyword::Static => "static",
            Keyword::Var => "var",
            Keyword::Int => "int",
            Keyword::Char => "char",
            Keyword::Boolean => "boolean",
            Keyword::Void => "void",
            Keyword::True => "true",
            Keyword::False => "false",
            Keyword::Null => "null",
            Keyword::This => "this",
            Keyword::Let => "let",
            Keyword::Do => "do",
            Keyword::If => "if",
            Keyword::Else => "else",
            Keyword::While => "while",
            Keyword::Return => "return",
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

    pub fn description(&self) -> String {
        match self {
            TokenType::Keyword(keyword) => format!("keyword '{}'", keyword.as_str()),
            TokenType::Symbol(symbol) => format!("symbol '{}'", symbol),
            TokenType::Identifier => "identifier".to_string(),
            TokenType::IntegerConstant => "integer constant".to_string(),
            TokenType::StringConstant => "string constant".to_string(),
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

    pub fn description(&self) -> String {
        format!(
            "{} com valor '{}'",
            self.token_type.description(),
            self.lexeme
        )
    }
}
