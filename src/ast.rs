use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
    fn print_string(&self) -> String;
}

#[derive(Debug, Clone)]
pub enum StatementNode {
    Let(LetStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
    Block(BlockStatement),
}

#[derive(Debug, Clone, Default)]
pub enum ExpressionNode {
    #[default]
    None,
    IdentifierNode(Identifier),
    Integer(IntegerLiteral),
    Prefix(PrefixExpression),
    Infix(InfixExpression),
    BooleanNode(Boolean),
    If(IfExpression),
    Funcion(FunctionLiteral),
    Call(CallExpression),
}

impl Node for StatementNode {
    fn token_literal(&self) -> String {
        match self {
            Self::Let(stmt) => stmt.token_literal(),
            Self::Return(stmt) => stmt.token_literal(),
            Self::Expression(stmt) => stmt.token_literal(),
            Self::Block(stmt) => stmt.token_literal(),
        }
    }

    fn print_string(&self) -> String {
        match self {
            Self::Let(stmt) => stmt.print_string(),
            Self::Return(stmt) => stmt.print_string(),
            Self::Expression(stmt) => stmt.print_string(),
            Self::Block(stmt) => stmt.print_string(),
        }
    }
}

impl Node for ExpressionNode {
    fn token_literal(&self) -> String {
        match self {
            Self::IdentifierNode(ident) => ident.token_literal(),
            Self::Integer(ident) => ident.token_literal(),
            Self::Prefix(ident) => ident.token_literal(),
            Self::Infix(ident) => ident.token_literal(),
            Self::BooleanNode(ident) => ident.token_literal(),
            Self::If(ident) => ident.token_literal(),
            Self::Funcion(ident) => ident.token_literal(),
            Self::Call(ident) => ident.token_literal(),
            Self::None => String::from(""),
        }
    }

    fn print_string(&self) -> String {
        match self {
            Self::IdentifierNode(ident) => ident.print_string(),
            Self::Integer(ident) => ident.print_string(),
            Self::Prefix(ident) => ident.print_string(),
            Self::Infix(ident) => ident.print_string(),
            Self::BooleanNode(ident) => ident.print_string(),
            Self::If(ident) => ident.print_string(),
            Self::Funcion(ident) => ident.print_string(),
            Self::Call(ident) => ident.print_string(),
            Self::None => String::from(""),
        }
    }
}

pub struct Program {
    pub statements: Vec<StatementNode>,
}

impl Node for Program {
    fn token_literal(&self) -> String {
        return if self.statements.len() > 0 {
            match &self.statements[0] {
                StatementNode::Let(stmt) => stmt.token_literal(),
                StatementNode::Return(stmt) => stmt.token_literal(),
                StatementNode::Expression(stmt) => stmt.token_literal(),
                StatementNode::Block(stmt) => stmt.token_literal(),
            }
        } else {
            String::from("")
        };
    }

    fn print_string(&self) -> String {
        let mut out = String::from("");

        for stmt in self.statements.as_slice() {
            out.push_str(stmt.print_string().as_str());
        }

        out
    }
}

#[derive(Debug, Clone)]
pub struct LetStatement {
    pub token: Token,
    pub name: Identifier,
    pub value: Option<ExpressionNode>,
}

impl Node for LetStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn print_string(&self) -> String {
        let mut out = String::from("");

        out.push_str(self.token_literal().as_str());
        out.push_str(" ");
        out.push_str(self.name.print_string().as_str());
        out.push_str(" = ");

        if let Some(value) = &self.value {
            out.push_str(value.print_string().as_str());
        }
        out.push_str(";");

        out
    }
}

#[derive(Debug, Default, Clone)]
pub struct Identifier {
    pub token: Token,
    pub value: String,
}

impl Node for Identifier {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn print_string(&self) -> String {
        self.value.clone()
    }
}

#[derive(Debug, Default, Clone)]
pub struct ReturnStatement {
    pub token: Token,
    pub ret_value: Option<ExpressionNode>,
}

impl Node for ReturnStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn print_string(&self) -> String {
        let mut out = String::from("");

        out.push_str(self.token_literal().as_str());
        out.push_str(" ");

        if let Some(val) = &self.ret_value {
            out.push_str(val.print_string().as_str());
        }

        out.push_str(";");

        out
    }
}

#[derive(Debug, Default, Clone)]
pub struct ExpressionStatement {
    pub token: Token,
    pub expression: Option<ExpressionNode>,
}

impl Node for ExpressionStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn print_string(&self) -> String {
        if let Some(exp) = &self.expression {
            return exp.print_string();
        }

        String::from("")
    }
}

