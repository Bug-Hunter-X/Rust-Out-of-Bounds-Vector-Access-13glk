fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let index = 10;
    match numbers.get(index) {
        Some(x) => println!("Value at index {} is: {}", index, x),
        None => println!("Index {} is out of bounds", index)
    }
} 