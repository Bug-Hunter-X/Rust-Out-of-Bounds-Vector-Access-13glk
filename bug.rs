fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let num = numbers.get(10);
    match num {
        Some(x) => println!("Value at index 10 is: {}", x),
        None => println!("Index out of bounds")
    }
}