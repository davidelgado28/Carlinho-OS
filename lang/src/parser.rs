use crate::ast::*;
use crate::lexer::Token;

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
            _ => None,
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

        Some(Statement::If {
            condition,
            consequence,
            elif_branches,
            alternative: None,
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
            self.advance();
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
        while self.peek() != Token::Eof && self.peek() != Token::Semicolon {
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
                }
            } else {
                break;
            }
        }
        Some(Statement::Cin { targets })
    }
    fn parse_expression(&mut self) -> Expr {
        Expr::Literal(Literal::Integer(0))
    }
}
