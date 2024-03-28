fn main() {
    println!("Hello, world!");

    another_function();
    let f4 = fibonacci(4);
    println!("fib 4 = {f4}");
}

fn another_function() {
    println!("Another function.");
}

fn fibonacci(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}
