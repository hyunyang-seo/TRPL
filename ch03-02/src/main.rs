fn main() {
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;
    println!("The value of sum is: {}", sum);
    println!("The value of difference is: {}", difference);
    println!("The value of product is: {}", product);
    println!("The value of quotient is: {}", quotient); 
    println!("The value of remainder is: {}", remainder);

    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!(
        "The value of five_hundred is: {}, six_point_four is: {}, one is: {}",
        five_hundred, six_point_four, one
    );

    let a = [1, 2, 3, 4, 5];
    let index = 1;
    let element = a[index];

    println!("The value of element is: {}", element);
}
