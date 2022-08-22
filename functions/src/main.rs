fn main() {
    let x = print_labeled_measurement(5);
    println! ("Возвращаемое значение :{x}");
}

fn print_labeled_measurement(value: i32) -> i32 {
    // значение функции - параметр плюс 3
    value + 3
}
