fn main() {
    println!("Hello, world!");
    another_function(5, 'h');
    let y = {
        let mut x = 3;
        x = x + 1;
        x + 3
    };
    println!("The value of y is: {}", y);
    let c = five(11);
    println!("The value of c is: {}", c);
}

fn another_function(x: i32, unit_label: char) {
    println!("The value of x is: {}{}", x, unit_label);
}

fn five(x: i32) -> i32 {
    if x > 10 {
        return 10;
    }
    let x = 33 * x;
    x * 5
}