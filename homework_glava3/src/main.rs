use std::io;


fn main() {
    println!("Введите градусы по Фаренгейту");

    let mut grad_f  = String::new();
    
    io::stdin()
        .read_line(&mut grad_f)
        .expect("Неправильный ввод!");
    
    let grad_f:i32 = grad_f.trim()
                            .parse::<i32>().expect("Ошибка ввода!");

    println!("Введено градусов по фаренгейту: {grad_f}");
    
    let grad_c:i32 = convert_to_celsius(grad_f);
    
    println!("В градусах цельсия: {grad_c}");
    
}

fn convert_to_celsius(grad_f :i32) -> i32 {
    (grad_f-32) * 10 / 18
}