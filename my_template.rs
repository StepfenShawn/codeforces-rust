#[allow(unused_imports)]
use std::cmp::{min,max};
use std::io::{BufWriter, stdin, stdout, Write};
 
#[derive(Default)]
struct Scanner { buffer: Vec<String> }

impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect()
        }
    }

    pub fn next_line(&mut self) -> String {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed read");
        input.trim().to_string()
    }
}

#[allow(unused_variables)]
macro_rules! io_init {
    ($scan: ident, $out: ident) => {
        let mut $scan: Scanner = Scanner::default();
        let $out = &mut BufWriter::new(stdout());
    };
}

macro_rules! read {
    ($scan: ident, $($v: pat => $t: ty), *) => {
        $(let $v = $scan.next::<$t>();)*
    };
}

fn solve() {
    // ...
}

fn main() {
    io_init!(scan, _out);
    read!(scan, t=>i32);
    for _ in 0..t {
        solve();
    }
}
