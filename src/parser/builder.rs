use crate::lexer::token::Token;

use super::tree::{
    Binary, ExpressionNode, Literal, Statement, StatementNode, Unary, VariableDefinition,
};

pub struct StatementListBuilder {
    pub statements: Vec<Statement>,
}

impl Default for StatementListBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl StatementListBuilder {
    pub fn new() -> Self {
        StatementListBuilder {
            statements: Vec::new(),
        }
    }

    pub fn start_statement(&mut self) {
        self.statements.push(Statement::new())
    }

    pub fn end_statement(&mut self, root_index: usize) {
        let current_expression = self.statements.last_mut().expect("Expression not started");
        current_expression.root_index = root_index;
    }

    pub fn add_variable_definition(&mut self, identifier: Token, expression: usize) -> usize {
        let variable_definition = VariableDefinition {
            identifier,
            expression,
        };

        let statement_node = StatementNode::VariableDefinition(variable_definition);

        self.add_node(statement_node)
    }

    pub fn add_print_statement(&mut self, expression: usize) -> usize {
        let statement_node = StatementNode::Print(expression);
        self.add_node(statement_node)
    }

    pub fn add_binary(&mut self, left: usize, operator: Token, right: usize) -> usize {
        let binary = Binary {
            left,
            operator,
            right,
        };

        let expression_node = ExpressionNode::Binary(binary);
        let node = StatementNode::Expression(expression_node);

        self.add_node(node)
    }

    pub fn add_unary(&mut self, operator: Token, right: usize) -> usize {
        let unary = Unary { operator, right };

        let expression_node = ExpressionNode::Unary(unary);
        let node = StatementNode::Expression(expression_node);

        self.add_node(node)
    }

    pub fn add_literal(&mut self, token: Token) -> usize {
        let literal = Literal { token };

        let expression_node = ExpressionNode::Literal(literal);
        let node = StatementNode::Expression(expression_node);

        self.add_node(node)
    }

    pub fn add_variable(&mut self, token: Token) -> usize {
        let expression_node = ExpressionNode::Variable(token);
        let node = StatementNode::Expression(expression_node);

        self.add_node(node)
    }

    pub fn add_grouping(&mut self, index: usize) -> usize {
        let grouping_node = ExpressionNode::Grouping(index);
        let node = StatementNode::Expression(grouping_node);

        self.add_node(node)
    }

    fn add_node(&mut self, node: StatementNode) -> usize {
        let current_expression = self.statements.last_mut().expect("Expression not started");

        let index = current_expression.tree.len();
        current_expression.tree.push(node);

        index
    }
}
