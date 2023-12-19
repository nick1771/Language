use std::collections::HashMap;

use crate::lexer;
use crate::lexer::token::Token;
use crate::lexer::token::TokenKind;

use crate::parser;
use crate::parser::tree::Binary;
use crate::parser::tree::Literal;
use crate::parser::tree::Statement;
use crate::parser::tree::StatementVisitor;
use crate::parser::tree::Unary;
use crate::parser::tree::VariableDefinition;

#[derive(Debug, Clone)]
pub enum Object {
    String(String),
    Boolean(bool),
    Number(f32),
    None,
}

impl Object {
    fn number(&self) -> f32 {
        match self {
            Object::Number(value) => *value,
            _ => panic!("Canot cast {:?} to boolean", self),
        }
    }

    fn boolean(&self) -> bool {
        match self {
            Object::Boolean(value) => *value,
            _ => true,
        }
    }

    fn equals(&self, rhs: &Object) -> bool {
        match (self, rhs) {
            (Object::Number(lhs), Object::Number(rhs)) => lhs == rhs,
            (Object::String(lhs), Object::String(rhs)) => lhs == rhs,
            (Object::Boolean(lhs), Object::Boolean(rhs)) => lhs == rhs,
            _ => panic!("Cannot compare {:?} to {:?}", self, rhs),
        }
    }
}

struct StatementInterpreter<'a> {
    context: &'a mut ProgramContext,
    tree: &'a Statement,
    source: &'a str,
}

impl<'a> StatementInterpreter<'a> {
    pub fn new(
        tree: &'a Statement,
        source: &'a str,
        context: &'a mut ProgramContext,
    ) -> StatementInterpreter<'a> {
        StatementInterpreter {
            tree,
            source,
            context,
        }
    }

    pub fn interpret(&mut self) -> Object {
        self.evaluate(self.tree.root_index)
    }

    fn evaluate(&mut self, index: usize) -> Object {
        self.tree.tree.get(index).unwrap().visit(self)
    }

    fn get_token_value(&self, token: &Token) -> &'a str {
        &self.source[token.offset..token.end]
    }

    fn get_token_string_value(&self, token: &Token) -> &'a str {
        &self.source[token.offset + 1..token.end - 1]
    }

    fn evaluate_binary_addition(&self, left: &Object, right: &Object) -> Object {
        match (left, right) {
            (Object::String(rhs), Object::String(lhs)) => {
                let mut result = rhs.to_owned();
                result.push_str(lhs);
                Object::String(result)
            }
            (Object::Number(rhs), Object::Number(lhs)) => Object::Number(rhs + lhs),
            _ => panic!("Operator + operands must be strings or numbers"),
        }
    }
}

impl StatementVisitor<Object> for StatementInterpreter<'_> {
    fn handle_literal_expression(&mut self, literal: &Literal) -> Object {
        if literal.token.kind == TokenKind::String {
            let value = self.get_token_string_value(&literal.token);
            Object::String(value.to_string())
        } else if literal.token.kind == TokenKind::Number {
            let value = self.get_token_value(&literal.token);
            Object::Number(value.parse::<f32>().unwrap())
        } else if literal.token.kind == TokenKind::True {
            Object::Boolean(true)
        } else if literal.token.kind == TokenKind::False {
            Object::Boolean(false)
        } else {
            panic!("Unexpected literal type {:?}", literal.token.kind)
        }
    }

    fn handle_binary_expression(&mut self, binary: &Binary) -> Object {
        let left = self.evaluate(binary.left);
        let right = self.evaluate(binary.right);

        match binary.operator.kind {
            TokenKind::Plus => self.evaluate_binary_addition(&left, &right),
            TokenKind::Greater => Object::Boolean(left.number() > right.number()),
            TokenKind::GreaterEqual => Object::Boolean(left.number() >= right.number()),
            TokenKind::Less => Object::Boolean(left.number() < right.number()),
            TokenKind::LessEqual => Object::Boolean(left.number() <= right.number()),
            TokenKind::BangEqual => Object::Boolean(!left.equals(&right)),
            TokenKind::EqualEqual => Object::Boolean(left.equals(&right)),
            TokenKind::Minus => Object::Number(left.number() - right.number()),
            TokenKind::Slash => Object::Number(left.number() / right.number()),
            TokenKind::Star => Object::Number(left.number() * right.number()),
            _ => panic!("Unexpected binary operator {:?}", binary.operator.kind),
        }
    }

    fn handle_grouping_expression(&mut self, index: usize) -> Object {
        self.evaluate(index)
    }

    fn handle_unary_expression(&mut self, unary: &Unary) -> Object {
        let right = self.evaluate(unary.right);

        match unary.operator.kind {
            TokenKind::Minus => Object::Number(-right.number()),
            TokenKind::Bang => Object::Boolean(!right.boolean()),
            _ => panic!("Unexpected unary operator {:?}", unary.operator.kind),
        }
    }

    fn handle_variable_expression(&mut self, variable: &Token) -> Object {
        let name = self.get_token_value(variable);
        self.context.lookup_variable(name)
    }

    fn handle_variable_definition_statement(&mut self, declaration: &VariableDefinition) -> Object {
        let name = self.get_token_value(&declaration.identifier);
        let value = self.evaluate(declaration.expression);
        self.context.add_variable(name, &value);

        Object::None
    }

    fn handle_print_statement(&mut self, expression: usize) -> Object {
        let value = self.evaluate(expression);
        println!("{:?}", value);

        Object::None
    }
}

#[derive(Default)]
struct Frame {
    pub variables: HashMap<String, Object>,
}

pub struct ProgramContext {
    stack_frames: Vec<Frame>,
}

impl Default for ProgramContext {
    fn default() -> Self {
        Self::new()
    }
}

impl ProgramContext {
    pub fn new() -> Self {
        Self {
            stack_frames: vec![Frame::default()],
        }
    }

    pub fn interpret(&mut self, source: &str) {
        let tokens = lexer::tokenize(source);
        let statements = parser::parse(tokens);

        for statement in statements.iter() {
            let mut interpreter = StatementInterpreter::new(statement, source, self);
            interpreter.interpret();
        }
    }

    pub fn add_variable(&mut self, name: &str, value: &Object) {
        self.stack_frames
            .last_mut()
            .unwrap()
            .variables
            .insert(name.to_string(), value.clone());
    }

    pub fn lookup_variable(&self, name: &str) -> Object {
        self.stack_frames
            .last()
            .unwrap()
            .variables
            .get(name)
            .unwrap_or_else(|| panic!("Undeclared variable {}", name))
            .clone()
    }
}
