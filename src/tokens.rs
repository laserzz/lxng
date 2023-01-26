mod tokens;

enum TokenKind {
    String,
    Var
}

struct Token {
    kind: TokenKind,
    literal: String
}

impl Token {
    pub fn new(kind: TokenKind, literal: String) {
        Self {
            kind: kind,
            literal: literal
        }
    }
}