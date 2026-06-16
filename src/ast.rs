use crate::token::Token;

pub trait Node {
    fn token_literal(&self) -> String;
    fn print_string(&self) -> String;
}

#[derive(Debug)]
pub enum StatementNode {
    Let(LetStatement),
}

#[derive(Debug)]
pub enum ExpressionNode {
    IdentifierNode(Identifier),
}

impl Node for StatementNode {
    fn token_literal(&self) -> String {
        match self {
            Self::Let(stmt) => stmt.token_literal(),
        }
    }

    fn print_string(&self) -> String {
        match self {
            Self::Let(stmt) => stmt.print_string(),
        }
    }
}

impl Node for ExpressionNode {
    fn token_literal(&self) -> String {
        match self {
            Self::IdentifierNode(ident) => ident.token_literal(),
        }
    }

    fn print_string(&self) -> String {
        match self {
            Self::IdentifierNode(ident) => ident.print_string(),
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

#[derive(Debug)]
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

        out
    }
}

#[derive(Debug, Default)]
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
