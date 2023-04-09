use crate::lex::Token;

impl Token {
    fn return_val(&self) -> (&str, Option<String>) {
        match self {
            Token::Variable(x) => ("VARIABLE", Some(x.to_string())),
            Token::Figure(x) => ("FIGURE", Some(x.to_string())),
            Token::Arithmetic(x) if &*x == "+" || &*x == "-" => ("ADDORSUB", Some(x.to_string())),
            Token::Comparison(_) => todo!(),
            Token::Type(_) => todo!(),
            Token::VOID => todo!(),
            Token::WHILE => todo!(),
            Token::FOR => todo!(),
            Token::IF => todo!(),
            Token::ELSE => todo!(),
            Token::BREAK => todo!(),
            Token::CONTINUE => todo!(),
            Token::EQ => todo!(),
            Token::LP => todo!(),
            Token::RP => todo!(),
            Token::LCB => todo!(),
            Token::RCB => todo!(),
            Token::LSB => todo!(),
            Token::RSB => todo!(),
            Token::COMMA => todo!(),
            Token::SEMICOL => todo!(),
            Token::RETURN => todo!(),
            Token::None => todo!(),
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum Node {
    Number(i32),
    Variable(String),
    FuncName(String),
    Type(String),
    Branch {
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
        branch: String,
    },
    StatusNode,
    None,
}

pub struct ParseNode {
    node: Node,
    status: String,
    lock: bool,
}

impl ParseNode {
    fn new(token: Token) -> ParseNode {
        match token.return_val() {
            (x @ "VARIABLE", Some(value)) => ParseNode {
                node: Node::Variable(value),
                status: x.to_string(),
                lock: false,
            },
            (x @ "FIGURE", Some(value)) => ParseNode {
                node: Node::Number(value.parse().unwrap()),
                status: x.to_string(),
                lock: false,
            },
            (x @ "TYPE", Some(value)) => ParseNode {
                node: Node::Type(value.parse().unwrap()),
                status: x.to_string(),
                lock: false,
            },
            (x @ _, Some(value)) => ParseNode {
                node: Node::Branch {
                    left: None,
                    right: None,
                    branch: (value),
                },
                status: x.to_string(),
                lock: false,
            },
            (x @ _, None) => ParseNode {
                node: Node::StatusNode,
                status: x.to_string(),
                lock: false,
            },
        }
    }
}
