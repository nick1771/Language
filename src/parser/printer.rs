use crate::lexer::token::Token;

use super::tree::Binary;
use super::tree::Literal;
use super::tree::Statement;
use super::tree::StatementVisitor;
use super::tree::Unary;
use super::tree::VariableDefinition;

struct DebugPrinter<'a> {
    tree: &'a Statement,
    source: &'a str,
}

impl<'a> DebugPrinter<'a> {
    pub fn debug_print(&mut self) {
        self.tree
            .tree
            .get(self.tree.root_index)
            .unwrap()
            .visit(self);

        println!()
    }

    fn print_node(&mut self, name: &str, expressions: &[usize]) {
        print!("(");
        print!("{}", name);

        for index in expressions {
            print!(" ");
            self.tree.tree.get(*index).unwrap().visit(self)
        }

        print!(")");
    }

    fn get_token_value(&self, token: &Token) -> &'a str {
        &self.source[token.offset..token.end]
    }
}

impl StatementVisitor<()> for DebugPrinter<'_> {
    fn handle_literal_expression(&mut self, literal: &Literal) {
        print!("{}", self.get_token_value(&literal.token));
    }

    fn handle_binary_expression(&mut self, binary: &Binary) {
        let operator_value = self.get_token_value(&binary.operator);
        self.print_node(operator_value, &[binary.left, binary.right]);
    }

    fn handle_grouping_expression(&mut self, index: usize) {
        self.print_node("group", &[index]);
    }

    fn handle_unary_expression(&mut self, unary: &Unary) {
        let operator_value = self.get_token_value(&unary.operator);
        self.print_node(operator_value, &[unary.right]);
    }

    fn handle_variable_expression(&mut self, variable: &Token) {
        print!("{}", self.get_token_value(variable));
    }

    fn handle_variable_definition_statement(&mut self, declaration: &VariableDefinition) {
        let variable_name = self.get_token_value(&declaration.identifier);
        self.print_node(variable_name, &[declaration.expression]);
    }

    fn handle_print_statement(&mut self, expression: usize) {
        self.print_node("print", &[expression])
    }
}

pub trait DebugPrint {
    fn debug_print(&self, source: &str);
}

impl DebugPrint for Statement {
    fn debug_print(&self, source: &str) {
        let mut printer = DebugPrinter { tree: self, source };
        printer.debug_print();
    }
}
