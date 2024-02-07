#[allow(unused_imports)]
use std::cmp::{min,max};
use std::io::{BufWriter, stdin, stdout, Write};
const BITS: usize = 19;
 
#[derive(Default)]
struct Scanner {
    buffer: Vec<String>
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}

#[allow(unused_variables)]
macro_rules! io_init {
    ($scan: ident, $out: ident) => {
        let mut $scan: Scanner = Scanner::default();
        let $out = &mut BufWriter::new(stdout());
    };
}

macro_rules! input {
    ($scan: ident, $($v: pat => $t: ty), *) => {
        let mut scan = $scan;
        $(let $v = scan.next::<$t>();)*
    };
}

macro_rules! array_2d {
    ($n: expr, $m: expr) => {
        vec![vec![0; $m]; $n]
    }
}

fn solve() {
    // ...
}

fn main() {
    io_init!(scan, out);
    input!(scan, t=>i32);
    for _ in 0..t {
        solve();
    }
}
