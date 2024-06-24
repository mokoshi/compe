// TODO トライ木実装 ing

use std::collections::VecDeque;

use proconio::{input, marker::Chars};

use std::fmt;

struct TrieTree {
    root: Node,
}

impl TrieTree {
    fn new() -> Self {
        Self { root: Node::new() }
    }
    fn add(&mut self, s: &Vec<char>) {
        let mut n = &mut self.root;
        for &c in s {
            let idx = (c as u8 - b'a') as usize;
            if let Some(next) = &n.v[idx] {
                n = next;
            }
            // match &mut n.v[idx] {
            //     Some(next) => n = next,
            //     None => n.v[idx] = Some(Node::new()),
            // }
        }
    }
}

#[derive(Clone)]
struct Node {
    v: Vec<Option<Node>>,
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Node {{ v: {:?} }}", self.v)
    }
}
impl Node {
    fn new() -> Self {
        Self { v: vec![None; 26] }
    }
}

fn main() {
    input! {
        n :usize,
        mut s: [Chars; n]
    }
    s.sort();
    println!("{:?}", s);

    let mut node = Node::new();
    node.v[0] = None;
    println!("{:?}", node);
    // let mut q = VecDeque::from([0]);
    // let mut e = vec![false; n];
    // while !q.is_empty() {
    //     let a = q.pop_front().unwrap();
    //     let c = s[a][0];
    // }
}
