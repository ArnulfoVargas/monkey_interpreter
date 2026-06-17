use crate::{
    ast::{ExpressionNode, Program, StatementNode},
    object::Object,
};

const TRUE: Object = Object::Boolean(true);
const FALSE: Object = Object::Boolean(false);
const NULL: Object = Object::Null;

pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Evaluator {
        Evaluator {}
    }

    pub fn eval_program(&self, program: Program) -> Object {
        let mut result = Object::Null;

        for stmt in program.statements {
            result = self.eval_statement(stmt);
        }

        result
    }

    fn eval_statement(&self, stmt: StatementNode) -> Object {
        match stmt {
            StatementNode::Expression(exp) => self.eval_expression(exp.expression),
            _ => Object::Null,
        }
    }

    fn eval_expression(&self, expression: Option<ExpressionNode>) -> Object {
        if let Some(exp) = expression {
            return match exp {
                ExpressionNode::Integer(int) => Object::Integer(int.value),
                ExpressionNode::BooleanNode(boolean) => {
                    Self::native_bool_to_boolean_object(boolean.value)
                }
                ExpressionNode::Prefix(pre) => {
                    let right = self.eval_expression(Some(*pre.right));
                    return Self::eval_prefix_expresion(pre.operator, right);
                }
                ExpressionNode::Infix(inf) => {
                    let left = self.eval_expression(Some(*inf.left));
                    let right = self.eval_expression(Some(*inf.right));

                    Self::eval_infix_expression(inf.operator, &left, &right)
                }

                _ => NULL,
            };
        }

        Object::Null
    }

    fn eval_infix_expression(operator: String, left: &Object, right: &Object) -> Object {
        return match (left, right, operator) {
            (Object::Integer(l), Object::Integer(r), op) => {
                Self::eval_integer_infix_expression(op, *l, *r)
            }
            (Object::Boolean(l), Object::Boolean(r), op) => {
                return match op.as_str() {
                    "==" => Self::native_bool_to_boolean_object(l == r),
                    "!=" => Self::native_bool_to_boolean_object(l != r),
                    _ => NULL,
                }
            }
            _ => NULL,
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
            _ => NULL,
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
            _ => Object::Null,
        }
    }

    fn native_bool_to_boolean_object(bool: bool) -> Object {
        return if bool { TRUE } else { FALSE };
    }
}

#[cfg(test)]
mod text {
    use super::*;
    use crate::{lexer::Lexer, object::Object, parser::Parser};

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

    fn test_eval(input: &str) -> Object {
        let lexer = Lexer::new(input);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let evaluator = Evaluator::new();

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
