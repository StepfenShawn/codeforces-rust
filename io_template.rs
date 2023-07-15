//================================================================================
#[allow(unused_imports)] use std::cmp::{max, min, Ordering};
#[allow(unused_imports)] use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
#[allow(unused_imports)] use std::iter::FromIterator;
use std::io::Write;

static mut GLOBAL_STDIN: *mut std::io::Stdin = 0usize as _;
static mut GLOBAL_STDINLOCK: *mut std::io::StdinLock<'static> = 0usize as _;
static mut GLOBAL_STDOUT: *mut std::io::Stdout = 0usize as _;
static mut GLOBAL_STDOUTLOCK: *mut std::io::StdoutLock<'static> = 0usize as _;
const BUF_SIZE: usize = 1024*1024;
static mut GLOBAL_INPUT_BUF: *mut u8 = 0usize as _;
static mut GLOBAL_INPUT_PTR: *const u8 = 0usize as _;
static mut GLOBAL_INPUT_END: *const u8 = 0usize as _;

fn init() {
    unsafe {
        GLOBAL_STDIN = Box::into_raw(Box::new(std::io::stdin()));
        GLOBAL_STDINLOCK = Box::into_raw(Box::new(GLOBAL_STDIN.as_ref().unwrap().lock()));
        GLOBAL_STDOUT = Box::into_raw(Box::new(std::io::stdout()));
        GLOBAL_STDOUTLOCK = Box::into_raw(Box::new(GLOBAL_STDOUT.as_mut().unwrap().lock()));
        let buf = Box::<[u8]>::into_raw(vec![0u8; BUF_SIZE].into_boxed_slice());
        GLOBAL_INPUT_BUF = (*buf).as_mut_ptr();
        GLOBAL_INPUT_PTR = GLOBAL_INPUT_BUF;
        GLOBAL_INPUT_END = GLOBAL_INPUT_BUF;
    }
}

fn peek() -> u8 { unsafe {
    use std::io::Read;
    if GLOBAL_INPUT_PTR == GLOBAL_INPUT_END {
        let n = GLOBAL_STDINLOCK.as_mut().unwrap().read(std::slice::from_raw_parts_mut(GLOBAL_INPUT_BUF, BUF_SIZE)).expect("I/O error");
        GLOBAL_INPUT_PTR = GLOBAL_INPUT_BUF;
        GLOBAL_INPUT_END = GLOBAL_INPUT_PTR.offset(n as isize);
    }
    *GLOBAL_INPUT_PTR
} }

fn getchar() -> u8 { let c = peek(); unsafe { GLOBAL_INPUT_PTR = GLOBAL_INPUT_PTR.offset(1); } c }
fn ungetc() { unsafe { GLOBAL_INPUT_PTR = GLOBAL_INPUT_PTR.offset(-1); } }
fn skip_whitespaces() { loop { match getchar() as char { ' ' | '\t' | '\n' | '\r' => { }, _ => { ungetc(); break; } } } }
trait FastRead { fn read() -> Self; }

