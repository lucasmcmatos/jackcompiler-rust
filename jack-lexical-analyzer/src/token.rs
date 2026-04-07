// Arquivo estrutural do projeto, responsável por definir a estrutura e tipagem
// dos tokens.

// Substituição das estruturas usadas no código python exemplo
//  - Enum (python) -> enum (rust): O rust tem uma estrutura nativa que realiza 
//    a função de "representar um conjunto fixo de valores";
//  - auto() (python): Na estrutura enum do rust nunva trabalharemos com valores
//    numéricos internos, dessa forma, não a necessidade de usar uma estrutura 
//    semelhante a função no python;
//  - dataclass (python) -> struct + derive (rust): Para replicar o comportamento
//    da estrutura em python podemos usar as duas estruturas para o projeto.

// Estruturas usadas:
// - derive: macro para criação de padrões com interfaces definidas assim como os
//   dataclass em python. É necessário ser setado com as configurações no caso foi
//   usado "Debug" ( permite impressão em console com formatadores), "Clone" 
//   ( permite copia das instâcias ) e "PartialEq" (permite comparação com operadores
//   como ==, !=...)

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

#[derive(Debug, Clone)]
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

    pub fn to_xml(&self) -> String {
        let category = self.get_category();
        let value = self.escape_xml(&self.lexeme);

        format!("<{}> {} </{}>", category, value, category)
    }

    fn get_category(&self) -> &str {
        match &self.token_type {
            TokenType::Keyword(_) => "keyword",
            TokenType::Symbol(_) => "symbol",
            TokenType::Identifier => "identifier",
            TokenType::IntegerConstant => "integerConstant",
            TokenType::StringConstant => "stringConstant",
        }
    }

    fn escape_xml(&self, text: &str) -> String {
        let mut result = text.to_string();

        if matches!(self.token_type, TokenType::StringConstant) {
            result = result.trim_matches('"').to_string();
        }

        result = result.replace('&',"&amp;");
        result = result.replace('<',"&lt;");
        result = result.replace('>',"&gt;");

        result
    }
}
