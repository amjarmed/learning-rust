fn main() {
    let x = String::from("hello");
    let y = 100;
    take_ownership(x.clone());
    take_ownership(x);
    make_copy(y);
    make_copy(y);
    make_copy(y);
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}

fn make_copy(some_integer: i32) {
    println!("{}", some_integer);
}