macro_rules! read_int_impl {
    ( $signed:expr, $($T:ident,)* ) => {
        $(impl FastRead for $T {
            fn read() -> $T {
                skip_whitespaces(); let is_negative = if $signed && peek() == '-' as u8 { getchar(); true } else { false }; let mut val: $T = 0;
                loop {
                    let c = getchar(); let d = c.wrapping_sub('0' as u8);
                    if d >= 10 { match c as char {
                        ' ' | '\t' | '\n' | '\r' => { ungetc(); return if is_negative { 0-val } else { val }; },
                        _ => panic!("invalid input character: `{}' (code: {})", c as char, c),
                    }}
                    val = 10*val + (d as $T);
                }
            }
        })*
    };
}
macro_rules! read_tuple_impl {
    ( ) => ();
    ( $a:ident, $($name:ident,)* ) => {
        impl<$a:FastRead, $($name:FastRead),*> FastRead for ($a, $($name,)*) { fn read() -> Self { ( $a::read(), $($name::read()),* ) } }
        read_tuple_impl!($($name,)*);
    }
}
macro_rules! snd_arg_impl { ($a:expr; $e:expr) => { $e }; }
macro_rules! read_array_impl {
    () => {};
    ($len:expr, $($lens:expr,)*) => {
        impl <T: FastRead> FastRead for [T; $len] { fn read() -> Self { [ $(snd_arg_impl!($lens; read::<T>())),* ] } }
        read_array_impl!($($lens,)*);
    };
}
unsafe fn extend_vec(v: &mut Vec<u8>, first: *const u8, last: *const u8) {
    let len = usize::wrapping_sub(last as _, first as _);
    v.extend_from_slice(&std::slice::from_raw_parts(first, len));
}
macro_rules! read_string_inplace {
    ($func:ident, $($pattern:pat_param)|+) => {
        #[allow(unused)] fn $func(s: &mut String) -> bool {
            skip_whitespaces();
            unsafe { let mut ptr = GLOBAL_INPUT_PTR; let end = GLOBAL_INPUT_END; let v = s.as_mut_vec(); v.clear();
                     loop {
                         if ptr == end { extend_vec(v, GLOBAL_INPUT_PTR, end); GLOBAL_INPUT_PTR = GLOBAL_INPUT_END; peek(); ptr = GLOBAL_INPUT_PTR; }
                         match *ptr as char {
                             $($pattern)|+ => { extend_vec(v, GLOBAL_INPUT_PTR, ptr); GLOBAL_INPUT_PTR = ptr; return v.is_empty(); }
                             _ => { ptr = ptr.offset(1); }
                         }
                     }
            }
        }
    };
}
read_string_inplace!(getword, ' ' | '\t' | '\n' | '\r');
read_string_inplace!(getline, '\n');
#[allow(unused)] fn read_line() -> String { let mut s = String::new(); getline(&mut s); s }
impl FastRead for String { fn read() -> String { let mut s = String::new(); getword(&mut s); s } }
impl FastRead for char { fn read() -> char { skip_whitespaces(); getchar() as char } }
read_int_impl!(false, u8, u16, u32, u64, usize,);
read_int_impl!(true,  i8, i16, i32, i64, isize,);
read_tuple_impl!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,);
read_array_impl!(12, 11, 10, 9, 8, 7, 6, 5, 4, 3, 2, 1, 0,);
#[allow(dead_code)] fn read<T: FastRead>() -> T { T::read() }
#[allow(unused_macros)] macro_rules! print   { ($($arg:tt)*) => { write!  (unsafe { GLOBAL_STDOUTLOCK.as_mut().unwrap() }, $($arg)*).unwrap() } }
#[allow(unused_macros)] macro_rules! println { ($($arg:tt)*) => { writeln!(unsafe { GLOBAL_STDOUTLOCK.as_mut().unwrap() }, $($arg)*).unwrap() } }
#[allow(dead_code)] fn read_vec<T: FastRead>(n: usize) -> Vec<T> { let mut v = Vec::with_capacity(n); for _ in 0..n { v.push(read()); } v }
#[allow(dead_code)] fn print_iter<I: Iterator<Item=T>, T: std::fmt::Display>(iter: I) { let mut iter = iter; if let Some(x) = iter.next() { print!("{}", x); for x in iter { print!(" {}", x); } } println!(""); }
#[allow(dead_code)] fn print_vec<T: std::fmt::Display>(v: &Vec<T>) { print_iter(v.iter()); }
#[allow(unused_macros)] macro_rules! debug { ($a:expr, $($b:expr),*) => (if cfg!(debug_assertions) { println!(concat!("DEBUG: ", stringify!($a), " = {:?}", $(", ", stringify!($b), " = {:?}"),*), $a, $($b),*); } ) }
//================================================================================
