#[derive(Debug, PartialEq, Clone, Copy)]

pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Minus,
    Plus,
    Slash,
    Star,
    Equal,
    Less,
    Greater,
    Bang,
    Semicolon,
    LessEqual,
    GreaterEqual,
    EqualEqual,
    BangEqual,

    Identifier,
    String,
    Number,

    And,
    Else,
    If,
    Or,
    Var,
    True,
    False,
    Class,
    Fn,
    For,
    While,
    Return,
    Print,

    None,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Token {
    pub kind: TokenKind,
    pub offset: usize,
    pub end: usize,
}

impl Token {
    pub const fn from_kind(kind: TokenKind) -> Self {
        Self {
            kind,
            offset: 0,
            end: 0,
        }
    }
}

pub trait TokenRepresentation {
    fn get_token_kind(&self) -> TokenKind;
}

pub trait Identifier {
    fn is_identifier_start(&self) -> bool;

    fn is_identifier_continue(&self) -> bool;
}

impl TokenRepresentation for char {
    fn get_token_kind(&self) -> TokenKind {
        match self {
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '[' => TokenKind::LeftBracket,
            ']' => TokenKind::RightBracket,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            '.' => TokenKind::Dot,
            ',' => TokenKind::Comma,
            '-' => TokenKind::Minus,
            '+' => TokenKind::Plus,
            '=' => TokenKind::Equal,
            '<' => TokenKind::Less,
            '>' => TokenKind::Greater,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '!' => TokenKind::Bang,
            ';' => TokenKind::Semicolon,
            _ => TokenKind::None,
        }
    }
}

impl TokenRepresentation for str {
    fn get_token_kind(&self) -> TokenKind {
        match self {
            "else" => TokenKind::Else,
            "and" => TokenKind::And,
            "var" => TokenKind::Var,
            "if" => TokenKind::If,
            "or" => TokenKind::Or,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "class" => TokenKind::Class,
            "fn" => TokenKind::Fn,
            "for" => TokenKind::For,
            "while" => TokenKind::While,
            "return" => TokenKind::Return,
            "print" => TokenKind::Print,
            _ => TokenKind::None,
        }
    }
}

impl Identifier for char {
    fn is_identifier_start(&self) -> bool {
        self.is_ascii_alphabetic() || *self == '_'
    }

    fn is_identifier_continue(&self) -> bool {
        self.is_identifier_start() || self.is_ascii_digit()
    }
}
