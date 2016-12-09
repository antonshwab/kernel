
// O-DSL AST

use std::fmt;
use std::iter;
use std::vec;
use std::result::Result;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use commands::command;
use streams::interpreter;
use streams::atomize::*;

#[derive(Debug)]
pub enum Error<'ast> {
    ParseError,
    EvalError { desc: String, ast: AST<'ast> },
}

impl<'ast> fmt::Display for Error<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ParseError => write!(f, "Parse error!\n"),
            Error::EvalError { ref desc, ref ast } => {
                write!(f, "Eval error: {}.\nCaused here: {:?}\n", desc, ast)
            }
        }
    }
}

#[derive(PartialEq,Debug, Clone)]
pub enum Type {
    Nil = 0,
    Number = 1,
    Char = 2,
    Symbol = 3,
    List = 4,
    Dictionary = 5,
    Function = 6,
    View = 7,
    NameRef = 8,
    Verb = 9,
    Adverb = 10,
    Return = 11,
    Cond = 12,
    Native = 13,
    Quote = 14,
}


// OK LANG

//        a          l           a-a         l-a         a-l         l-l         triad    tetrad
// "+" : [ident,     flip,       ad(plus),   ad(plus),   ad(plus),   ad(plus),   null,    null  ],
// "-" : [am(negate),am(negate), ad(minus),  ad(minus),  ad(minus),  ad(minus),  null,    null  ],
// "*" : [first,     first,      ad(times),  ad(times),  ad(times),  ad(times),  null,    null  ],
// "%" : [sqrt,      am(sqrt),   ad(divide), ad(divide), ad(divide), ad(divide), null,    null  ],
// "!" : [iota,      odometer,   mod,        md,         ar(mod),    md,         null,    null  ],
// "&" : [where,     where,      ad(min),    ad(min),    ad(min),    ad(min),    null,    null  ],
// "|" : [rev,       rev,        ad(max),    ad(max),    ad(max),    ad(max),    null,    null  ],
// "<" : [asc,       asc,        ad(less),   ad(less),   ad(less),   ad(less),   null,    null  ],
// ">" : [desc,      desc,       ad(more),   ad(more),   ad(more),   ad(more),   null,    null  ],
// "=" : [imat,      group,      ad(equal),  ad(equal),  ad(equal),  ad(equal),  null,    null  ],
// "~" : [am(not),   am(not),    match,      match,      match,      match,      null,    null  ],
// "," : [enlist,    enlist,     cat,        cat,        cat,        cat,        null,    null  ],
// "^" : [pisnull,   am(pisnull),except,     except,     except,     except,     null,    null  ],
// "#" : [count,     count,      take,       reshape,    take,       reshape,    null,    null  ],
// "_" : [am(floor), am(floor),  drop,       ddrop,      drop,       cut,        null,    null  ],
// "$" : [kfmt,      am(kfmt),   dfmt,       dfmt,       dfmt,       dfmt,       null,    null  ],
// "?" : [real,      unique,     rnd,        pfind,      rnd,        ar(pfind),  splice,  null  ],
// "@" : [type,      type,       atd,        atl,        atd,        ar(atl),    amend4,  amend4],
// "." : [keval,     keval,      call,       call,       call,       call,       dmend4,  dmend4],
// "'" : [null,      null,       null,       atl,        kwindow,    ar(atl),    null,    null  ],
// "/" : [null,      null,       null,       null,       pack,       pack,       null,    null  ],
// "\\": [null,      null,       null,       unpack,     split,      null,       null,    null  ],

#[derive(PartialEq,Debug,Clone)]
pub enum Verb {
    Plus = 0,
    Minus = 1,
    Times = 2,
    Divide = 3,
    Mod = 4,
    Min = 5,
    Max = 6,
    Less = 7,
    More = 8,
    Equal = 9,
    Match = 10,
    Concat = 11,
    Except = 12,
    Take = 13,
    Drop = 14,
    Cast = 15,
    Find = 16,
    At = 17,
    Dot = 18,
    Gets = 19,
    Pack = 20,
    Unpack = 21,
    New = 22,
}

