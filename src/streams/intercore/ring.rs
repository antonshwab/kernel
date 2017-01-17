use super::ctx::Ctx;
use commands::ast::AST;
use commands::ast::Value;
use queues::publisher::Publisher;

pub fn pub_<'a>(args: &'a AST<'a>, ctx: &Ctx) -> AST<'a> {
    println!("publishers {:?}", args);
    let pubs = ctx.publishers();
    let cap = match args {
        &AST::Value(Value::Number(n)) => n,
        _ => 1024,
    } as usize;
    pubs.push(Publisher::with_capacity(cap));
    AST::Value(Value::Number(pubs.len() as i64 - 1))
}

pub fn sub_<'a>(args: &'a AST<'a>, ctx: &Ctx) -> AST<'a> {
    println!("subscribers {:?}", args);
    let subs = ctx.subscribers();
    let pubs = ctx.publishers();
    match args {
        &AST::Value(Value::Number(n)) if n < pubs.len() as i64 => {
            if let Some(p) = pubs.get_mut(n as usize) {
                subs.push(p.subscribe())
            }
        }
        _ => panic!("oops!"),
    }
    AST::Value(Value::Number(subs.len() as i64 - 1))
}

pub fn snd_<'a>(args: &'a AST<'a>, ctx: &Ctx) -> AST<'a> {
    let pubs = ctx.publishers();
    // println!("SND {:?}", args);
    match args {
        &AST::Cons(&AST::Value(Value::Number(val)), tail) => {
            match tail {
                &AST::Cons(&AST::Value(Value::Number(cursor_id)), tail) => {
                    if let Some(p) = pubs.get_mut(cursor_id as usize) {
                        match p.next() {
                            Some(v) => {
                                *v = val as u64;
                                println!("snd_{} {:?}", cursor_id, v);
                                p.commit();
                            }
                            None => return AST::Yield,
                        }
                    }
                }
                _ => panic!("oops!"),
            }
        }
        _ => panic!("oops!"),
    }
    AST::Nil
}

pub fn rcv_<'a>(args: &'a AST<'a>, ctx: &Ctx) -> AST<'a> {
    let subs = ctx.subscribers();
    let mut res = 0u64;
    // println!("RECV {:?}", args);
    match args {
        &AST::Value(Value::Number(n)) => {
            if let Some(s) = subs.get_mut(n as usize) {
                // println!("SOME {:?}", s);
                match s.recv() {
                    Some(v) => {
                        res = *v;
                        println!("rcv_{} {:?}", n, res);
                        s.commit();
                    }
                    None => return AST::Yield,
                }
            }
        }
        _ => panic!("oops!"),
    }
    AST::Value(Value::Number(res as i64))
}
