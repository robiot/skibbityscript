// parser.rs
use crate::{
    error,
    lexer::{Lexer, Token},
};

#[derive(Clone, Debug)]
pub enum Expr {
    Ident(String),
    Number(i64),
    StringLiteral(String),
    Boolean(bool),
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
    BinOp {
        left: Box<Expr>,
        op: String,
        right: Box<Expr>,
    },
}

#[derive(Clone, Debug)]
pub enum Stmt {
    Function {
        name: String,
        body: Vec<Stmt>,
        line: usize,
    },
    VariableAssign {
        name: String,
        value: Expr,
        line: usize,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
        line: usize,
    },
    If {
        condition: Expr,
        then_branch: Vec<Stmt>,
        else_branch: Option<Vec<Stmt>>,
        line: usize,
    },
    Expression {
        value: Expr,
        line: usize,
    },
    Return {
        value: Expr,
        line: usize,
    },
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer) -> Result<Self, error::ParseError> {
        let current_token = lexer.next_token()?;

        Ok(Parser {
            lexer,
            current_token,
        })
    }

    fn next_token(&mut self) -> Result<(), error::ParseError> {
        self.current_token = self.lexer.next_token()?;

        println!("next_token: {:?}", self.current_token);

        Ok(())
    }

    fn expect_token(&mut self, expected: Token) -> Result<(), error::ParseError> {
        if self.current_token == expected {
            self.next_token()?;

            Ok(())
        } else {
            Err(error::ParseError::UnexpectedToken {
                expected: expected.clone(),
                found: self.current_token.clone(),
                line: self.lexer.line,
            })
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, error::ParseError> {
        let mut statements = Vec::new();
        while self.current_token != Token::EOF {
            // ...debug
            println!("current_token: {:?}", self.current_token);
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Stmt, error::ParseError> {       
        match self.current_token {
            Token::Cookable => self.parse_function(),
            Token::Ident(ref ident) if ident == "score" => self.parse_increment(),
            Token::Ident(_) => self.parse_variable_assign_or_expression(),
            Token::Skibidi => self.parse_while(),
            Token::Suspect => self.parse_if(),
            Token::Blud => self.parse_return(),
            _ => Err(error::ParseError::UnknownUnexpectedToken {
                found: self.current_token.clone(),
                line: self.lexer.line,
            }),
        }
    }

    fn parse_function(&mut self) -> Result<Stmt, error::ParseError> {
        self.expect_token(Token::Cookable)?;
        let name = if let Token::Ident(ident) = &self.current_token {
            Ok(ident.clone())
        } else {
            Err(error::ParseError::GeneralError {
                line: self.lexer.line,
                message: format!(
                    "found token: {:?}, expected function name",
                    self.current_token.clone()
                ),
            })
        }?;

        self.next_token()?;
        self.expect_token(Token::LeftParen)?;
        self.expect_token(Token::RightParen)?;

        let mut body = Vec::new();
        while self.current_token != Token::Slay && self.current_token != Token::EOF {
            body.push(self.parse_statement()?);
        }

        self.expect_token(Token::Slay)?;

        Ok(Stmt::Function {
            name,
            body,
            line: self.lexer.line,
        })
    }

    fn parse_increment(&mut self) -> Result<Stmt, error::ParseError> {
        let name = if let Token::Ident(ident) = &self.current_token {
            Ok(ident.clone())
        } else {
            Err(error::ParseError::Other("Expected variable name".into()))
        }?;
        self.next_token()?;
        self.expect_token(Token::Is)?;
        self.expect_token(Token::Ident("more".into()))?;
        self.expect_token(Token::Ident(name.clone()))?;

        Ok(Stmt::VariableAssign {
            name: name.clone(),
            value: Expr::BinOp {
                left: Box::new(Expr::Ident(name.clone())),
                op: "+".into(),
                right: Box::new(Expr::Number(1)),
            },
            line: self.lexer.line,
        })
    }

    fn parse_variable_assign_or_expression(&mut self) -> Result<Stmt, error::ParseError> {
        let name = if let Token::Ident(ident) = &self.current_token {
            Ok(ident.clone())
        } else {
            Err(error::ParseError::Other("Expected identifier".into()))
        }?;

        self.next_token()?;

        if self.current_token == Token::Is {
            self.next_token()?;

            let value = self.parse_expression()?;

            Ok(Stmt::VariableAssign {
                name,
                value,
                line: self.lexer.line,
            })
        } else {
            let expr = self.parse_expression()?;

            Ok(Stmt::Expression {
                value: expr,
                line: self.lexer.line,
            })
        }
    }

    fn parse_expression(&mut self) -> Result<Expr, error::ParseError> {
        match &self.current_token {
            Token::Ident(ident) => {
                let name = ident.clone();
                self.next_token()?;

                if self.current_token == Token::LeftParen {
                    self.next_token()?;
                    let mut args = Vec::new();

                    if self.current_token != Token::RightParen {
                        args.push(self.parse_expression()?);
                        while self.current_token == Token::Comma {
                            self.next_token()?;
                            args.push(self.parse_expression()?);
                        }
                    }
                    self.expect_token(Token::RightParen)?;

                    Ok(Expr::FunctionCall { name, args })
                } else {
                    Ok(Expr::Ident(name))
                }
            }
            Token::Number(num) => {
                let value = *num;
                self.next_token()?;

                Ok(Expr::Number(value))
            }
            Token::StringLiteral(string) => {
                let value = string.clone();
                self.next_token()?;
                Ok(Expr::StringLiteral(value))
            }
            Token::Sigma => {
                self.next_token()?;
                Ok(Expr::Boolean(true))
            }
            Token::Ohio => {
                self.next_token()?;
                Ok(Expr::Boolean(false))
            }
            _ => Err(error::ParseError::UnknownUnexpectedToken {
                found: self.current_token.clone(),
                line: self.lexer.line,
            }),
        }
    }

    fn parse_while(&mut self) -> Result<Stmt, error::ParseError> {
        self.expect_token(Token::Skibidi)?;
        self.expect_token(Token::LeftParen)?;

        let condition = self.parse_expression()?;

        self.expect_token(Token::RightParen)?;

        let mut body = Vec::new();

        while self.current_token != Token::Slay && self.current_token != Token::EOF {
            body.push(self.parse_statement()?);
        }

        self.expect_token(Token::Slay)?;

        Ok(Stmt::While {
            condition,
            body,
            line: self.lexer.line,
        })
    }

    fn parse_if(&mut self) -> Result<Stmt, error::ParseError> {
        self.expect_token(Token::Suspect)?;
        self.expect_token(Token::LeftParen)?;
        let condition = self.parse_expression()?;
        self.expect_token(Token::RightParen)?;
        self.expect_token(Token::Then)?;
        let mut then_branch = Vec::new();

        while self.current_token != Token::Ick
            && self.current_token != Token::Slay
            && self.current_token != Token::EOF
        {
            then_branch.push(self.parse_statement()?);
        }

        let else_branch = if self.current_token == Token::Ick {
            self.next_token()?;
            let mut else_stmts = Vec::new();
            while self.current_token != Token::Slay && self.current_token != Token::EOF {
                else_stmts.push(self.parse_statement()?);
            }
            Some(else_stmts)
        } else {
            None
        };

        self.expect_token(Token::Slay)?;

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
            line: self.lexer.line,
        })
    }

    fn parse_return(&mut self) -> Result<Stmt, error::ParseError> {
        self.expect_token(Token::Blud)?;
        let expr = self.parse_expression()?;

        Ok(Stmt::Return {
            value: expr,
            line: self.lexer.line,
        })
    }
}