#[derive(Debug)]
pub enum Monadic {
    Flip = 0,
    Negate = 1,
    First = 2,
    Sqrt = 3,
    Iota = 4,
    Where = 5,
    Rev = 6,
    Asc = 7,
    Desc = 8,
    Group = 9,
    Not = 10,
    List = 11,
    Nil = 12,
    Count = 13,
    Floor = 14,
    Fmt = 15,
    Unique = 16,
    Type = 17,
    Eval = 18,
}

impl Verb {
    pub fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "+" => Ok(Verb::Plus),
            "-" => Ok(Verb::Minus),
            "*" => Ok(Verb::Times),
            "%" => Ok(Verb::Divide),
            "!" => Ok(Verb::Mod),
            "&" => Ok(Verb::Min),
            "|" => Ok(Verb::Max),
            "<" => Ok(Verb::Less),
            ">" => Ok(Verb::More),
            "=" => Ok(Verb::Equal),
            "~" => Ok(Verb::Match),
            "," => Ok(Verb::Concat),
            "^" => Ok(Verb::Except),
            "#" => Ok(Verb::Take),
            "_" => Ok(Verb::Drop),
            "$" => Ok(Verb::Cast),
            "?" => Ok(Verb::Find),
            "@" => Ok(Verb::At),
            "." => Ok(Verb::Dot),
            ";" => Ok(Verb::New),
            _ => Err(Error::ParseError),
        }
    }
}

impl fmt::Display for Verb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Verb::Plus => write!(f, "+"),
            Verb::Minus => write!(f, "-"),
            Verb::Times => write!(f, "*"),
            Verb::Divide => write!(f, "%"),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[derive(PartialEq,Debug,Clone)]
pub enum Adverb {
    Each,
    EachPrio,
    EachLeft,
    EachRight,
    Over,
    Scan,
    Iterate,
    Fixed,
    Assign,
    View,
    Separator,
}

impl Adverb {
    pub fn from_str(s: &str) -> Result<Self, Error> {
        match s {
            "/" => Ok(Adverb::Over),
            "\\" => Ok(Adverb::Scan),
            "'" => Ok(Adverb::Each),
            ";" => Ok(Adverb::Separator),
            ";:" => Ok(Adverb::Separator),
            "':" => Ok(Adverb::EachPrio),
            ":" => Ok(Adverb::Assign),
            "::" => Ok(Adverb::View),
            "\\:" => Ok(Adverb::EachLeft),
            "/:" => Ok(Adverb::EachRight),
            _ => Err(Error::ParseError),
        }
    }
}

impl fmt::Display for Adverb {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Adverb::Over => write!(f, "/"),
            _ => write!(f, "{:?}", self),
        }
    }
}

#[derive(PartialEq,Debug,Clone)]
pub enum AST<'ast> {
    // 0
    Nil,
    // 1
    Cons(&'ast AST<'ast>, &'ast AST<'ast>),
    // 2
    List(&'ast AST<'ast>),
    // 3
    Dict(&'ast AST<'ast>),
    // 4
    Call(&'ast AST<'ast>, &'ast AST<'ast>),
    // 5
    Lambda(&'ast AST<'ast>, &'ast AST<'ast>),
    // 6
    Verb(Verb, &'ast AST<'ast>, &'ast AST<'ast>),
    // 7
    Adverb(Adverb, &'ast AST<'ast>, &'ast AST<'ast>),
    // 8
    Ioverb(String),
    // 9
    NameInt(u16),
    SymbolInt(u16),
    SequenceInt(u16),
    Name(String),
    // A
    Number(i64),
    // B
    Hexlit(i64),
    // C
    Bool(bool),
    // D
    Symbol(String),
    // E
    Sequence(String),
    // F
    Cell(Box<Cell<'ast>>),
    // Syntactic sugar
    Assign(&'ast AST<'ast>, &'ast AST<'ast>),
    //
    Cond(&'ast AST<'ast>, &'ast AST<'ast>, &'ast AST<'ast>),
}

