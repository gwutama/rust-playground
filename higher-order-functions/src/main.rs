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

    // consumers
    let range = 0..10;
    let mut range2 = range.clone();
    let range3 = range.clone();
    let range4 = range.clone();

    let range_vec = range.collect::<Vec<i32>>();
    println!("range_vec = {:?}", range_vec);

    let eight = range2.find(|x| *x == 8);
    println!("eight = {:?}", eight);

    // filter
    let range_even = range3.filter(|x| is_even(*x)).collect::<Vec<i32>>();
    println!("range_even = {:?}", range_even);

    // map
    let range_even_cubed = range4.filter(|x| is_even(*x))
        .map(|x| x * x * x).collect::<Vec<i32>>();
    println!("range_even_cubed = {:?}", range_even_cubed);

    // fold
    let sum = range_even_cubed.iter().fold(0, |sum, x| sum + x);
    println!("sum = {}", sum);
}

fn triples(value: i32) -> i32 {
    return value * 3;
}

fn again<F: Fn(i32) -> i32>(fun: F, value: i32) -> i32 {
    return fun(fun(value));
}

fn is_even(value: i32) -> bool {
    return value % 2 == 0;
}