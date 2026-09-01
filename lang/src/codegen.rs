use crate::ast::*;

pub struct CodeGenerator {
    output: String,
    indent_level: usize,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            output: String::new(),
            indent_level: 0,
        }
    }

    pub fn generate(&mut self, statements: &[Statement]) -> String {
        self.output.push_str("#include <carlinho.h>\n");
        self.output.push_str("using namespace facilitador;\n\n");
        self.output.push_str("int main() {\n");
        self.indent_level += 1;

        for stmt in statements {
            self.generate_statement(stmt);
        }

        self.indent_level -= 1;
        self.output.push_str("    return 0;\n");
        self.output.push_str("}\n");

        self.output.clone()
    }

    fn indent(&self) -> String {
        "    ".repeat(self.indent_level)
    }

    fn generate_statement(&mut self, stmt: &Statement) {
        match stmt {
            Statement::VarDecl { ty, name, value } => {
                let type_str = match ty {
                    Type::Int => "int",
                    Type::Double => "double",
                    Type::Char => "char",
                    Type::String => "string",
                    Type::Bool => "bool",
                    Type::LongLong => "long long",
                    Type::Void => "void",
                };
                self.output.push_str(&format!("{}{} {}", self.indent(), type_str, name));
                if let Some(val) = value {
                    self.output.push_str(" = ");
                    self.generate_expr(val);
                }
                self.output.push_str("\n");
            }
            Statement::Assignment { name, value } => {
                self.output.push_str(&format!("{}{0} = ", self.indent()));
                self.generate_expr(value);
                self.output.push_str("\n");
            }
            Statement::Cout { values } => {
                self.output.push_str(&format!("{}cout", self.indent()));
                for val in values {
                    self.output.push_str(" << ");
                    self.generate_expr(val);
                }
                self.output.push_str("\n");
            }
            Statement::Cin { targets } => {
                self.output.push_str(&format!("{}cin", self.indent()));
                for target in targets {
                    self.output.push_str(&format!(" >> {}", target));
                }
                self.output.push_str("\n");
            }
            Statement::Return(expr) => {
                self.output.push_str(&format!("{}return", self.indent()));
                if let Some(val) = expr {
                    self.output.push_str(" ");
                    self.generate_expr(val);
                }
                self.output.push_str("\n");
            }
            Statement::Expression(expr) => {
                self.output.push_str(&self.indent());
                self.generate_expr(expr);
                self.output.push_str("\n");
            }
            _ => {}
        }
    }

    fn generate_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Ident(name) => self.output.push_str(name),
            Expr::Literal(lit) => match lit {
                Literal::Integer(n) => self.output.push_str(&n.to_string()),
                Literal::Float(f) => self.output.push_str(&f.to_string()),
                Literal::String(s) => self.output.push_str(&format!("\"{}\"", s)),
                Literal::Char(c) => self.output.push_str(&format!("'{}'", c)),
                Literal::Boolean(b) => self.output.push_str(if *b { "true" } else { "false" }),
            },
            Expr::BinaryOp { op, left, right } => {
                self.generate_expr(left);
                self.output.push_str(&format!(" {} ", op));
                self.generate_expr(right);
            }
            Expr::FunctionCall { name, args } => {
                self.output.push_str(&format!("{}(", name));
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.generate_expr(arg);
                }
                self.output.push_str(")");
            }
        }
    }
}
