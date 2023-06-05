#[derive(Copy, Clone)]
struct MagicNumber {
    number: u32,
}

struct Magician {
    name: String,
    age: u32,
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
    let o: &i32 = &m; // can borrow. Multiple immutable borrows are allowed
    println!("The address of m is {:p} and n is {:p}", &m, &n);
    println!("The value of m is {} and n is {} or {}", m, n, *n);
    println!("{}", square(&m)); // &m because m is not a pointer and square expects a pointer
    println!("{}", square(n)); // n is a pointer and square expects a pointer
    println!("{}", square(&n)); // this works too!

    let mut p: i32 = 5;
    println!("The value of p is {}", p);
    let mut q: &mut i32 = &mut p; // a mutable pointer that borrows p
    // let mut r: &mut i32 = &mut p; // cannot borrow. Only one mutable borrow is allowed
    println!("The value of q is {} or {}", q, *q);
    *q = 10;
    println!("The value of q is {} or {}", q, *q);
    println!("The value of p is {}", p);
    p = 20;
    println!("The value of p is {}", p);
    add_ten(&mut p);
    println!("The value of p is {}", p); // p is now 30

    // matching references
    let t: i32 = 5;
    match t {
        ref r => println!("Got a reference to {}", r),
    }

    let mut s: i32 = 5;
    match s {
        ref mut r => {
            *r = 10;
            println!("Got a mutable reference to {}", r);
        },
    }
}

fn square(x: &i32) -> i32 {
    return x * x;
}

fn add_ten(x: &mut i32) {
    *x += 10;
}
