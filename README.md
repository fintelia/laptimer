# laptimer

A simple Rust crate to print the time between different points in a program:

```rust
laptimer::print!(); 

// do some work...

laptimer::print!();
```

Which produces output like:

```
[   0.000ms] src/lib.rs:10
[   1.234ms] src/lib.rs:20
```