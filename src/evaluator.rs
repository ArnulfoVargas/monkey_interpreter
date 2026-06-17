use std::ops::Deref;

use crate::{
    ast::{BlockStatement, ExpressionNode, Identifier, IfExpression, Program, StatementNode},
    object::{Environment, Function, Object},
};

const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);
const NULL: Object = Object::Null;

pub struct Evaluator {
    env: Environment,
}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {
            env: Environment::new(),
        }
    }

    pub fn eval_program(&mut self, program: Program) -> Object {
        let mut result = Object::Null;

        for stmt in program.statements {
            result = self.eval_statement(stmt);

            if let Object::ReturnValue(ret) = result {
                return *ret;
            }

            if Self::is_error(&result) {
                return result;
            }
        }

        result
    }

    fn eval_statement(&mut self, stmt: StatementNode) -> Object {
        match stmt {
            StatementNode::Expression(exp) => self.eval_expression(exp.expression),
            StatementNode::Return(ret) => {
                let val = self.eval_expression(ret.ret_value);

                if Self::is_error(&val) {
                    return val;
                }

                return Object::ReturnValue(Box::new(val));
            }
            StatementNode::Let(stmt) => {
                let value = self.eval_expression(stmt.value);

                if Self::is_error(&value) {
                    return value;
                }

                self.env.set(stmt.name.value, value).unwrap()
            }
            _ => Object::Null,
        }
    }

    fn eval_expression(&mut self, expression: Option<ExpressionNode>) -> Object {
        if let Some(exp) = expression {
            return match exp {
                ExpressionNode::Integer(int) => Object::Integer(int.value),
                ExpressionNode::BooleanNode(boolean) => {
                    Self::native_bool_to_boolean_object(boolean.value)
                }
                ExpressionNode::Prefix(pre) => {
                    let right = self.eval_expression(Some(*pre.right));

                    if Self::is_error(&right) {
                        return right;
                    }

                    return Self::eval_prefix_expresion(pre.operator, right);
                }
                ExpressionNode::Infix(inf) => {
                    let left = self.eval_expression(Some(*inf.left));

                    if Self::is_error(&left) {
                        return left;
                    }

                    let right = self.eval_expression(Some(*inf.right));

                    if Self::is_error(&right) {
                        return right;
                    }

                    Self::eval_infix_expression(inf.operator, &left, &right)
                }
                ExpressionNode::If(expr) => self.eval_if_expression(expr),
                ExpressionNode::IdentifierNode(ident) => self.eval_identifier(ident),
                ExpressionNode::Funcion(func) => Object::Function(Function {
                    parameters: func.parameters,
                    body: func.body,
                    env: self.env.clone(),
                }),
                ExpressionNode::Call(call) => {
                    let func = self.eval_expression(Some(call.function.deref().clone()));

                    if Self::is_error(&func) {
                        return func;
                    }

                    let args = self.eval_expressions(call.arguments);

                    if args.len() == 1 && Self::is_error(&args[0]) {
                        return args[0].clone();
                    }

                    self.apply_function(func, args)
                }

                _ => NULL,
            };
        }

        Object::Null
    }

    fn apply_function(&mut self, func: Object, args: Vec<Object>) -> Object {
        return match func {
            Object::Function(func) => {
                let old_env = self.env.clone();
                let ext_env = self.extended_function_env(func.clone(), args);

                self.env = ext_env;
                let eval = self.eval_block_statement(func.body);

                self.env = old_env;
                return Self::unwrap_retrun_value(eval);
            }
            other => Object::Error(format!("not a function: {}", other)),
        };
    }

    fn unwrap_retrun_value(obj: Object) -> Object {
        return match obj {
            Object::ReturnValue(val) => *val,
            _ => obj,
        };
    }

    fn extended_function_env(&mut self, func: Function, args: Vec<Object>) -> Environment {
        let mut env = Environment::new_enclosed_environment(Box::new(func.env));

        for (idx, param) in func.parameters.into_iter().enumerate() {
            env.set(param.value, args[idx].clone());
        }

        env
    }

    fn eval_expressions(&mut self, expressions: Vec<ExpressionNode>) -> Vec<Object> {
        let mut result = vec![];

        for exp in expressions {
            let eval = self.eval_expression(Some(exp));

            if Self::is_error(&eval) {
                return vec![eval];
            }

            result.push(eval);
        }

        result
    }

    fn eval_identifier(&self, identifier: Identifier) -> Object {
        let val = self.env.get(identifier.value.clone());
        return match val {
            Some(v) => v,
            None => Object::Error(format!("identifier not found: {}", identifier.value)),
        };
    }

    fn eval_if_expression(&mut self, exp: IfExpression) -> Object {
        let condition = self.eval_expression(Some(*exp.condition));
        return if Self::is_truth(condition) {
            self.eval_block_statement(exp.consequence)
        } else if let Some(alt) = exp.alternative {
            self.eval_block_statement(alt)
        } else {
            NULL
        };
    }

    fn eval_block_statement(&mut self, block: BlockStatement) -> Object {
        let mut result = NULL;

        for stmt in block.statements {
            result = self.eval_statement(stmt);

            if let Object::ReturnValue(_) = result {
                return result;
            }

            if Self::is_error(&result) {
                return result;
            }
        }

        result
    }

    fn is_truth(obj: Object) -> bool {
        match obj {
            Object::Null => false,
            Object::Boolean(b) => b,
            Object::Integer(i) => i != 0,
            _ => true,
        }
    }

    fn eval_infix_expression(operator: String, left: &Object, right: &Object) -> Object {
        if left.object_type() != right.object_type() {
            return Object::Error(format!(
                "type mismatch: {} {} {}",
                left.object_type(),
                operator,
                right.object_type()
            ));
        }
        return match (left, right, operator) {
            (Object::Integer(l), Object::Integer(r), op) => {
                Self::eval_integer_infix_expression(op, *l, *r)
            }
            (Object::Boolean(l), Object::Boolean(r), op) => {
                return match op.as_str() {
                    "==" => Self::native_bool_to_boolean_object(l == r),
                    "!=" => Self::native_bool_to_boolean_object(l != r),
                    _ => Object::Error(format!(
                        "unknown operator: {} {} {}",
                        left.object_type(),
                        op,
                        right.object_type()
                    )),
                }
            }
            (l, r, op) => Object::Error(format!(
                "unknown operator: {} {} {}",
                l.object_type(),
                op,
                r.object_type()
            )),
        };
    }

    fn eval_integer_infix_expression(op: String, left: i64, right: i64) -> Object {
        return match op.as_str() {
            "+" => Object::Integer(left + right),
            "-" => Object::Integer(left - right),
            "*" => Object::Integer(left * right),
            "/" => Object::Integer(left / right),
            "<" => Self::native_bool_to_boolean_object(left < right),
            ">" => Self::native_bool_to_boolean_object(left > right),
            "==" => Self::native_bool_to_boolean_object(left == right),
            "!=" => Self::native_bool_to_boolean_object(left != right),
            _ => NULL,
        };
    }

    fn eval_prefix_expresion(operator: String, right: Object) -> Object {
        match operator.as_str() {
            "!" => Self::eval_bang_operator(right),
            "-" => Self::eval_minus_operator(right),
            _ => Object::Error(format!(
                "unknown operator: {}{}",
                operator,
                right.object_type()
            )),
        }
    }

    fn eval_bang_operator(right: Object) -> Object {
        match right {
            Object::Boolean(true) => FALSE,
            Object::Boolean(false) => TRUE,
            Object::Null => TRUE,
            Object::Integer(i) => return if i != 0 { FALSE } else { TRUE },
            _ => FALSE,
        }
    }

    fn eval_minus_operator(right: Object) -> Object {
        match right {
            Object::Integer(int) => Object::Integer(-int),
            _ => Object::Error(format!("unknown operator: -{}", right.object_type())),
        }
    }

    fn native_bool_to_boolean_object(bool: bool) -> Object {
        return if bool { TRUE } else { FALSE };
    }

    fn is_error(obj: &Object) -> bool {
        return if let Object::Error(_) = obj {
            true
        } else {
            false
        };
    }
}

