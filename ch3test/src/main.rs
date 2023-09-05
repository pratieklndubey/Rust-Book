fn main() {
    println!("{}",f2c(-40));
    println!("{}",fib(97));
}

fn f2c(x:i32) -> i32 {
    (x-32)*5/9
}

fn fib(n:i128) -> i128 {
    let mut a: i128 = 0;
    let mut b: i128 = 1;
    let mut c: i128 = 0;
    let mut cnt = 3;
    while cnt != n+2 {
        c = a+b;
        a = b;
        b = c;
        cnt += 1;
    }
    c
}
