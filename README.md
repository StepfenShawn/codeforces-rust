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
read_line!(scan, s => String)
```

# More?
* [SparseTable(ST)](./ST.rs)
