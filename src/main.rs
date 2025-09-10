use std::time::Duration;
use math_operation_queue::ThreadPool;
fn main() {
    let pool = ThreadPool::new(3);
    std::thread::sleep(Duration::from_secs(1));
    pool.execute(|| {
        println!("birinci job basladi");

        println!("birinci job bitti");
    });

    pool.execute(|| {
        println!("ikinci job basladi");
        let fib_result = fib(29);
        println!("fibonacci hesaplamasinin sonucu: {}",fib_result);

        println!("ikinci job bitti");
    });
    pool.execute(|| {
        println!("ucuncu job basladi");
        let factorial_result = factorial(15);
        println!("faktoriyel hesaplamasinin sonucu : {}", factorial_result);

        println!("ucuncu job bitti");
    });
}


fn fib(n: u64) -> u64 {
    if n <= 1 { n } else { fib(n-1) + fib(n-2)}
}

fn factorial(n: u64) -> u64 {
    if n < 1 {1} else { n * factorial(n-1)}
}