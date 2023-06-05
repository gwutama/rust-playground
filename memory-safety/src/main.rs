#[derive(Copy, Clone)]
struct MagicNumber {
    number: u32,
}

fn main() {
    println!("Hello, world!");

    // copy and clone
    let number1 = MagicNumber { number: 42 };
    let number2 = number1; // copy
    let number3 = number1.clone(); // clone
    println!("number1: {:?}", &number1 as *const MagicNumber);
    println!("number2: {:?}", &number2 as *const MagicNumber);
    println!("number3: {:?}", &number3 as *const MagicNumber);

    // pointers
    let m: i32 = 5;
    let n: &i32 = &m; // a pointer that borrows m
    println!("The address of m is {:p} and n is {:p}", &m, &n);
    println!("The value of m is {} and n is {} or {}", m, n, *n);
    println!("{}", square(&m)); // &m because m is not a pointer and square expects a pointer
    println!("{}", square(n)); // n is a pointer and square expects a pointer
    println!("{}", square(&n)); // this works too!
}

fn square(x: &i32) -> i32 {
    return x * x;
}