#[cfg(test)]
mod text {
    use super::*;
    use crate::{ast::Node, lexer::Lexer, object::Object, parser::Parser};

    #[test]
    fn test_eval_integer_expr() {
        let tests = vec![
            ("5", 5),
            ("10", 10),
            ("-5", -5),
            ("-10", -10),
            ("5 + 5 + 5 + 5 - 10", 10),
            ("2 * 2 * 2 * 2 * 2", 32),
            ("-50 + 100 + -50", 0),
            ("5 * 2 + 10", 20),
            ("5 + 2 * 10", 25),
            ("20 + 2 * -10", 0),
            ("50 / 2 * 2 + 10", 60),
            ("2 * (5 + 10)", 30),
            ("3 * 3 * 3 + 10", 37),
            ("3 * (3 * 3) + 10", 37),
            ("(5 + 10 * 2 + 15 / 3) * 2 + -10", 50),
        ];
        for test in tests {
            let (input, val) = test;

            let eval = test_eval(input);
            test_integer_object(eval, val);
        }
    }

    #[test]
    fn test_eval_boolean_expr() {
        let tests = vec![
            ("true", true),
            ("false", false),
            ("1 < 2", true),
            ("1 > 2", false),
            ("1 > 1", false),
            ("1 == 1", true),
            ("1 != 1", false),
            ("1 == 2", false),
            ("1 != 2", true),
            ("true == true", true),
            ("false == false", true),
            ("true == false", false),
            ("true != false", true),
            ("false != true", true),
            ("(1 < 2) == true", true),
            ("(1 < 2) == false", false),
            ("(1 > 2) == true", false),
            ("(1 > 2) == false", true),
            ("(10 * 10 > 5) == (5 * 5 == 25)", true),
            ("(10 * 10 > 5) != (5 * 5 == 25)", false),
            ("(10 * 10 > 123) == (5 * 5 == 25)", false),
            ("(10 * 10 > 123) != (5 * 5 == 25)", true),
        ];
        for test in tests {
            let (input, val) = test;

            let evaluated = test_eval(input);
            test_boolean_object(evaluated, val);
        }
    }

