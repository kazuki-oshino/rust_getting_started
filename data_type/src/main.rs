use std::io;
fn main() {
    let sum = 5 + 10;
    println!("sum: {}", sum);
    let difference = 95.5 - 4.3;
    println!("difference: {}", difference);
    let product = 4 * 30;
    println!("product: {}", product);
    let quotient = 56.7 / 32.2;
    println!("quotient: {}", quotient);
    let floored = 2 / 3;
    println!("floored: {}", floored);
    let remainder = 43 % 5;
    println!("remainder: {}", remainder);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup: {:?}", tup);
    let five_hundred = tup.0;
    println!("five_hundred: {}", five_hundred);
    let a = [1, 2, 3, 4, 5];
    println!("a: {:?}", a);

    println!("Please enter an array index.");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");
    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");
    let element = a[index];
    println!("The value of the element at index {} is: {}", index, element);

}
