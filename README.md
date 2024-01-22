# Template
Just copy this!!! refered from [here](https://codeforces.com/blog/entry/67391).  
```rust
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

macro_rules! array_2d {
    ($n: expr, $m: expr) => {
        vec![vec![0; $m]; $n]
    }
}
```
# Example
[F. Array Partition](https://codeforces.com/contest/1454/problem/F):  
```rust
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

macro_rules! array_2d {
    ($n: expr, $m: expr) => {
        vec![vec![0; $m]; $n]
    }
}

struct ST {
    mx: Vec<Vec<i32>>,
    mi: Vec<Vec<i32>>
}

impl ST {
    fn build(nums: &[i32]) -> Self {
        let n = nums.len();
        let mut mx = array_2d!(n+1, 33);
        let mut mi = array_2d!(n+1, 33);

        for i in 1..n {
            mx[i][0] = nums[i];
            mi[i][0] = nums[i];
        }

        for j in 1..=((31 - (n as i32).leading_zeros()) as usize) {
            for i in 1..=(n - (1 << j) + 1) {
                mx[i][j] = i32::max(mx[i][j - 1], mx[i + (1 << (j - 1))][j - 1]);
                mi[i][j] = i32::min(mi[i][j - 1], mi[i + (1 << (j - 1))][j - 1]);
            }
        }

        ST {mx, mi}
    }

    fn query_min(&self, l: usize, r: usize) -> i32 {
        let x = ((r - l + 1) as f64).log2() as usize;
        i32::min(self.mi[l][x], self.mi[r - (1 << x) + 1][x])
    }

    fn query_max(&self, l: usize, r: usize) -> i32 {
        let x = ((r - l + 1) as f64).log2() as usize;
        i32::max(self.mx[l][x], self.mx[r - (1 << x) + 1][x])
    }
}

fn solve() {
    io_init!(scan, out);
    let n = scan.next();
    let mut a: Vec<_> = (0..n).map(|_| scan.next::<i32>()).collect();
    a.insert(0, 0);
    let st = ST::build(&a);
    
    for x in 1..=(n-2) {
        let mut l = x + 1;
        let mut r = n - 1;
        let mmax1 = st.query_max(1, x);

        while l <= r {
            let mid = (l + r) >> 1;
            let mmin2 = st.query_min(x+1, mid);
            let mmax2 = st.query_max(mid+1, n);

            if mmax1 == mmin2 && mmin2 == mmax2 {
                println!("YES");
                println!("{} {} {}", x, mid - x, n - mid);
                return;
            }
            else if mmax1 < mmin2 {
                l = mid + 1;
            }
      
            else if mmax1 > mmin2 {
                r = mid - 1;
            }
      
            else if mmin2 < mmax2 {
                l = mid + 1;  
            }

            else if mmin2 > mmax2 {
                r = mid - 1;
            }
        }
    }

    println!("NO");
}

fn main() {
    io_init!(scan, out);
    let t = scan.next();
    for _ in 0..t {
        solve();
    }
}
```
