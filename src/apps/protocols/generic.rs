extern crate kernel;
use kernel::abstractions::session_types::*;
use std::thread::spawn;

fn srv<A: std::marker::Send + 'static>(x: A, c: Chan<(), Send<A, Eps>>) {
    c.send(x).close();
}

fn cli<A: std::marker::Send + std::fmt::Debug + 'static>(c: Chan<(), Recv<A, Eps>>) {
    let (c, x) = c.recv();
    println!("{:?}", x);
    c.close();
}

fn main() {
    let (c1, c2) = session_channel();
    let t = spawn(move || srv(42u8, c1));
    cli(c2);

    t.join().unwrap();
}
