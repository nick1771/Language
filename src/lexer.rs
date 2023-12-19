use crate::cursor::{Cursor, ToCursor};

use self::token::{Identifier, Token, TokenKind, TokenRepresentation};

pub mod token;

struct Lexer<'a> {
    source: &'a str,
    cursor: Cursor<char>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Lexer<'a> {
        Lexer {
            cursor: source.chars().collect::<Vec<char>>().to_cursor('\0'),
            source,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.cursor.is_at_end() {
            self.cursor.skip_while(|ch| ch.is_whitespace());

            let token_start_offset = self.cursor.offset();
            let token_start_char = self.cursor.next_or_end();

            let token_kind = if token_start_char.is_numeric() {
                self.parse_number()
            } else if token_start_char.is_identifier_start() {
                self.parse_identifier(token_start_offset)
            } else if token_start_char == '\"' {
                self.parse_string()
            } else {
                self.parse_character(token_start_char)
            };

            if token_kind == TokenKind::None {
                continue;
            }

            let token = Token {
                kind: token_kind,
                offset: token_start_offset,
                end: self.cursor.offset(),
            };

            tokens.push(token)
        }

        tokens
    }

    fn parse_character(&mut self, character: char) -> TokenKind {
        if character == '!' && self.cursor.matches('=') {
            TokenKind::BangEqual
        } else if character == '<' && self.cursor.matches('=') {
            TokenKind::LessEqual
        } else if character == '>' && self.cursor.matches('=') {
            TokenKind::GreaterEqual
        } else if character == '=' && self.cursor.matches('=') {
            TokenKind::EqualEqual
        } else {
            character.get_token_kind()
        }
    }

    fn parse_identifier(&mut self, token_start_offset: usize) -> TokenKind {
        self.cursor.skip_while(|ch| ch.is_identifier_continue());

        let token_value = &self.source[token_start_offset..self.cursor.offset()];
        match token_value.get_token_kind() {
            TokenKind::None => TokenKind::Identifier,
            other => other,
        }
    }

    fn parse_number(&mut self) -> TokenKind {
        self.cursor.skip_while(|ch| ch.is_ascii_digit());
        if self.cursor.peek_first() == '.' && self.cursor.peek_second().is_ascii_digit() {
            self.cursor.next_or_end();
            self.cursor.skip_while(|ch| ch.is_ascii_digit());
        }

        TokenKind::Number
    }

    fn parse_string(&mut self) -> TokenKind {
        self.cursor.skip_while(|ch| ch != '"');
        if self.cursor.peek(0) != '"' {
            panic!("Unterminated string")
        } else {
            self.cursor.next_or_end();
        }

        TokenKind::String
    }
}

pub fn tokenize(source: &str) -> Vec<Token> {
    let mut lexer = Lexer::new(source);
    lexer.tokenize()
}

mod tests {

    #[allow(unused_imports)]
    use crate::lexer::token::TokenKind;

    #[allow(unused_imports)]
    use super::tokenize;

    #[test]
    fn should_parse_text1() {
        let source = "   23123123 some_identifier         \"some string\"";
        let tokens = tokenize(source);

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].kind, TokenKind::Number);
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[2].kind, TokenKind::String);
    }

    #[test]
    fn should_parse_text2() {
        let source = "{}[]();+-===<=>=!==></*";
        let tokens = tokenize(source);

        assert_eq!(tokens.len(), 19);

        assert_eq!(tokens[0].kind, TokenKind::LeftBrace);
        assert_eq!(tokens[1].kind, TokenKind::RightBrace);
        assert_eq!(tokens[2].kind, TokenKind::LeftBracket);
        assert_eq!(tokens[3].kind, TokenKind::RightBracket);
        assert_eq!(tokens[4].kind, TokenKind::LeftParen);
        assert_eq!(tokens[5].kind, TokenKind::RightParen);
        assert_eq!(tokens[6].kind, TokenKind::Semicolon);
        assert_eq!(tokens[7].kind, TokenKind::Plus);
        assert_eq!(tokens[8].kind, TokenKind::Minus);
        assert_eq!(tokens[9].kind, TokenKind::EqualEqual);
        assert_eq!(tokens[10].kind, TokenKind::Equal);
        assert_eq!(tokens[11].kind, TokenKind::LessEqual);
        assert_eq!(tokens[12].kind, TokenKind::GreaterEqual);
        assert_eq!(tokens[13].kind, TokenKind::BangEqual);
        assert_eq!(tokens[14].kind, TokenKind::Equal);
        assert_eq!(tokens[15].kind, TokenKind::Greater);
        assert_eq!(tokens[16].kind, TokenKind::Less);
        assert_eq!(tokens[17].kind, TokenKind::Slash);
        assert_eq!(tokens[18].kind, TokenKind::Star);
    }

    #[test]
    fn should_parse_text3() {
        let source = "123.3123.function_call";
        let tokens = tokenize(source);

        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].kind, TokenKind::Number);
        assert_eq!(tokens[1].kind, TokenKind::Dot);
        assert_eq!(tokens[2].kind, TokenKind::Identifier);
    }
}
