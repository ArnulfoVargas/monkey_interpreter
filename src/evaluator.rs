use crate::{
    ast::{BlockStatement, ExpressionNode, IfExpression, Program, StatementNode},
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

            if let Object::ReturnValue(ret) = result {
                return *ret;
            }
        }

        result
    }

    fn eval_statement(&self, stmt: StatementNode) -> Object {
        match stmt {
            StatementNode::Expression(exp) => self.eval_expression(exp.expression),
            StatementNode::Return(ret) => {
                let val = self.eval_expression(ret.ret_value);
                return Object::ReturnValue(Box::new(val));
            }
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
                ExpressionNode::If(expr) => self.eval_if_expression(expr),

                _ => NULL,
            };
        }

        Object::Null
    }

    fn eval_if_expression(&self, exp: IfExpression) -> Object {
        let condition = self.eval_expression(Some(*exp.condition));
        return if Self::is_truth(condition) {
            self.eval_block_statement(exp.consequence)
        } else if let Some(alt) = exp.alternative {
            self.eval_block_statement(alt)
        } else {
            NULL
        };
    }

    fn eval_block_statement(&self, block: BlockStatement) -> Object {
        let mut result = NULL;

        for stmt in block.statements {
            result = self.eval_statement(stmt);

            if let Object::ReturnValue(_) = result {
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
