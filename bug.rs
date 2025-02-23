fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let num = numbers.get(10);
    match num {
        Some(n) => println!("The number is {}", n),
        None => println!("No number found"),
    }
}