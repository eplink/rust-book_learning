use std::io;

fn main() {
    // input_and_convert();
    generate_fib();
}

// Перевод введенных градусов Фаренгейта в Градусы Цельсия c
fn input_and_convert() {
    println!("Введите градусы по Фаренгейту");
    let mut grad_f = String::new();

    io::stdin()
        .read_line(&mut grad_f)
        .expect("Неправильный ввод!");

    let grad_f: i32 = grad_f.trim().parse::<i32>().expect("Ошибка ввода!");

    match convert_to_celsius(grad_f) {
        Ok(grad) => println!("в градусах цельсия это: {0}", grad),
        Err(err) => println!("Ошибка: {0}", err),
    }
}

// функция конвертации градусов фаренгейта в градусы цельсия
fn convert_to_celsius(grad_f: i32) -> Result<i32, String> {
    const MIN_FH: i32 = -459; // абсолютный ноль по Фаренгейту
    if grad_f > MIN_FH {
        return Ok((grad_f - 32) * 5 / 9);
    } else {
        return Err("Переданное значение меньше абсолютного нуля!".to_owned());
    };
}

// Генерирование n-го числа Фибоначчи
fn generate_fib() {
    println!("Введите номер числа Фибоначчи ");

    let mut n_fib = String::new();

    io::stdin()
        .read_line(&mut n_fib)
        .expect("Неправильный ввод!");

    let n_fib: u32 = n_fib.trim().parse::<u32>().expect("Ошибка ввода!");

    println!("Число Фибоначчи: {0}", f_cycle(n_fib))
}

// циклическая функция генерации числа фибоначчи
fn f_cycle(n: u32) -> u32 {
    //const MAX_X: u32 = 47; // максимальный x для 32 разрядного числа фибоначи

    let mut r: u32 = 0;
    let mut _r: u32 = 0;
    let mut x: u32 = 0;

    for i in 1..(n + 1) {
        if i == 1 {
            r = 1;
            x = r;
        } else {
            r = _r + x;
            _r = x;
            x = r;
        };
    }

    r
}

/*
// рекурсивная функция генерации числа фибоначи
fn f(x: u32, mut buf ) -> u32 {
    //const MAX_X: u32 = 47; // максимальный x для 32 разрядного числа фибоначи
    let mut fib:u32 = 0;

    if buf == 0 {

    }


    if x == 1 {
            fib = 1;
            fib_ = 0;
            buf = 0;
        } else {
            buf = f(x + 1, buf) + buf;
        }

    println!("Число: {0}", fib);
    fib
}
*/
