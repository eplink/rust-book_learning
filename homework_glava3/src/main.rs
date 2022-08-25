use std::io;

fn main() {
    input_and_convert(); 
    generate_fib();
}

// Перевод введенных градусов Фаренгейта в Градусы Цельсия c
fn input_and_convert() {
    println!("Введите градусы по Фаренгейту");
    let mut grad_f  = String::new();
    
    io::stdin()
        .read_line(&mut grad_f)
        .expect("Неправильный ввод!");
    
    let grad_f:i32 = grad_f.trim()
                            .parse::<i32>().expect("Ошибка ввода!");

    match convert_to_celsius(&grad_f) {
        Ok(grad) => println!("в градусах цельсия это: {0}",grad),
        Err(err) => println!("Ошибка: {0}",err),
    }
}

// функция конвертации градусов фаренгейта в градусы цельсия
fn convert_to_celsius(grad_f: &i32) -> Result <i32, String> {
    const MIN_FH:i32 = -459; // абсолютный ноль по Фаренгейту
    if grad_f.to_owned() > MIN_FH {
        return  Ok((grad_f.to_owned() - 32) * 5 / 9);
    } else {
        return Err("Переданное значение меньше абсолютного нуля!".to_owned());
    };
}

// Генерирование n-го числа Фибоначчи
fn generate_fib() {
    println!("Введите номер числа Фибоначчи ");
    let mut n_fib  = String::new();
    
    io::stdin()
        .read_line(&mut n_fib)
        .expect("Неправильный ввод!");
    
    let n_fib:u32 = n_fib.trim()
                            .parse::<u32>().expect("Ошибка ввода!");

    match gen_fib (&n_fib) {
        Ok(fib) => println!("Число: {0}",fib),
        Err(err) => println!("Ошибка: {0}",err),
    }
}

// функция генерации числа фибоначи в цикле
fn gen_fib (n_fib: &u32) -> Result <u32, String> {
    const MAX_N:u32 = 47; // максимальный N для 32 разрядного числа фибоначи
    
    if n_fib.to_owned() > MAX_N {
        return Err("Сгенерированное значение будет больше возможного".to_owned());        
    };

    let mut next:u32 = 1;
    let mut prev:u32 = 0;
    for i in 2..n_fib.to_owned() {
        println!("{}!", i * next);
    }



    return  Ok(n_fib.to_owned());

}