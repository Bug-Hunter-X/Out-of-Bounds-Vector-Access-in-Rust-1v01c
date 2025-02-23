fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let index = 10;
    match numbers.get(index) {
        Some(n) => println!("The number at index {} is {}", index, n),
        None => println!("Index {} is out of bounds", index),
    }

    //Alternative solution using if let
    if let Some(n) = numbers.get(index){
        println!("The number at index {} is {}", index, n);
    } else {
        println!("Index {} is out of bounds", index);
    }
} 