use std::io;                                                // добавление в область видимости модуля io из пакета std
use rand::Rng;                                              // +-+ видимости типажа Rng пакета rand
use std::cmp::Ordering;                                     // +-+ видимости типа Ordering из модуля cmp пакета std

fn main() {                                                 // функция верхнего уровня, точка входа
    println!("Я загадал число!");                           // вывод на дисплей сообщения
    
    let secret_number = rand::thread_rng()   // объявление целочисленной переменной с присвоением ей -    
                             .gen_range(1..=100);           // значения, методом gen_range() вр. объекта типа ThreadRng


    loop {
        println!("Введите ваш вариант: ");                  // +-+ сообщения
    
        let mut guess = String::new();              // создание объекта типа String 
     
        io::stdin()                                   // вызов метода read_line() временного объекта типа Stdin c - 
            .read_line(&mut guess) // мутабельной ссылкой на переменную guess в параметре, и -
            .expect("Ошибка ввода!");                  // обработкой возможной ошибки через метод Result.expect()  
     
        let guess: u32 = match guess.trim().parse() {       // затенение guess целочисленной переменной и ее инициализация
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {                   // переключатель вывода сообщений по результату сравнения -
            Ordering::Less => println!("Слишком мало!"),    // загаданного числа и предположенного
            Ordering::Greater => println!("Слишком много!"),
            Ordering::Equal => {
                println!("Вы угадали!!! Секретное число: {secret_number}");
                break;
            }
        }
    }
}