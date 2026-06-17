use std::collections::HashMap;

use crate::{
    ast::{
        Boolean, ExpressionNode, ExpressionStatement, Identifier, InfixExpression, IntegerLiteral,
        LetStatement, PrefixExpression, Program, ReturnStatement, StatementNode,
    },
    lexer::Lexer,
    token::{
        Token,
        TokenKind::{self},
    },
};

type PrefixParseFn = fn(parser: &mut Parser) -> Option<ExpressionNode>;
type InfixParseFn = fn(parser: &mut Parser, exp: ExpressionNode) -> Option<ExpressionNode>;

#[derive(Clone)]
enum PrecedenceLevel {
    Lowest = 0,
    Equals = 1,
    LessGreater = 2,
    Sum = 3,
    Product = 4,
    Prefix = 5,
    Call = 6,
}

fn precedence_map(kind: &TokenKind) -> PrecedenceLevel {
    return match kind {
        TokenKind::Equal => PrecedenceLevel::Equals,
        TokenKind::NotEqual => PrecedenceLevel::Equals,
        TokenKind::LessThan => PrecedenceLevel::LessGreater,
        TokenKind::GreaterThan => PrecedenceLevel::LessGreater,
        TokenKind::Plus => PrecedenceLevel::Sum,
        TokenKind::Minus => PrecedenceLevel::Sum,
        TokenKind::Asterisk => PrecedenceLevel::Product,
        TokenKind::Slash => PrecedenceLevel::Product,
        _ => PrecedenceLevel::Lowest,
    };
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
        parser.register_prefix(TokenKind::Int, Self::parse_integer_literal);
        parser.register_prefix(TokenKind::Bang, Self::parse_prefix_expression);
        parser.register_prefix(TokenKind::Minus, Self::parse_prefix_expression);
        parser.register_prefix(TokenKind::True, Self::parse_boolean);
        parser.register_prefix(TokenKind::False, Self::parse_boolean);
        parser.register_prefix(TokenKind::Lparen, Self::parse_grouped_expression);

        parser.register_infix(TokenKind::Plus, Self::parse_infix_expression);
        parser.register_infix(TokenKind::Minus, Self::parse_infix_expression);
        parser.register_infix(TokenKind::Slash, Self::parse_infix_expression);
        parser.register_infix(TokenKind::Asterisk, Self::parse_infix_expression);
        parser.register_infix(TokenKind::GreaterThan, Self::parse_infix_expression);
        parser.register_infix(TokenKind::LessThan, Self::parse_infix_expression);
        parser.register_infix(TokenKind::Equal, Self::parse_infix_expression);
        parser.register_infix(TokenKind::NotEqual, Self::parse_infix_expression);

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

    fn parse_integer_literal(&mut self) -> Option<ExpressionNode> {
        let mut literal = IntegerLiteral {
            token: self.current_token.clone(),
            value: Default::default(),
        };

        return match self.current_token.literal.parse::<i64>() {
            Ok(value) => {
                literal.value = value;
                Some(ExpressionNode::Integer(literal))
            }
            Err(_) => {
                let msg = format!("couldn't parse {} as integer", self.current_token.literal);
                self.errors.push(msg);
                None
            }
        };
    }

    fn parse_prefix_expression(&mut self) -> Option<ExpressionNode> {
        let mut expression = PrefixExpression {
            token: self.current_token.clone(),
            operator: self.current_token.literal.clone(),
            right: Default::default(),
        };

        self.next_token();
        match self.parse_expression(PrecedenceLevel::Prefix) {
            Some(exp) => {
                expression.right = Box::new(exp);
            }
            None => return None,
        }

        Some(ExpressionNode::Prefix(expression))
    }

    fn parse_infix_expression(&mut self, left: ExpressionNode) -> Option<ExpressionNode> {
        self.next_token();
        let mut expression = InfixExpression {
            token: self.current_token.clone(),
            left: Box::new(left),
            operator: self.current_token.literal.clone(),
            right: Default::default(),
        };

        let precedence = self.current_precedence();
        self.next_token();

        match self.parse_expression(precedence) {
            Some(exp) => expression.right = Box::new(exp),
            None => return None,
        }

        Some(ExpressionNode::Infix(expression))
    }

    fn parse_boolean(&mut self) -> Option<ExpressionNode> {
        Some(ExpressionNode::BooleanNode(Boolean {
            token: self.current_token.clone(),
            value: self.current_token_is(TokenKind::True),
        }))
    }

    fn parse_grouped_expression(&mut self) -> Option<ExpressionNode> {
        self.next_token();

        let exp = self.parse_expression(PrecedenceLevel::Lowest);
        if !self.expect_peek(TokenKind::Rparen) {
            return None;
        }

        exp
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
            let mut left_exp = prefix_fn(self);

            while !self.peek_token_is(TokenKind::Semicolon)
                && (precedence.clone() as u8) < (self.peek_precedence() as u8)
            {
                let infix_fn = self.infix_parse_fns.get(&self.peek_token.kind);
                if let Some(infix) = infix_fn {
                    left_exp = infix(self, left_exp.expect("left expression should be present"));
                }
            }

            return left_exp;
        }

        self.no_prefix_parse_fn_error(self.current_token.kind.clone());
        None
    }

    fn no_prefix_parse_fn_error(&mut self, kind: TokenKind) {
        let msg = format!("no prefix parse function for {} found", kind);
        self.errors.push(msg);
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

    fn peek_precedence(&self) -> PrecedenceLevel {
        precedence_map(&self.peek_token.kind)
    }

    fn current_precedence(&self) -> PrecedenceLevel {
        precedence_map(&self.current_token.kind)
    }
}

