# How to use?
Initilize it:  
```rust
io_init!(scan, _out);
```
Read by spliting whitespace:    
```rust
read!(scan, t => i32, mut n => i32, ...); 
```
Read by lines (for String):
```rust
let s = scan.next_line();
```
Read an array with size of `n`:  
```rust
let a: Vec<_> = (0..n).map(|_| scan.next::<i32>()).collect();
```

# More?
* [SparseTable(ST)](./ST.rs)
