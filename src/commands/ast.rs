
// K primitives: http://kparc.com/lisp.txt

#[derive(Debug)]
pub enum AST {
    Integer(u64), Symbol(String),
    Float(f64),
    Append, Get, Set,
    Curry, Compose, Lambda(Box<AST>),
    Expr, Nil,
    CommaList(Box<AST>), ColonList(Box<AST>),
    Cons(Box<AST>,Box<AST>), Car, Setq, Cond,
    Map, Reduce, Min, Max,
    Plus, Add, Mul, Div,
    Greater, Less, Equal,
    Length, Reverse, Member,
}
