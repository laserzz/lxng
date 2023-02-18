use std::{fs, env};

fn main() {
    let fp: Vec<String> = env::args().collect();

    let contents = fs::read_to_string(&fp[1]);

    let mut lexer = Lexer::new(strip(contents.unwrap()));
    let toks = lexer.lex();
    let mut parser = Parser::new(toks);
    parser.parse();
}

fn strip(contents: String) -> String {
    let mut cloned = contents.to_string();
    cloned = cloned.replace('\n', " ");
    cloned = cloned.replace('\r', "");
    return cloned;
}

#[derive(Debug)]
#[derive(Clone)]
enum TokenKind {
    String,
    Var,
    Put
}

#[derive(Debug)]
#[derive(Clone)]
struct Variable {
    name: String,
    literal: String,
}

#[derive(Debug)]
#[derive(Clone)]
struct Token {
    kind: TokenKind,
    literal: String
}

#[derive(Debug)]
struct Lexer {
    contents: Vec<char>,
    ct: usize
}

impl Variable {
    pub fn new(name: String, literal: String) -> Self {
        Self {
            name: name,
            literal: literal
        }
    }
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
        if self.cur_char() == '\'' {
            buf.push(self.cur_char());
            self.adv();
            while self.cur_char() != '\'' {
                buf.push(self.cur_char());
                self.adv();
            }
            buf.push(self.cur_char());
            self.adv();
        } else {
            while self.cur_char() != ' ' {
                buf.push(self.cur_char());
                self.adv();
            }
        }
        self.adv();
        return buf;
    }

    fn id_tok(&mut self, mut tok: String) -> Token {
        let tok_bytes: Vec<char> = tok.chars().collect();
        let new_tok: Token;
        match tok_bytes {
            _ if tok_bytes[0] == '\'' && tok_bytes.last().unwrap().clone() == '\'' => {
                tok = self.strip_string(tok);
                new_tok = Token::new(TokenKind::String, tok);
            }

            _ if tok == "pt" => {
                new_tok = Token::new(TokenKind::Put, tok);
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

// Parser

struct Parser {
    tokens: Vec<Token>,
    ct: usize,
    stack: Vec<Variable>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            ct: 0,
            stack: Vec::new()
        }
    }

    pub fn parse(&mut self) {
        while self.tokens.len() > self.ct {
            match self.cur_tok().kind {
                TokenKind::String => {
                    let v = self.str_var();
                    if v {
                        self.stack.push(Variable::new(self.find_tok(-1).literal, self.cur_tok().literal));
                        self.adv();
                    } else {
                        self.adv();
                    }
                }

                TokenKind::Put => {
                    let res = self.to_print().unwrap();
                    println!("{}", res);
                    self.adv();
                }

                _ => {
                    self.adv();
                }
            }
        }
    }

    fn cur_tok(&self) -> Token {
        let t = self.tokens.get(self.ct);
        return t.unwrap().clone();
    }

    fn str_var(&self) -> bool {
        if matches!(self.tokens[self.ct - 1].kind, TokenKind::Var) {
            true
        } else {
            false
        }
    }

    fn find_tok(&self, offset: i32) -> Token {
        let o: usize;
        let sum: usize;
        if offset.is_negative() {
            o = -offset as usize;
            sum = self.ct - o;
        } else {
            o = offset as usize;
            sum = self.ct + o;
        }
        self.tokens[sum].to_owned()
    }

    fn to_print(&self) -> Result<String, ()> {
        let thing = self.find_tok(1);
        if matches!(thing.kind, TokenKind::String) {
            Ok(thing.literal)
        } else if matches!(thing.kind, TokenKind::Var) {
            let res = self.stack.iter().position(|v| v.name == thing.literal);

            if !res.is_none() {
                let ind = self.stack[res.unwrap()].to_owned();
                Ok(ind.literal)
            } else {
                panic!("Var {} not found", self.find_tok(1).literal);
            }
        } else {
            Err(())
        }
    }

    fn adv(&mut self) {
        self.ct += 1;
    }
}