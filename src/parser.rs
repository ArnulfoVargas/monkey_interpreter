use std::{collections::HashMap, path::Component::Prefix};

use crate::{
    ast::{
        ExpressionNode, ExpressionStatement, Identifier, LetStatement, Program, ReturnStatement,
        StatementNode,
    },
    lexer::Lexer,
    token::{
        Token,
        TokenKind::{self, Ident},
    },
};

type PrefixParseFn = fn(parser: &mut Parser) -> Option<ExpressionNode>;
type InfixParseFn = fn(parser: &mut Parser, exp: ExpressionNode) -> Option<ExpressionNode>;

enum PrecedenceLevel {
    Lowest = 0,
    Equals = 1,
    LessGreater = 2,
    Sum = 3,
    Product = 4,
    Prefix = 5,
    Call = 6,
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
    prefix_parse_fns: HashMap<TokenKind, PrefixParseFn>,
    infix_parse_fns: HashMap<TokenKind, InfixParseFn>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            current_token: Default::default(),
            peek_token: Default::default(),
            errors: vec![],
            prefix_parse_fns: HashMap::new(),
            infix_parse_fns: HashMap::new(),
        };

        parser.register_prefix(TokenKind::Ident, Self::parse_identifier);

        parser.next_token();
        parser.next_token();

        parser
    }

    fn parse_identifier(&mut self) -> Option<ExpressionNode> {
        Some(ExpressionNode::IdentifierNode(Identifier {
            token: self.current_token.clone(),
            value: self.current_token.literal.clone(),
        }))
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Option<Program> {
        let mut program = Program { statements: vec![] };

        while !self.current_token_is(TokenKind::Eof) {
            if let Some(statement) = self.parse_statement() {
                program.statements.push(statement);
            }

            self.next_token();
        }

        Some(program)
    }

    fn parse_statement(&mut self) -> Option<StatementNode> {
        match self.current_token.kind {
            TokenKind::Let => self.parse_let_statement(),
            TokenKind::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_let_statement(&mut self) -> Option<StatementNode> {
        let mut stmt: LetStatement = LetStatement {
            token: self.current_token.clone(),
            name: Default::default(),
            value: Default::default(),
        };

        return if !self.expect_peek(TokenKind::Ident) {
            None
        } else {
            stmt.name = Identifier {
                token: self.current_token.clone(),
                value: self.current_token.literal.clone(),
            };

            // TODO add asignation to 0
            if !self.expect_peek(TokenKind::Assign) {
                None
            } else {
                self.next_token();
                // TODO need to parse expression
                while !self.current_token_is(TokenKind::Semicolon) {
                    self.next_token();
                }

                Some(StatementNode::Let(stmt))
            }
        };
    }

    fn parse_expression_statement(&mut self) -> Option<StatementNode> {
        let stmt = ExpressionStatement {
            token: self.current_token.clone(),
            expression: self.parse_expression(PrecedenceLevel::Lowest),
        };

        if self.peek_token_is(TokenKind::Semicolon) {
            self.next_token();
        }

        Some(StatementNode::Expression(stmt))
    }

    fn parse_expression(&mut self, precedence: PrecedenceLevel) -> Option<ExpressionNode> {
        let prefix = self.prefix_parse_fns.get(&self.current_token.kind);
        if let Some(prefix_fn) = prefix {
            let left_exp = prefix_fn(self);
            return left_exp;
        };
        None
    }

    fn expect_peek(&mut self, kind: TokenKind) -> bool {
        if self.peek_token_is(kind.clone()) {
            self.next_token();
            return true;
        }
        self.peek_error(kind);
        false
    }

    fn peek_token_is(&self, kind: TokenKind) -> bool {
        self.peek_token.kind == kind
    }

    fn current_token_is(&self, kind: TokenKind) -> bool {
        self.current_token.kind == kind
    }

    pub fn errors(&self) -> &Vec<String> {
        &self.errors
    }

    fn peek_error(&mut self, token_kind: TokenKind) {
        let msg = format!(
            "expected next token to be '{}', got '{}' instead",
            token_kind, self.peek_token.kind,
        );

        self.errors.push(msg);
    }

    fn parse_return_statement(&mut self) -> Option<StatementNode> {
        let stmt = ReturnStatement {
            token: self.current_token.clone(),
            ret_value: Default::default(),
        };

        self.next_token();

        while !self.current_token_is(TokenKind::Semicolon) {
            self.next_token();
        }

        Some(StatementNode::Return(stmt))
    }

    fn register_prefix(&mut self, token_kind: TokenKind, prefix_parse_fn: PrefixParseFn) {
        self.prefix_parse_fns.insert(token_kind, prefix_parse_fn);
    }

    fn register_infix(&mut self, token_kind: TokenKind, infix_parse_fn: InfixParseFn) {
        self.infix_parse_fns.insert(token_kind, infix_parse_fn);
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{ExpressionNode, Identifier, Node, StatementNode},
        lexer::Lexer,
        parser::Parser,
    };

    #[test]
    fn test_let_statements() {
        let input = r#"
        let x = 5;
        let y= 10;
        let foobar=8383;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();

        check_parser_erors(parser);

        match program {
            None => panic!("parse program should not be none"),
            Some(program) => {
                assert_eq!(
                    program.statements.len(),
                    3,
                    "statements does not contain 3 statements: got: {}",
                    program.statements.len()
                );

                let expected = vec!["x", "y", "foobar"];
                for (idx, exp) in expected.into_iter().enumerate() {
                    let stmt = &program.statements[idx];

                    test_let_statement(stmt, exp);
                }
            }
        }
    }

    fn test_let_statement(stmt: &StatementNode, expected: &str) {
        assert_eq!(
            stmt.token_literal(),
            "let",
            "token literal not 'let'. got = {}",
            stmt.token_literal()
        );

        match stmt {
            StatementNode::Let(stm) => {
                assert_eq!(
                    stm.name.value, expected,
                    "LetStatement name value not {}. got = {}",
                    expected, stm.name.value
                );

                assert_eq!(
                    stm.name.token_literal(),
                    expected,
                    "LetStatement name value not {}. got = {}",
                    expected,
                    stm.name.token_literal()
                )
            }
            _ => panic!("not a let statement"),
        }
    }

    fn check_parser_erors(parser: Parser) {
        let errors = parser.errors();

        if errors.len() == 0 {
            return;
        }

        for error in errors {
            eprintln!("parser error: {}", error);
        }

        panic!("parser error present");
    }

    #[test]
    fn test_return_statements() {
        let input = r#"
        return 5;
        return 10;
        return 992395;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        check_parser_erors(parser);

        match program {
            None => panic!("parse program should not be none"),
            Some(program) => {
                assert_eq!(
                    program.statements.len(),
                    3,
                    "statements does not contain 3 statements: got: {}",
                    program.statements.len()
                );

                for stmt in program.statements {
                    match stmt {
                        StatementNode::Return(stm) => {
                            assert_eq!(
                                stm.token_literal(),
                                "return",
                                "token literal not 'return', got = '{}'",
                                stm.token_literal()
                            )
                        }
                        _ => panic!("not a return statement"),
                    }
                }
            }
        }
    }

    #[test]
    fn test_identifier_expresion() {
        let input = "foobar;";
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);

        let program = parser.parse_program().unwrap();
        check_parser_erors(parser);

        assert_eq!(
            program.statements.len(),
            1,
            "statements does not containt enoufh statements. got {}",
            program.statements.len()
        );

        match &program.statements[0] {
            StatementNode::Expression(exp) => {
                assert!(exp.expression.is_some());
                match exp.expression.as_ref().unwrap() {
                    ExpressionNode::IdentifierNode(ident) => {
                        assert_eq!(
                            ident.value, "foobar",
                            "identifier value not 'foobar'. got = {}",
                            ident.value
                        );

                        assert_eq!(
                            ident.token_literal(),
                            "foobar",
                            "identifier.token_literal is not foobar. got = {}",
                            ident.token_literal()
                        );
                    }
                }
            }
            other => panic!(
                "program.statements[0] is not ExpressionStatement. got = '{:?}'",
                other
            ),
        }
    }
}