    #[test]
    fn test_bang_operator() {
        let tests = vec![
            ("!true", false),
            ("!false", true),
            ("!5", false),
            ("!!true", true),
            ("!!false", false),
            ("!!5", true),
            ("!0", true),
            ("!1", false),
        ];

        for test in tests {
            let (input, val) = test;

            let evaluated = test_eval(input);
            test_boolean_object(evaluated, val);
        }
    }

    #[test]
    fn test_if_else_expression() {
        let tests = vec![
            ("if (true) { 10 }", Some(10)),
            ("if (false) { 10 }", None),
            ("if (1) { 10 }", Some(10)),
            ("if (1 < 2) { 10 }", Some(10)),
            ("if (1 > 2) { 10 }", None),
            ("if (1 > 2) { 10 } else { 20 }", Some(20)),
            ("if (1 < 2) { 10 } else { 20 }", Some(10)),
        ];
        for test in tests {
            let (input, value) = test;
            let evaluated = test_eval(input);

            if let Some(int) = value {
                test_integer_object(evaluated, int);
            } else {
                test_null_object(evaluated);
            }
        }
    }

    #[test]
    fn test_return_statements() {
        let tests = vec![
            ("return 10", 10),
            ("return 10; 9;", 10),
            ("return 2 * 5; 9;", 10),
            ("9; return 2 * 5; 9;", 10),
            (
                "if (10 > 1) {
                if (10 > 1) {
                    return 10;
                }
                return 1;
            }",
                10,
            ),
        ];

