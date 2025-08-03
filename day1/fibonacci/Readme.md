```
➜  fibonacci git:(main) ✗ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/fibonacci`

thread 'main' panicked at src/main.rs:7:9:
not yet implemented: Implement this
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
* ```src/main.rs``` 파일의 7번째 줄, 9번째 열에서 ```panic!``` 발생
* 메시지 : ```not yet implemented: Implement this``` -> 이건 ```unimplemented!("Implement this")```가 호출되었을 때 나오는 것

# 수정본

```rust
fn fib(n: u32) -> u32 {
    if n <= 2 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn main() {
    let n = 20;
    println!("fib({n}) = {}", fib(n));
}
```

