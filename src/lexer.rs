use tokens;

struct Lexer {
    contents: Vec<char>,
    ct: usize
}

impl Lexer {
    pub fn new(content: String) -> Self {
        Self {
            contents: content.chars().collect(),
            counter: 0
        }
    }

    fn cur_char(&self) -> char {
        let c = self.contents.get(self.counter);
        
        match c {
            Some(_value) => {
                *c.unwrap()
            }
            None => {
                '!'
            }
        }
    }

    pub fn lex(&mut self) {
        let mut token_list = Vec::new();

        while self.contents.len() > self.ct {
            let buf = self.get_tok();
            let tok = self.id_tok(buf);
            token_list.push(tok);
            self.adv();
        }
        return token_list;
    }

    fn get_tok(&mut self) {
        let buf = String::new();
        while self.cur_tok() != ' ' {
            buf.push(self.cur_tok());
            self.adv();
        }
        return buf;
    }

    fn id_tok(&mut self, tok) {
        let tok_bytes = tok.as_bytes();
        let mut new_tok: Token;
        match tok_bytes {
            _ if tok_bytes[0] == '\'' && tok_bytes[-1] == '\'' {
                new_tok = Token::new(TokenKind::String, tok);
            }

            _ => {
                new_tok = Token::new(TokenKind::Var, tok);
            }
        }
        return new_tok;
    }

    fn adv(&mut self) {
        self.ct += 1;
    }
}