        for test in tests {
            let evaluated = test_eval(test.0);
            test_integer_object(evaluated, test.1);
        }
    }

    #[test]
    fn test_error_handling() {
        let tests = vec![
            ("5 + true;", "type mismatch: int + bool"),
            ("5 + true; 5;", "type mismatch: int + bool"),
            ("-true", "unknown operator: -bool"),
            ("true + false;", "unknown operator: bool + bool"),
            ("5; true + false; 5", "unknown operator: bool + bool"),
            (
                "if (10 > 1) { true + false; }",
                "unknown operator: bool + bool",
            ),
            (
                "if (10 > 1) {
                if (10 > 1) {
                return true + false;
                }
                return 1;
                }
                ",
                "unknown operator: bool + bool",
            ),
            ("foobar", "identifier not found: foobar"),
        ];

        for test in tests {
            let evaluated = test_eval(test.0);
            match evaluated {
                Object::Error(err) => assert_eq!(err, test.1),
                other => panic!("no error object returned. got = {:?}", other),
            }
        }
    }

    #[test]
    fn test_let_statements() {
        let tests = vec![
            ("let a = 5; a;", 5),
            ("let a = 5 * 5; a;", 25),
            ("let a = 5; let b = a; b;", 5),
            ("let a = 5; let b = a; let c = a + b + 5; c;", 15),
        ];

        for test in tests {
            let (input, value) = test;
            test_integer_object(test_eval(input), value);
        }
    }

    #[test]
    fn test_function_object() {
        let input = "fn(x) { x + 2; }";
        let evaluated = test_eval(input);

        match evaluated {
            Object::Function(function) => {
                assert_eq!(
                    function.parameters.len(),
                    1,
                    "function has wrong parameters length. got = {}",
                    function.parameters.len()
                );
                assert_eq!(
                    function.parameters[0].print_string(),
                    "x",
                    "parameter is not `x`, got = {}",
                    function.parameters[0].print_string()
                );
                assert_eq!(
                    function.body.print_string(),
                    "(x + 2)",
                    "body is not `(x + 2)`. got = {}",
                    function.body.print_string()
                );
            }
            other => panic!("object is not Function. got = {:?}", other),
        }
    }

    #[test]
    fn test_function_application() {
        let tests = vec![
            ("let identity = fn(x) { x; }; identity(5);", 5),
            ("let identity = fn(x) { return x; }; identity(5);", 5),
            ("let double = fn(x) { x * 2; }; double(5);", 10),
            ("let add = fn(x, y) { x + y; }; add(5, 5);", 10),
            ("let add = fn(x, y) { x+ y; }; add(5 + 5, add(5, 5));", 20),
            ("fn(x) { x; }(5)", 5),
        ];

        for test in tests {
            test_integer_object(test_eval(test.0), test.1);
        }
    }

    fn test_null_object(obj: Object) {
        match obj {
            Object::Null => assert!(true),
            _ => assert!(false),
        }
    }

    fn test_eval(input: &str) -> Object {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new();

        evaluator.eval_program(program.unwrap())
    }

    fn test_integer_object(obj: Object, value: i64) {
        match obj {
            Object::Integer(int) => assert_eq!(
                int, value,
                "object has wrong value, expected '{}'. got = {}",
                value, int
            ),
            other => panic!("object is not integer. got = {:?}", other),
        }
    }

    fn test_boolean_object(obj: Object, value: bool) {
        match obj {
            Object::Boolean(boolean) => assert_eq!(
                boolean, value,
                "object has wrong value, expected '{}'. got = {}",
                value, boolean
            ),
            other => panic!("object is not boolean. got = {:?}", other),
        }
    }
}
