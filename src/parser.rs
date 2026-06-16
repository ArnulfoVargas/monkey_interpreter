use crate::{
    ast::{Identifier, LetStatement, Program, ReturnStatement, StatementNode},
    lexer::Lexer,
    token::{Token, TokenKind},
};

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    errors: Vec<String>,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Parser {
        let mut parser = Parser {
            lexer,
            current_token: Default::default(),
            peek_token: Default::default(),
            errors: vec![],
        };

        parser.next_token();
        parser.next_token();

        parser
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
            _ => None,
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
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{Node, StatementNode},
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
}
