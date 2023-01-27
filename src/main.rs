use std::{fs, env};

fn main() {
    let fp: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&fp[1]);

    let mut lexer = Lexer::new(contents.unwrap());
    let toks = lexer.lex();

    println!("{:?}", toks);
}

#[derive(Debug)]
enum TokenKind {
    String,
    Var
}

#[derive(Debug)]
struct Token {
    kind: TokenKind,
    literal: String
}

#[derive(Debug)]
struct Lexer {
    contents: Vec<char>,
    ct: usize
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) -> Self {
        Self {
            kind: kind,
            literal: literal
        }
    }
}

impl Lexer {
    pub fn new(content: String) -> Self {
        Self {
            contents: content.chars().collect(),
            ct: 0
        }
    }

    fn cur_char(&self) -> char {
        let c = self.contents.get(self.ct);
        
        match c {
            Some(_value) => {
                *c.unwrap()
            }
            None => {
                ' '
            }
        }
    }

    pub fn lex(&mut self) -> Vec<Token> {
        let mut token_list: Vec<Token> = Vec::new();

        while self.contents.len() > self.ct {
            let buf = self.get_tok();
            let tok = self.id_tok(buf);
            token_list.push(tok);
        }
        return token_list;
    }

    fn get_tok(&mut self) -> String {
        let mut buf = String::new();
        while self.cur_char() != ' ' {
            buf.push(self.cur_char());
            println!("{}", self.cur_char());
            self.adv();
        }
        self.adv();
        return buf;
    }

    fn id_tok(&mut self, mut tok: String) -> Token {
        let tok_bytes: Vec<char> = tok.chars().collect();
        let mut new_tok: Token;
        match tok_bytes {
            _ if tok_bytes[0] == '\'' && tok_bytes.last().unwrap().clone() == '\'' => {
                tok = self.strip_string(tok);
                new_tok = Token::new(TokenKind::String, tok);
            }

            _ => {
                new_tok = Token::new(TokenKind::Var, tok);
            }
        }
        return new_tok;
    }

    fn strip_string(&mut self, buf: String) -> String {
        let mut str_chars: Vec<char> = buf.chars().collect();
        str_chars.remove(0);
        str_chars.remove(str_chars.len() - 1);
        let mut my_str = String::new();
        for c in &str_chars {
            my_str.push(*c);
        }
        return my_str;
    }

    fn adv(&mut self) {
        self.ct += 1;
    }
}