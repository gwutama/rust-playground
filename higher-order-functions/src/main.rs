fn main() {
    println!("Hello, world!");
    let mut x: i32 = 5;
    println!("x = {}", x);
    x = triples(x);
    println!("x = {}", x);

    // higher order function
    x = again(triples, x);
    println!("x = {}", x);

    // closure
    let triples_closure = |x: i32| -> i32 { x * 3 };
    x = again(triples_closure, 15);
    println!("x = {}", x);

    // inline closure
    x = again(|x: i32| -> i32 { x * 3 }, 15);
    println!("x = {}", x);

    // another inline closure
    let y: i32 = 42;
    let foo = |x: i32| -> i32 {
        return x + y;
    };
    println!("foo = {}", foo(8));

    // iterators
    let items = vec![1, 2, 3, 4, 5];
    for item in items.iter() {
        println!("item = {}", item);
    }
}

fn triples(value: i32) -> i32 {
    return value * 3;
}

fn again<F: Fn(i32) -> i32>(fun: F, value: i32) -> i32 {
    return fun(fun(value));
}