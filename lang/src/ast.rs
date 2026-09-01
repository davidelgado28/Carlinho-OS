#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Double,
    Char,
    String,
    Bool,
    LongLong,
    Void,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Char(char),
    Boolean(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Ident(String),
    Literal(Literal),
    BinaryOp {
        op: String,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VarDecl {
        ty: Type,
        name: String,
        value: Option<Expr>,
    },
    Assignment {
        name: String,
        value: Expr,
    },
    If {
        condition: Expr,
        consequence: Vec<Statement>,
        elif_branches: Vec<(Expr, Vec<Statement>)>,
        alternative: Option<Vec<Statement>>,
    },
    Match {
        value: Expr,
        cases: Vec<MatchCase>,
    },
    For {
        init: Box<Statement>,
        condition: Expr,
        update: Expr,
        body: Vec<Statement>,
    },
    While {
        condition: Expr,
        body: Vec<Statement>,
    },
    DoWhile {
        body: Vec<Statement>,
        condition: Expr,
    },
    Cout {
        values: Vec<Expr>,
    },
    Cin {
        targets: Vec<String>,
    },
    Return(Option<Expr>),
    Expression(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchCase {
    Case {
        pattern: MatchPattern,
        body: Vec<Statement>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum MatchPattern {
    Literal(Literal),
    Wildcard,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub return_type: Type,
    pub name: String,
    pub parameters: Vec<(Type, String)>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub includes: Vec<String>,
    pub namespaces: Vec<String>,
    pub functions: Vec<Function>,
}