#[derive(Debug, Clone)]
pub struct IntegerLiteral {
    pub token: Token,
    pub value: i64,
}

impl Node for IntegerLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn print_string(&self) -> String {
        self.token_literal()
    }
}

#[derive(Debug, Clone, Default)]
pub struct PrefixExpression {
    pub token: Token,
    pub operator: String,
    pub right: Box<ExpressionNode>,
}

impl Node for PrefixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn print_string(&self) -> String {
        let mut out = String::from("");

        out.push_str("(");
        out.push_str(&self.operator.as_str());
        out.push_str(self.right.print_string().as_str());
        out.push_str(")");

        out
    }
}

#[derive(Debug, Default, Clone)]
pub struct InfixExpression {
    pub token: Token,
    pub left: Box<ExpressionNode>,
    pub operator: String,
    pub right: Box<ExpressionNode>,
}

impl Node for InfixExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn print_string(&self) -> String {
        let mut out = String::from("");

        out.push_str("(");
        out.push_str(self.left.print_string().as_str());
        out.push_str(format!(" {} ", self.operator).as_str());
        out.push_str(self.right.print_string().as_str());
        out.push_str(")");

        out
    }
}

#[derive(Debug, Clone, Default)]
pub struct Boolean {
    pub token: Token,
    pub value: bool,
}

impl Node for Boolean {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn print_string(&self) -> String {
        self.token_literal()
    }
}

#[derive(Debug, Default, Clone)]
pub struct IfExpression {
    pub token: Token,
    pub condition: Box<ExpressionNode>,
    pub consequence: BlockStatement,
    pub alternative: Option<BlockStatement>,
}

#[derive(Debug, Default, Clone)]
pub struct BlockStatement {
    pub token: Token,
    pub statements: Vec<StatementNode>,
}

impl Node for IfExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn print_string(&self) -> String {
        let mut out = String::from("");

        out.push_str("if");
        out.push_str(self.condition.print_string().as_str());
        out.push_str(" ");
        out.push_str(self.consequence.print_string().as_str());

        if let Some(alt) = &self.alternative {
            out.push_str("else ");
            out.push_str(alt.print_string().as_str());
        }

        out
    }
}

impl Node for BlockStatement {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn print_string(&self) -> String {
        let mut out = String::from("");

        for stmt in &self.statements {
            out.push_str(stmt.print_string().as_str());
        }

        out
    }
}

#[derive(Debug, Default, Clone)]
pub struct FunctionLiteral {
    pub token: Token,
    pub parameters: Vec<Identifier>,
    pub body: BlockStatement,
}

impl Node for FunctionLiteral {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn print_string(&self) -> String {
        let mut out = String::from("");
        let mut params = vec![];

        for param in &self.parameters {
            params.push(param.print_string());
        }

        out.push_str(self.token_literal().as_str());
        out.push_str("(");
        out.push_str(params.join(", ").as_str());
        out.push_str(")");
        out.push_str(self.body.print_string().as_str());

        out
    }
}

#[derive(Debug, Default, Clone)]
pub struct CallExpression {
    pub token: Token,
    pub function: Box<ExpressionNode>,
    pub arguments: Vec<ExpressionNode>,
}

impl Node for CallExpression {
    fn token_literal(&self) -> String {
        self.token.literal.clone()
    }

    fn print_string(&self) -> String {
        let mut out = String::from("");
        let mut args = vec![];

        for arg in &self.arguments {
            args.push(arg.print_string());
        }

        out.push_str(self.function.print_string().as_str());
        out.push_str("(");
        out.push_str(args.join(", ").as_str());
        out.push_str(")");

        out
    }
}

#[cfg(test)]
mod test {
    use crate::{
        ast::{ExpressionNode, Identifier, LetStatement, Node, Program, StatementNode},
        token::{Token, TokenKind},
    };
    #[test]
    fn test_print_string() {
        let program = Program {
            statements: vec![StatementNode::Let(LetStatement {
                token: Token {
                    kind: TokenKind::Let,
                    literal: String::from("let"),
                },
                name: Identifier {
                    token: Token {
                        kind: TokenKind::Ident,
                        literal: String::from("myVar"),
                    },
                    value: String::from("myVar"),
                },
                value: Some(ExpressionNode::IdentifierNode(Identifier {
                    token: Token {
                        kind: TokenKind::Ident,
                        literal: String::from("anotherVar"),
                    },
                    value: String::from("anotherVar"),
                })),
            })],
        };

        assert_eq!(
            program.print_string(),
            String::from("let myVar = anotherVar;"),
            "print string wrong. got = {}",
            program.print_string()
        )
    }
}
