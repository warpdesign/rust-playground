fn fibo(n: i32) -> i32 {
    if n <= 1 {
        return n;
    } else {
        fibo(n-2) + fibo(n-1)
    }
}

fn main() {
    for number in 0..20 {
        println!("fb({})={}", number, fibo(number));
    }
}
