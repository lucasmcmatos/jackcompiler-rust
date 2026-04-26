// Arquivo responsável por montar a saída XML do analisador sintático.
// Ele centraliza indentação, abertura/fechamento de tags e escrita dos tokens.

use crate::token::Token;

pub struct XmlWriter {
    output: String,
    indent: usize,
}

impl XmlWriter {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent: 0,
        }
    }

    pub fn open_tag(&mut self, tag: &str) {
        self.write_indent();
        self.output.push_str(&format!("<{}>\n", tag));
        self.indent += 1;
    }

    pub fn close_tag(&mut self, tag: &str) {
        if self.indent > 0 {
            self.indent -= 1;
        }

        self.write_indent();
        self.output.push_str(&format!("</{}>\n", tag));
    }

    pub fn write_token(&mut self, token: &Token) {
        let category = token.token_type.xml_category();
        let lexeme = escape_xml(&token.lexeme);

        self.write_indent();
        self.output
            .push_str(&format!("<{}> {} </{}>\n", category, lexeme, category));
    }

    pub fn content(&self) -> &str {
        &self.output
    }

    pub fn current_indent(&self) -> usize {
        self.indent
    }

    fn write_indent(&mut self) {
        self.output.push_str(&"  ".repeat(self.indent));
    }
}

fn escape_xml(text: &str) -> String {
    text.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
