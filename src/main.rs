fn main() {
    let number = 3;

    match number {
        1 => println!("One"),
        2..=5 => println!("Between 2 and 5"),
        _ => println!("Any number"),
    }
}

fn find_item(items: Vec<&str>, search: &str) -> Option<usize> {
    for (index, item) in items.iter().enumerate() {
        if item == &search {
            return Some(index);
        }
    }
    None
}