pub struct Arena<'ast> {
    data: RefCell<Vec<Box<AST<'ast>>>>,
}

impl<'ast> Arena<'ast> {
    pub fn new() -> Arena<'ast> {
        Arena { data: RefCell::new(vec![]) }
    }

    pub fn alloc(&'ast self, n: AST<'ast>) -> &'ast AST<'ast> {
        let b = Box::new(n);
        let p: *const AST<'ast> = &*b;
        self.data.borrow_mut().push(b);
        unsafe { &*p }
    }
}

pub fn parse<'ast>(s: &String) -> AST<'ast> {
    let ref mut x = interpreter::Interpreter::new().unwrap();
    let a = command::parse_Mex(s).unwrap();
    atomize(a, x)
}

impl<'ast> AST<'ast> {
    pub fn len(&self) -> usize {
        match self {
            &AST::List(ref car) => car.len(),
            &AST::Cons(_, ref cdr) => 1 + cdr.len(),
            &AST::Nil => 0,
            _ => 1,
        }
    }
    pub fn is_empty(&self) -> bool {
        self == &AST::Nil
    }
    pub fn is_cons(&self) -> bool {
        match self {
            &AST::Cons(_, _) => true,
            _ => false,
        }
    }
    pub fn shift(self) -> Option<(AST<'ast>, AST<'ast>)> {
        match self {
            AST::Cons(car, cdr) => Some((car, cdr)),
            AST::Nil => None,
            x => Some((x, AST::Nil)),
        }
    }
    pub fn to_vec(self) -> Vec<AST<'ast>> {
        let mut out = vec![];
        let mut l = self;
        loop {
            match l.clone() {
                AST::Cons(car, cdr) => {
                    out.push(car);
                    l = cdr;
                }
                AST::Nil => break,
                x => {
                    out.push(x);
                    break;
                }
            }
        }
        out
    }
}

impl<'ast> iter::IntoIterator for AST<'ast> {
    type Item = AST<'ast>;
    type IntoIter = vec::IntoIter<AST<'ast>>;

    fn into_iter(self) -> Self::IntoIter {
        self.to_vec().into_iter()
    }
}


impl<'ast> fmt::Display for AST<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AST::Nil => write!(f, ""),
            AST::Cons(ref a, ref b) => write!(f, "{} {}", a, b),
            AST::List(ref a) => write!(f, "{}", a),
            AST::Dict(ref d) => write!(f, "[{};]", d),
            AST::Call(ref a, ref b) => write!(f, "{} {}", a, b),
            AST::Lambda(ref a, ref b) => {
                match a {
                    &AST::Nil => write!(f, "{{[x]{}}}", b),
                    _ => {
                        let args = format!("{}", a).replace(" ", ";");
                        write!(f, "{{[{}]{}}}", args, b)
                    }
                }
            }
            AST::Verb(ref v, ref a, ref b) => write!(f, "{}{}{}", a, v, b),
            AST::Adverb(ref v, ref a, ref b) => write!(f, "{}{}{}", a, v, b),
            AST::Ioverb(ref v) => write!(f, "{}", v),
            AST::Number(n) => write!(f, "{}", n),
            AST::Hexlit(h) => write!(f, "0x{}", h),
            AST::Bool(b) => write!(f, "{:?}", b),
            AST::Name(ref n) => write!(f, "{}", n),
            AST::Symbol(ref s) => write!(f, "{}", s),
            AST::Sequence(ref s) => write!(f, "{:?}", s),
            AST::NameInt(ref n) => write!(f, "{}", n),
            AST::SymbolInt(ref s) => write!(f, "{}", s),
            AST::SequenceInt(ref s) => write!(f, "{:?}", s),
            AST::Cell(ref c) => write!(f, "{}", c),
            AST::Assign(ref a, ref b) => write!(f, "{}:{}", a, b),
            AST::Cond(ref c, ref a, ref b) => write!(f, "$[{};{};{}]", c, a, b),
        }

    }
}

