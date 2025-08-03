fn fib(n: u32) -> u32 {
    if n <= 2 {
        1 // 기본 사례: 첫 번째와 두 번째 피보나치 수는 1
    } else {
        fib(n - 1) + fib(n - 2) // 재귀 호출
    }
}

fn main() {
    let n = 20;
    println!("fib({n}) = {}", fib(n));
}

