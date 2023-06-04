struct Pair<T> {
    first: T,
    second: T,
}

struct AnotherPair<T1, T2> {
    first: T1,
    second: T2,
}

fn get_second<T>(pair: Pair<T>) -> T {
    return pair.second;
}

fn get_another_second<T1, T2>(pair: AnotherPair<T1, T2>) -> T2 {
    return pair.second;
}

fn main() {
    println!("Hello, world!");
    let pair: Pair<u32> = Pair {
        first: 1,
        second: 2,
    };

    println!("pair first = {} second = {}", pair.first, pair.second);
    let second: u32 = get_second(pair);
    println!("second = {}", second);

    let another_pair: AnotherPair<String, u32> = AnotherPair {
        first: String::from("hello"),
        second: 42,
    };
    let another_second: u32 = get_another_second(another_pair);
    println!("another_second = {}", another_second);
}
