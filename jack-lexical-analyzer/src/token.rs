// Arquivo estrutural do projeto, responsável por definir a estrutura e tipagem
// dos tokens.

// Estruturas usadas no código:
//  - enum, impl e struct: rust diferente do python e java que podemos implementar as funções,
//    dados e estruturas dentro do "class Keyword". No rust temos uma estrutura para
//    cada uma dessas definições. enum -> possíveis tipos de dados, struct -> estrutura específica e
//    impl -> comportamentos (funções). 
//  - "derive": macro para criação de classes padrões, funciona como o "dataclass"
//    no python so que precisa definir especificamente as permições como no caso
//    "Debug" (permite impressão no console), "Clone" (permite instância da classe)
//    e "ParitalEq" (permite operadores de comparação entre as insTâncias).
//  - Option: Basicamente define um retorno dentro de alguma estrutura ja definida
//    no caso a estrura self refere-se ao Keyword.
//  - Some(): É a forma de retornar algum valor, no python isso é nativo basta 
//    basta retornar o valor. No rust tem que definir que vai retornar um valor,
//    basicamente esta dizendo "retorna um valor, este é o valor" (o contrário seria
//    apenas "None").
//  - match: essa estrutura basicamente faz uma verificação se alguma variável é 
//    igual ao valor definido. 

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