#[cfg(test)]
mod test {
    use std::any;

    use crate::{
        ast::{ExpressionNode, Node, StatementNode},
        lexer::Lexer,
        parser::Parser,
        token::TokenKind,
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
                    other => panic!("expression not identifier. got = {:?}", other),
                }
            }
            other => panic!(
                "program.statements[0] is not ExpressionStatement. got = '{:?}'",
                other
            ),
        }
    }

    #[test]
    fn test_integer_literal_expression() {
        let input = "5;";

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();

        check_parser_erors(parser);

        assert_eq!(
            program.statements.len(),
            1,
            "program.statements does not contain enough statements. got = {}",
            program.statements.len()
        );

        match &program.statements[0] {
            StatementNode::Expression(exp) => {
                assert!(exp.expression.is_some());
                match exp.expression.as_ref().unwrap() {
                    ExpressionNode::Integer(integer) => {
                        assert_eq!(
                            integer.value, 5,
                            "integer.value not '5'. got = {}",
                            integer.value
                        );
                        assert_eq!(
                            integer.token_literal(),
                            "5",
                            "integer.value not '5'. got = {}",
                            integer.token_literal()
                        );
                    }
                    other => panic!("expression not an IntegerLiteral. got = {:?}", other),
                }
            }
            other => panic!("program.statements[0] not expression. got = {:?}", other),
        }
    }

    #[test]
    fn test_parsing_prefix_expressions() {
        let prefix_tests: Vec<(&str, &str, Box<dyn any::Any>)> = vec![
            ("!5", "!", Box::new(5)),
            ("-15", "-", Box::new(15)),
            ("!true", "!", Box::new(true)),
            ("!false", "!", Box::new(false)),
        ];
        for test in prefix_tests {
            let (input, prefix, value) = test;

            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program().unwrap();

            check_parser_erors(parser);

            assert_eq!(
                program.statements.len(),
                1,
                "program.statements does not contain enough statements. got = {}",
                program.statements.len()
            );

            match &program.statements[0] {
                StatementNode::Expression(exp) => {
                    assert!(exp.expression.is_some());
                    let expression = exp.expression.as_ref().unwrap();

                    match expression {
                        ExpressionNode::Prefix(pre) => {
                            assert_eq!(
                                pre.operator, prefix,
                                "prefix.operator not {}. got = {}",
                                prefix, pre.operator
                            );

                            test_literal_expression(&pre.right, value);
                        }
                        other => panic!("Expresion not PrefixExpression. got = {:?}", other),
                    }
                }
                other => panic!(
                    "program.statements[0] is not expression statement. got = {:?}",
                    other
                ),
            }
        }
    }

    #[test]
    fn test_parsing_infix_expressions() {
        let infix_tests: Vec<(&str, Box<dyn any::Any>, &str, Box<dyn any::Any>)> = vec![
            ("5 + 5;", Box::new(5), "+", Box::new(5)),
            ("5 - 5;", Box::new(5), "-", Box::new(5)),
            ("5 * 5;", Box::new(5), "*", Box::new(5)),
            ("5 / 5;", Box::new(5), "/", Box::new(5)),
            ("5 > 5;", Box::new(5), ">", Box::new(5)),
            ("5 < 5;", Box::new(5), "<", Box::new(5)),
            ("5 == 5;", Box::new(5), "==", Box::new(5)),
            ("5 != 5;", Box::new(5), "!=", Box::new(5)),
            ("true == true", Box::new(true), "==", Box::new(true)),
            ("true != false", Box::new(true), "!=", Box::new(false)),
            ("false == false", Box::new(false), "==", Box::new(false)),
        ];
        for test in infix_tests {
            let (input, left, operator, right) = test;

            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program().unwrap();

            check_parser_erors(parser);

            assert_eq!(
                program.statements.len(),
                1,
                "program.statements does not contain enough statements. got = {}",
                program.statements.len()
            );

            match &program.statements[0] {
                StatementNode::Expression(exp) => {
                    assert!(exp.expression.is_some());
                    let expression = exp.expression.as_ref().unwrap();

                    test_infix_expression(expression, left, operator.to_string(), right);
                }
                other => panic!(
                    "program.statements[0] is not expression statement. got = {:?}",
                    other
                ),
            }
        }
    }

    fn test_integer_literal(exp: &ExpressionNode, value: i64) {
        match exp {
            ExpressionNode::Integer(int) => {
                assert_eq!(
                    int.value, value,
                    "IntegerLiteral.value not {}. got = {}",
                    value, int.value
                );

                assert_eq!(
                    int.token_literal(),
                    format!("{}", value),
                    "IntegerLiteral.token_literal() not {}. got = {}",
                    value,
                    int.token_literal()
                );
            }

            other => panic!("expression not IntegerLiteral. got = {:?}", other),
        }
    }

    #[test]
    fn test_operator_precedence_parsing() {
        let tests = vec![
            ("-a * b", "((-a) * b)"),
            ("!-a", "(!(-a))"),
            ("a + b + c", "((a + b) + c)"),
            ("a + b - c", "((a + b) - c)"),
            ("a * b * c", "((a * b) * c)"),
            ("a * b / c", "((a * b) / c)"),
            ("a + b / c", "(a + (b / c))"),
            ("a + b * c + d / e - f", "(((a + (b * c)) + (d / e)) - f)"),
            ("3 + 4; -5 * 5", "(3 + 4)((-5) * 5)"),
            ("5 > 4 == 3 < 4", "((5 > 4) == (3 < 4))"),
            ("5 < 4 != 3 > 4", "((5 < 4) != (3 > 4))"),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            (
                "3 + 4 * 5 == 3 * 1 + 4 * 5",
                "((3 + (4 * 5)) == ((3 * 1) + (4 * 5)))",
            ),
            ("true", "true"),
            ("false", "false"),
            ("3 > 5 == false", "((3 > 5) == false)"),
            ("3 < 5 == true", "((3 < 5) == true)"),
            ("1 + (2 + 3) + 4", "((1 + (2 + 3)) + 4)"),
            ("(5 + 5) * 2", "((5 + 5) * 2)"),
            ("2 / (5 + 5)", "(2 / (5 + 5))"),
            ("-(5 + 5)", "(-(5 + 5))"),
            ("!(true == true)", "(!(true == true))"),
        ];

        for test in tests {
            let (input, value) = test;

            let lexer = Lexer::new(input);
            let mut parser = Parser::new(lexer);
            let program = parser.parse_program().unwrap();
            check_parser_erors(parser);

            let actual = program.print_string();
            assert_eq!(actual, value, "expected = {}, got = {}", value, actual);
        }
    }

    fn test_identifier(exp: &ExpressionNode, value: String) {
        match exp {
            ExpressionNode::IdentifierNode(ident) => {
                assert_eq!(
                    ident.value, value,
                    "ident.value not {}. got = {}",
                    value, ident.value
                );

                assert_eq!(
                    ident.token_literal(),
                    value,
                    "ident.token_literal not {}. got = {}",
                    value,
                    ident.token_literal()
                );
            }
            other => panic!("exp not Identifier. got = {:?}", other),
        }
    }

    fn test_literal_expression(exp: &ExpressionNode, expected: Box<dyn any::Any>) {
        match expected.downcast_ref::<String>() {
            Some(val) => test_identifier(exp, val.to_string()),
            None => match expected.downcast_ref::<i64>() {
                Some(int) => test_integer_literal(exp, int.to_owned()),
                None => match expected.downcast_ref::<bool>() {
                    Some(boolean) => {
                        test_boolean_literal(exp, boolean.to_owned());
                    }
                    None => (),
                },
            },
        }
    }

    fn test_boolean_literal(exp: &ExpressionNode, value: bool) {
        match exp {
            ExpressionNode::BooleanNode(boolean) => {
                assert_eq!(
                    boolean.value, value,
                    "boolean.value not {}. got = {}",
                    value, boolean.value
                );

                assert_eq!(
                    boolean.token_literal(),
                    format!("{}", value),
                    "boolean.token_literal not {}. got = {}",
                    value,
                    boolean.token_literal()
                );
            }
            other => panic!("exp is not a Boolean. got = {:?}", other),
        }
    }

    fn test_infix_expression(
        exp: &ExpressionNode,
        left: Box<dyn any::Any>,
        operator: String,
        right: Box<dyn any::Any>,
    ) {
        match exp {
            ExpressionNode::Infix(inf) => {
                test_literal_expression(&inf.left, left);
                assert_eq!(
                    inf.operator, operator,
                    "operator is not {}. got = {}",
                    operator, inf.operator
                );
                test_literal_expression(&inf.right, right);
            }
            other => panic!("exp is not an ExpressionNode. got = {:?}", other),
        }
    }

    #[test]
    fn test_boolean_expression() {
        let input = r#"
        true;
        false;
        "#;

        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program().unwrap();

        check_parser_erors(parser);

        assert_eq!(
            program.statements.len(),
            2,
            "program.statements does not contain enough statements. got = {}",
            program.statements.len()
        );

        let expected_values = vec![(TokenKind::True, "true"), (TokenKind::False, "false")];

        for (idx, test) in expected_values.into_iter().enumerate() {
            let (kind, value) = test;
            match &program.statements[idx] {
                StatementNode::Expression(stmt) => {
                    assert!(stmt.expression.is_some());
                    let exp = stmt.expression.as_ref().unwrap();
                    match exp {
                        ExpressionNode::BooleanNode(boolean) => {
                            assert_eq!(
                                boolean.token.kind, kind,
                                "statement[{}] token kind not {}. got = {}",
                                idx, kind, boolean.token.kind
                            );

                            assert_eq!(
                                boolean.token_literal(),
                                value,
                                "statement[{}] value not {}. got = {}",
                                idx,
                                value,
                                boolean.token_literal()
                            );
                        }
                        other => panic!("statement is not a BooleanNode. got = {:?}", other),
                    }
                }
                other => panic!(
                    "program.statements[{}] is not ExpressionStatement. got = {:?}",
                    idx, other
                ),
            }
        }
    }
}
