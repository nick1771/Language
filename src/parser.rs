pub mod builder;
pub mod printer;
pub mod rules;
pub mod tree;

use self::builder::StatementListBuilder;
use self::rules::is_comparison_token;
use self::rules::is_equality_token;
use self::rules::is_factor_token;
use self::rules::is_term_token;
use self::rules::is_unary_token;
use self::tree::Statement;

use crate::cursor::Cursor;
use crate::cursor::ToCursor;

use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;
use crate::parser::rules::is_primary_token;

struct RecursiveDescentParser {
    cursor: Cursor<Token>,
    builder: StatementListBuilder,
}

impl RecursiveDescentParser {
    const TERMINATOR_TOKEN: Token = Token::from_kind(TokenKind::None);

    pub fn new(tokens: Vec<Token>) -> RecursiveDescentParser {
        RecursiveDescentParser {
            cursor: tokens.to_cursor(Self::TERMINATOR_TOKEN),
            builder: StatementListBuilder::new(),
        }
    }

    pub fn parse(mut self) -> Vec<Statement> {
        loop {
            if self.cursor.is_at_end() {
                break self.builder.statements;
            }

            self.statement();
        }
    }

    fn statement(&mut self) {
        if self.matches(|kind| kind == TokenKind::Var).is_some() {
            self.variable_definition_statement();
        } else if self.matches(|kind| kind == TokenKind::Print).is_some() {
            self.print_statement();
        } else {
            self.expression_statement();
        }
    }

    fn variable_definition_statement(&mut self) {
        let name = self.expect(TokenKind::Identifier);
        self.expect(TokenKind::Equal);

        self.builder.start_statement();

        let expression = self.expression();
        let variable_definition = self.builder.add_variable_definition(name, expression);

        self.builder.end_statement(variable_definition);

        self.expect(TokenKind::Semicolon);
    }

    fn print_statement(&mut self) {
        self.builder.start_statement();

        let expression = self.expression();
        let print_statement = self.builder.add_print_statement(expression);

        self.builder.end_statement(print_statement);

        self.expect(TokenKind::Semicolon);
    }

    fn expression_statement(&mut self) {
        self.builder.start_statement();
        let root_index = self.expression();
        self.builder.end_statement(root_index);

        self.expect(TokenKind::Semicolon);
    }

    fn expression(&mut self) -> usize {
        self.equality()
    }

    fn equality(&mut self) -> usize {
        let mut left = self.comparison();

        while let Some(operator) = self.matches(is_equality_token) {
            let right = self.comparison();
            left = self.builder.add_binary(left, operator, right);
        }

        left
    }

    fn comparison(&mut self) -> usize {
        let mut left = self.term();

        while let Some(operator) = self.matches(is_comparison_token) {
            let right = self.term();
            left = self.builder.add_binary(left, operator, right);
        }

        left
    }

    fn term(&mut self) -> usize {
        let mut left = self.factor();

        while let Some(operator) = self.matches(is_term_token) {
            let right = self.factor();
            left = self.builder.add_binary(left, operator, right);
        }

        left
    }

    fn factor(&mut self) -> usize {
        let mut left = self.unary();

        while let Some(operator) = self.matches(is_factor_token) {
            let right = self.unary();
            left = self.builder.add_binary(left, operator, right);
        }

        left
    }

    fn unary(&mut self) -> usize {
        if let Some(operator) = self.matches(is_unary_token) {
            let right = self.unary();
            self.builder.add_unary(operator, right)
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> usize {
        if let Some(token) = self.matches(is_primary_token) {
            self.builder.add_literal(token)
        } else if self.matches(|kind| kind == TokenKind::LeftParen).is_some() {
            let expression = self.expression();
            self.expect(TokenKind::RightParen);
            self.builder.add_grouping(expression)
        } else if let Some(token) = self.matches(|kind| kind == TokenKind::Identifier) {
            self.builder.add_variable(token)
        } else {
            panic!("Expected a primary expression!")
        }
    }

    fn matches(&mut self, rule: fn(TokenKind) -> bool) -> Option<Token> {
        if rule(self.cursor.peek(0).kind) {
            Some(self.cursor.next_or_end())
        } else {
            None
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Token {
        let next_token = self.cursor.next_or_end();
        if kind == next_token.kind {
            next_token
        } else {
            panic!("Expected token kind {:?}", kind)
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<Statement> {
    let parser = RecursiveDescentParser::new(tokens);
    parser.parse()
}

#[allow(unused_imports)]
mod tests {

    use crate::lexer;
    use crate::parser;
    use crate::parser::tree::ExpressionNode;
    use crate::parser::tree::StatementNode;

    #[test]
    fn should_parse_binary_expression() {
        let source = "1 + 2;";

        let tokens = lexer::tokenize(source);
        let statements = parser::parse(tokens);

        assert_eq!(statements.len(), 1);

        let root = statements[0].tree.get(statements[0].root_index);

        assert!(matches!(
            root,
            Some(StatementNode::Expression(ExpressionNode::Binary(_)))
        ))
    }

    #[test]
    fn should_parse_unary_expression() {
        let source = "-2;";

        let tokens = lexer::tokenize(source);
        let statements = parser::parse(tokens);

        assert_eq!(statements.len(), 1);

        let root = statements[0].tree.get(statements[0].root_index);

        assert!(matches!(
            root,
            Some(StatementNode::Expression(ExpressionNode::Unary(_)))
        ))
    }

    #[test]
    fn should_parse_literal_expression() {
        let source = "12341231;";

        let tokens = lexer::tokenize(source);
        let statements = parser::parse(tokens);

        assert_eq!(statements.len(), 1);

        let root = statements[0].tree.get(statements[0].root_index);

        assert!(matches!(
            root,
            Some(StatementNode::Expression(ExpressionNode::Literal(_)))
        ))
    }

    #[test]
    fn should_parse_grouping_expression() {
        let source = "(12341231 + 123);";

        let tokens = lexer::tokenize(source);
        let statements = parser::parse(tokens);

        assert_eq!(statements.len(), 1);

        let root = statements[0].tree.get(statements[0].root_index);

        assert!(matches!(
            root,
            Some(StatementNode::Expression(ExpressionNode::Grouping(_)))
        ))
    }

    #[test]
    fn should_parse_variable_expression() {
        let source = "asdasdsad;";

        let tokens = lexer::tokenize(source);
        let statements = parser::parse(tokens);

        assert_eq!(statements.len(), 1);

        let root = statements[0].tree.get(statements[0].root_index);

        assert!(matches!(
            root,
            Some(StatementNode::Expression(ExpressionNode::Variable(_)))
        ))
    }

    #[test]
    fn should_parse_variable_definition_statement() {
        let source = "var b = 123;";

        let tokens = lexer::tokenize(source);
        let statements = parser::parse(tokens);

        assert_eq!(statements.len(), 1);

        let root = statements[0].tree.get(statements[0].root_index);

        assert!(matches!(root, Some(StatementNode::VariableDefinition(_))))
    }

    #[test]
    fn should_parse_print_statement() {
        let source = "print 123;";

        let tokens = lexer::tokenize(source);
        let statements = parser::parse(tokens);

        assert_eq!(statements.len(), 1);

        let root = statements[0].tree.get(statements[0].root_index);

        assert!(matches!(root, Some(StatementNode::Print(_))))
    }
}
