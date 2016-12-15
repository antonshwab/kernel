use std::fmt;
use std::hash::BuildHasherDefault;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::iter;
use std::vec;
use commands::ast::*;
use fnv::*;
use std::cell::UnsafeCell;
use streams::otree::*;

#[derive(Debug, Clone)]
pub struct Entry<'a>(u16, &'a AST<'a>);

#[derive(Debug)]
pub struct Environment<'a> {
    tree: UnsafeCell<Tree<'a, Entry<'a>>>,
}

impl<'a> Environment<'a> {
    pub fn new_root() -> Result<Environment<'a>, Error> {
        let mut s = Tree::with_capacity(10000 as usize);
        Ok(Environment { tree: UnsafeCell::new(s) })
    }

    pub fn last(&'a self) -> &'a Node<'a> {
        let tree = unsafe { &*self.tree.get() };
        tree.last_node()
    }

    pub fn new_child(&'a self, n: &'a Node<'a>) -> &'a Node<'a> {
        let tree = unsafe { &mut *self.tree.get() };
        tree.append_node(n)
    }

    pub fn define(&'a self, key: u16, value: &'a AST<'a>) -> Result<(), Error> {
        let tree = unsafe { &mut *self.tree.get() };
        tree.insert(Entry(key, value));
        Ok(())
    }

    pub fn get(&'a self, key: u16, n: &'a Node<'a>) -> Option<(&'a AST, &Node<'a>)> {
        let tree = unsafe { &mut *self.tree.get() };
        match tree.get(n, |e| e.0 == key) {
            Some(x) => Some(((x.0).1, x.1)),
            None => None,
        }
    }

    pub fn clean(&self) {
        let tree = unsafe { &mut *self.tree.get() };
        tree.clean();
    }
}
