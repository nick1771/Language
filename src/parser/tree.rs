use crate::lexer::token::Token;

pub struct Literal {
    pub token: Token,
}

pub struct Binary {
    pub left: usize,
    pub operator: Token,
    pub right: usize,
}

pub struct Unary {
    pub operator: Token,
    pub right: usize,
}

pub struct VariableDefinition {
    pub identifier: Token,
    pub expression: usize,
}

pub enum ExpressionNode {
    Unary(Unary),
    Binary(Binary),
    Literal(Literal),
    Grouping(usize),
    Variable(Token),
}

pub enum StatementNode {
    Expression(ExpressionNode),
    VariableDefinition(VariableDefinition),
    Print(usize),
}

impl StatementNode {
    pub fn visit<T>(&self, visitor: &mut dyn StatementVisitor<T>) -> T {
        match self {
            StatementNode::Expression(node) => match node {
                ExpressionNode::Unary(expr) => visitor.handle_unary_expression(expr),
                ExpressionNode::Literal(expr) => visitor.handle_literal_expression(expr),
                ExpressionNode::Grouping(expr) => visitor.handle_grouping_expression(*expr),
                ExpressionNode::Binary(expr) => visitor.handle_binary_expression(expr),
                ExpressionNode::Variable(token) => visitor.handle_variable_expression(token),
            },
            StatementNode::VariableDefinition(definition) => {
                visitor.handle_variable_definition_statement(definition)
            }
            StatementNode::Print(expr) => visitor.handle_print_statement(*expr),
        }
    }
}

pub struct Statement {
    pub tree: Vec<StatementNode>,
    pub root_index: usize,
}

impl Default for Statement {
    fn default() -> Self {
        Self::new()
    }
}

impl Statement {
    pub fn new() -> Self {
        Self {
            tree: Vec::new(),
            root_index: 0,
        }
    }
}

pub trait StatementVisitor<T> {
    fn handle_literal_expression(&mut self, literal: &Literal) -> T;

    fn handle_binary_expression(&mut self, binary: &Binary) -> T;

    fn handle_grouping_expression(&mut self, index: usize) -> T;

    fn handle_unary_expression(&mut self, unary: &Unary) -> T;

    fn handle_variable_expression(&mut self, variable: &Token) -> T;

    fn handle_variable_definition_statement(&mut self, declaration: &VariableDefinition) -> T;

    fn handle_print_statement(&mut self, expression: usize) -> T;
}
