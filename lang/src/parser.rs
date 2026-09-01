use crate::ast::*;
use crate::lexer::Token;

#[derive(PartialEq, PartialOrd, Debug, Clone, Copy)]
pub enum Precedence {
    Lowest,
    Sum,
    Product,
    Call,
}

pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, position: 0 }
    }

    fn peek(&self) -> Token {
        self.tokens.get(self.position).cloned().unwrap_or(Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let tok = self.peek();
        self.position += 1;
        tok
    }

    fn expect(&mut self, expected: Token) -> bool {
        if self.peek() == expected {
            self.advance();
            true
        } else {
            false
        }
    }

    pub fn parse_statement(&mut self) -> Option<Statement> {
        match self.peek() {
            Token::Int | Token::Double | Token::Char | Token::String | Token::Bool | Token::Long => {
                self.parse_var_decl()
            }
            Token::Ident(ref name) => {
                let id = name.clone();
                self.advance();
                if self.peek() == Token::Assign {
                    self.advance();
                    let value = self.parse_expression();
                    Some(Statement::Assignment { name: id, value })
                } else if self.peek() == Token::LeftParen {
                    self.advance();
                    let args = self.parse_argument_list();
                    Some(Statement::Expression(Expr::FunctionCall { name: id, args }))
                } else {
                    Some(Statement::Expression(Expr::Ident(id)))
                }
            }
            Token::If => self.parse_if_stmt(),
            Token::Match => self.parse_match_stmt(),
            Token::For => self.parse_for_stmt(),
            Token::While => self.parse_while_stmt(),
            Token::Cout => self.parse_cout_stmt(),
            Token::Cin => self.parse_cin_stmt(),
            Token::Return => {
                self.advance();
                let expr = if self.peek() != Token::RightBrace && self.peek() != Token::Eof {
                    Some(self.parse_expression())
                } else {
                    None
                };
                Some(Statement::Return(expr))
            }
            _ => {
                self.advance();
                None
            }
        }
    }

    fn parse_type(&mut self) -> Option<Type> {
        match self.advance() {
            Token::Int => Some(Type::Int),
            Token::Double => Some(Type::Double),
            Token::Char => Some(Type::Char),
            Token::String => Some(Type::String),
            Token::Bool => Some(Type::Bool),
            Token::Long => Some(Type::LongLong),
            _ => None,
        }
    }

    fn parse_var_decl(&mut self) -> Option<Statement> {
        let ty = self.parse_type()?;
        let name = match self.advance() {
            Token::Ident(n) => n,
            _ => return None,
        };

        let value = if self.peek() == Token::Assign {
            self.advance();
            Some(self.parse_expression())
        } else {
            None
        };

        Some(Statement::VarDecl { ty, name, value })
    }

    fn parse_if_stmt(&mut self) -> Option<Statement> {
        self.advance();
        self.expect(Token::LeftParen);
        let condition = self.parse_expression();
        self.expect(Token::RightParen);
        self.expect(Token::LeftBrace);
        
        let mut consequence = Vec::new();
        while self.peek() != Token::RightBrace && self.peek() != Token::Eof {
            if let Some(stmt) = self.parse_statement() {
                consequence.push(stmt);
            }
        }
        self.expect(Token::RightBrace);

        let mut elif_branches = Vec::new();
        while self.peek() == Token::Elif {
            self.advance();
            self.expect(Token::LeftParen);
            let elif_cond = self.parse_expression();
            self.expect(Token::RightParen);
            self.expect(Token::LeftBrace);
            
            let mut elif_body = Vec::new();
            while self.peek() != Token::RightBrace && self.peek() != Token::Eof {
                if let Some(stmt) = self.parse_statement() {
                    elif_body.push(stmt);
                }
            }
            self.expect(Token::RightBrace);
            elif_branches.push((elif_cond, elif_body));
        }

        let alternative = if self.peek() == Token::Else {
            self.advance();
            self.expect(Token::LeftBrace);
            let mut else_body = Vec::new();
            while self.peek() != Token::RightBrace && self.peek() != Token::Eof {
                if let Some(stmt) = self.parse_statement() {
                    else_body.push(stmt);
                }
            }
            self.expect(Token::RightBrace);
            Some(else_body)
        } else {
            None
        };

        Some(Statement::If {
            condition,
            consequence,
            elif_branches,
            alternative,
        })
    }

    fn parse_match_stmt(&mut self) -> Option<Statement> {
        self.advance();
        self.expect(Token::LeftParen);
        let value = self.parse_expression();
        self.expect(Token::RightParen);
        self.expect(Token::LeftBrace);

        let mut cases = Vec::new();
        while self.peek() != Token::RightBrace && self.peek() != Token::Eof {
            if self.peek() == Token::Case {
                self.advance();
                let pattern = match self.peek() {
                    Token::Ident(ref s) if s == "_" => {
                        self.advance();
                        MatchPattern::Wildcard
                    }
                    Token::Int(val) => {
                        self.advance();
                        MatchPattern::Literal(Literal::Integer(val))
                    }
                    _ => return None,
                };
                self.expect(Token::Colon);

                let mut body = Vec::new();
                while self.peek() != Token::Case && self.peek() != Token::RightBrace && self.peek() != Token::Eof {
                    if let Some(stmt) = self.parse_statement() {
                        body.push(stmt);
                    }
                }
                cases.push(MatchCase::Case { pattern, body });
            } else {
                self.advance();
            }
        }
        self.expect(Token::RightBrace);

        Some(Statement::Match { value, cases })
    }

    fn parse_for_stmt(&mut self) -> Option<Statement> {
        self.advance();
        self.expect(Token::LeftParen);
        let init = Box::new(self.parse_var_decl()?);
        self.expect(Token::Comma);
        let condition = self.parse_expression();
        self.expect(Token::Comma);
        let update = self.parse_expression();
        self.expect(Token::RightParen);
        self.expect(Token::LeftBrace);

        let mut body = Vec::new();
        while self.peek() != Token::RightBrace && self.peek() != Token::Eof {
            if let Some(stmt) = self.parse_statement() {
                body.push(stmt);
            }
        }
        self.expect(Token::RightBrace);

        Some(Statement::For { init, condition, update, body })
    }

    fn parse_while_stmt(&mut self) -> Option<Statement> {
        self.advance();
        self.expect(Token::LeftParen);
        let condition = self.parse_expression();
        self.expect(Token::RightParen);
        self.expect(Token::LeftBrace);

        let mut body = Vec::new();
        while self.peek() != Token::RightBrace && self.peek() != Token::Eof {
            if let Some(stmt) = self.parse_statement() {
                body.push(stmt);
            }
        }
        self.expect(Token::RightBrace);

        Some(Statement::While { condition, body })
    }

    fn parse_cout_stmt(&mut self) -> Option<Statement> {
        self.advance();
        let mut values = Vec::new();
        while self.peek() != Token::Eof {
            if self.peek() == Token::ShiftLeft {
                self.advance();
                values.push(self.parse_expression());
            } else {
                break;
            }
        }
        Some(Statement::Cout { values })
    }

    fn parse_cin_stmt(&mut self) -> Option<Statement> {
        self.advance();
        let mut targets = Vec::new();
        while self.peek() != Token::Eof {
            if self.peek() == Token::ShiftRight {
                self.advance();
                if let Token::Ident(name) = self.advance() {
                    targets.push(name);
                } else {
                    return None;
                }
            } else {
                break;
            }
        }
        Some(Statement::Cin { targets })
    }

    pub fn parse_expression(&mut self) -> Expr {
        self.parse_precedence(Precedence::Lowest)
    }

    fn parse_precedence(&mut self, precedence: Precedence) -> Expr {
        let mut left = self.parse_prefix();

        while precedence < self.peek_precedence() {
            let op_token = self.peek();
            if !self.is_infix_token(&op_token) {
                return left;
            }
            self.advance();
            left = self.parse_infix(left, op_token);
        }

        left
    }

    fn parse_prefix(&mut self) -> Expr {
        match self.advance() {
            Token::Int(val) => Expr::Literal(Literal::Integer(val)),
            Token::Ident(name) => {
                if self.peek() == Token::LeftParen {
                    self.advance();
                    let args = self.parse_argument_list();
                    Expr::FunctionCall { name, args }
                } else {
                    Expr::Ident(name)
                }
            }
            Token::LeftParen => {
                let expr = self.parse_expression();
                self.expect(Token::RightParen);
                expr
            }
            _ => Expr::Literal(Literal::Integer(0)),
        }
    }

    fn parse_infix(&mut self, left: Expr, token: Token) -> Expr {
        let op = match token {
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Star => "*".to_string(),
            Token::Slash => "/".to_string(),
            _ => return left,
        };

        let precedence = self.token_precedence(&token);
        let right = self.parse_precedence(precedence);

        Expr::BinaryOp {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn parse_argument_list(&mut self) -> Vec<Expr> {
        let mut args = Vec::new();
        if self.peek() == Token::RightParen {
            self.advance();
            return args;
        }
        args.push(self.parse_expression());
        while self.peek() == Token::Comma {
            self.advance();
            args.push(self.parse_expression());
        }
        self.expect(Token::RightParen);
        args
    }

    fn token_precedence(&self, token: &Token) -> Precedence {
        match token {
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Star | Token::Slash => Precedence::Product,
            Token::LeftParen => Precedence::Call,
            _ => Precedence::Lowest,
        }
    }

    fn peek_precedence(&self) -> Precedence {
        self.token_precedence(&self.peek())
    }

    fn is_infix_token(&self, token: &Token) -> bool {
        matches!(token, Token::Plus | Token::Minus | Token::Star | Token::Slash)
    }
}
