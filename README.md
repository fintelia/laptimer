# laptimer

A simple Rust crate to print the time between different points in a program. Each call to `print!` indicates the amount of time elapsed since the previous call:

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