use crate::lexer::token::TokenKind;

pub fn is_equality_token(kind: TokenKind) -> bool {
    matches!(kind, TokenKind::BangEqual | TokenKind::EqualEqual)
}

pub fn is_comparison_token(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::Greater | TokenKind::GreaterEqual | TokenKind::Less | TokenKind::LessEqual
    )
}

pub fn is_term_token(kind: TokenKind) -> bool {
    matches!(kind, TokenKind::Minus | TokenKind::Plus)
}

pub fn is_factor_token(kind: TokenKind) -> bool {
    matches!(kind, TokenKind::Slash | TokenKind::Star)
}

pub fn is_unary_token(kind: TokenKind) -> bool {
    matches!(kind, TokenKind::Bang | TokenKind::Minus)
}

pub fn is_primary_token(kind: TokenKind) -> bool {
    matches!(
        kind,
        TokenKind::True | TokenKind::False | TokenKind::String | TokenKind::Number
    )
}
