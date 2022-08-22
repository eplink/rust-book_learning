use std::io;
const MINUT_HOUR:u32 = 60;

fn main() {
    let x = 5;
    println!("The value of x is: {x}");

    let x = x + 1;
    println!("The value of x is: {x}");

    {
        let x = x *2;
        println!("The value of x is: {x}");
    }

    println!("The value of x is: {x}");
    println!("The const MINUT_HOUR is: {MINUT_HOUR}");

    // addition
    let sum = 5+10;
    println!("Сумма: {sum}");

    // substraction
    let difference = 95.5 - 4.3;
    println!("Разность: {difference}");
    
    // multiplication
    let product = 4 * 30;
    println!("Множество: {product}");

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;
    println!("Частное дробное: {quotient}");
    println!("Частное целое: {floored}");

    // remainder
    let remainder = 43 % 5;
    println!("Остаток от деления: {remainder}");

    // logical
    let t = true;
    let f:bool = false;
    println!("t: {t}, f: {f}");

    // char
    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';
    println!("c: {c}, f: {z}, heart_eyed_cat: {heart_eyed_cat}");

    // tupes
    let tup = (500, 6.4, 1);
    let (x , y, z) = tup;
    println!("x = {x}, y = {y}, z = {z}");
    let y = tup.1;
    println!("tup.1 = {y}");

    // array
    let a = [1, 2, 3, 4, 5];

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

    println!("The value of the element at index {index} is: {element}");


}
