#![allow(unused_must_use)]
#![allow(unused_variables)]
#![allow(unused_imports)]

extern crate kernel;

// The Kernel Shell

use std::io::prelude::*;
use std::io;
use kernel::commands::*;

pub fn prompt() {
    print!("> ");
    let _ = io::stdout().flush();
}

fn main() {
    let mut input_line = String::new();
    loop {
        prompt();
        let res = io::stdin().read_line(&mut input_line);
        match res {
            Ok(0) => break,
            x => {
                match input_line.trim() {
                    "exit" => break,
                    line => println!("{:?}", command::parse_Mex(&line.to_string())),
                }
            }
        }
        input_line.clear();
    }
    println!("May the force be with you!");
}
