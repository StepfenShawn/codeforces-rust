#[allow(unused_imports)]
use std::cmp::{min,max};
use std::io::{BufWriter, stdin, stdout, Write};
 
#[derive(Default)]
struct Scanner {
    buffer: Vec<String>
}

#[warn(dead_code)]
enum Split {
    WHITESPACE,
    NEWLINE
}

impl Scanner {
    fn read<T: std::str::FromStr>(&mut self, split_ty: Split) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            match split_ty {
                Split::WHITESPACE =>
                    self.buffer = input.split_whitespace().rev().map(String::from).collect(),
                Split::NEWLINE =>
                    self.buffer = input.split_inclusive("\n").rev().map(String::from).collect()
            }
        }
    }

    pub fn next<T: std::str::FromStr>(&mut self) -> T {
        self.read::<T>(Split::WHITESPACE)
    }

    pub fn next_line<T: std::str::FromStr>(&mut self) -> T {
        self.read::<T>(Split::NEWLINE)
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

macro_rules! read_line {
    ($scan: ident, $($v: pat => $t: ty), *) => {
        $(let $v = $scan.next_line::<$t>();)*
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
