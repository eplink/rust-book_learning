fn main() {
    another_function();
    another_function2(5);
    print_labeled_measurement(5, 'Z');
    codeblock();
    println! ("returned  :{0} !", plus_one(5));
}

fn another_function() {
    println!("Another function.");
}

fn another_function2(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn codeblock() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

fn plus_one(value: i32) -> i32 {
    // значение функции - параметр плюс 3
    value + 1
}
