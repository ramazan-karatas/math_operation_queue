use math_operation_queue::ThreadPool;
fn main() {
    let pool = ThreadPool::new(3);


}


fn fib(n: u64) -> u64 {
    if n <= 1 { n } else { fib(n-1) + fib(n-2)}
}

fn factorial(n: u64) -> u64 {
    if n < 1 {1} else { n * factorial(n-1)}
}