pub fn extract_name<'ast>(a: AST<'ast>) -> u16 {
    match a {
        AST::NameInt(s) => s,
        x => 0,
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Cell<'ast> {
    t: Type,
    v: Vec<AST<'ast>>,
}

impl<'ast> fmt::Display for Cell<'ast> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(");
        for i in &self.v {
            write!(f, "{}", i);
        }
        write!(f, ")")
    }
}

pub fn nil<'ast>(arena: &'ast Arena<'ast>) -> &'ast AST<'ast> {
    arena.alloc(AST::Nil)
}

pub fn alloc<'ast>(n: AST<'ast>, arena: &'ast Arena<'ast>) -> &'ast AST<'ast> {
    arena.alloc(n)
}


pub fn call<'ast>(l: AST<'ast>, r: AST<'ast>, arena: &'ast Arena<'ast>) -> &'ast AST<'ast> {
    alloc(AST::Call(arena.alloc(l), arena.alloc(r)))
}

pub fn cons<'ast>(l: AST<'ast>, r: AST<'ast>, arena: &'ast Arena<'ast>) -> &'ast AST<'ast> {
    alloc(AST::Cons(arena.alloc(l), arena.alloc(r)))
}

pub fn fun<'ast>(l: AST<'ast>, r: AST<'ast>, arena: &'ast Arena<'ast>) -> &'ast AST<'ast> {
    match l {
        AST::Nil => alloc(AST::Lambda(arena.alloc(AST::Name("x".to_string())), arena.alloc(r))),
        _ => alloc(AST::Lambda(arena.alloc(l), arena.alloc(r))),
    }
}

pub fn dict<'ast>(l: AST<'ast>, arena: &'ast Arena<'ast>) -> &'ast AST<'ast> {
    match l {
        AST::Cons(a, b) => AST::Dict(arena.alloc(AST::Cons(a, b))),
        x => x,
    }
}

pub fn list<'ast>(l: AST<'ast>, arena: &'ast Arena<'ast>) -> &'ast AST<'ast> {
    match l {
        AST::Cons(a, b) => AST::List(arena.alloc(AST::Cons(a, b))),
        x => x,
    }
}

pub fn verb<'ast>(v: Verb,
                  l: AST<'ast>,
                  r: AST<'ast>,
                  arena: &'ast Arena<'ast>)
                  -> &'ast AST<'ast> {
    match v {
        Verb::Cast => {
            let rexpr = match r {
                AST::Dict(d) => {
                    match d {
                        AST::Cons(a, b) => {
                            match b {
                                AST::Cons(t, f) => {
                                    AST::Cond(arena.alloc(a),
                                              arena.alloc(t),
                                              arena.alloc(AST::List(arena.alloc(f))))
                                }
                                x => x,
                            }
                        }
                        x => x,
                    }
                }
                x => x, 
            };
            match l {
                AST::Nil => rexpr,
                _ => AST::Call(arena.alloc(l), arena.alloc(rexpr)), 
            }
        }
        _ => {
            match r { // optional AST transformations could be done during parsing
                AST::Adverb(a, al, ar) => {
                    match a {
                        Adverb::Assign => AST::Assign(arena.alloc(al), arena.alloc(ar)),
                        _ => {
                            AST::Adverb(a,
                                        arena.alloc(AST::Verb(v,
                                                              arena.alloc(l),
                                                              arena.alloc(AST::Nil))),
                                        ar)
                        }
                    }
                }
                _ => AST::Verb(v, arena.alloc(l), arena.alloc(r)),
            }
        }
    }
}

pub fn adverb<'ast>(a: Adverb,
                    l: AST<'ast>,
                    r: AST<'ast>,
                    arena: &'ast Arena<'ast>)
                    -> &'ast AST<'ast> {
    match a {
        Adverb::Assign => AST::Assign(arena.alloc(l), arena.alloc(r)),
        _ => AST::Adverb(a, arena.alloc(l), arena.alloc(r)),
    }
}
