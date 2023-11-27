// fn main() {
//     // let age:i8 = 10;
//     // let mut ages: i32 = 10;
//     // ages = 432423;
//     // age = 20 - ошибка, она не мутабельна. Чтобы сделать это, нужно указать mut
//     const AGE:f32 =423423.3232;
//     let mut ages: f64 = 452342343.543534543534;


//     ages  = 3424234.234234;

//     println!("Hello, world!");
//     println!("{}", AGE);
//     println!("{}", ages);
// }

// i32 -  12 33423 5345234123 - Целые числа, i - инт 32 - количество битов в системе занимающее. Чем меньший размер - тем меньший. Есть i8 i16 i32 i64 i128 isize. Что такое isize - это переменная которая с таким значением и такими же битами, сколько операционная система, 64 - бита на 64 битовой системе
// float - 12.33423 12.53452341 - Плавающая точка


// У флота 2 разрядности 32 и 64


// fn main () {
//     let mut name: &str = "John"; // - не безопасное
//     let mut names = String::from("STRING"); //Безопасное создание
//     println!("UnCorrect string {}", name);
//     println!("Correct string {}", names)
// }

// u8 u16 u32 u64 u128 usize в чем разница от i от u , целые числа могут быть отрицательными, u только положние +> 0

// Bool str

// char -  Символьный тип, мы можем хранить только 1 символ. Какие особенности есть?
// bool - Boolean  логический тип данных, logic true or false , 1 or 0, утверждение 5 > 4 true, 5 < 4 false
// fn main () {
//     let symbol: char = 'a';
//     let boo: bool = bool(1);
//     let text = String::from("Smth");
//     println!("{}", text);
//     println!("{}", symbol);
// }


// Циклы это какие-то функции которые позволяют нам контроллировать входящие данные
// fn main () {
//     let mut nam: i16 = 1;
//     while nam < 100 {
//         if nam % 2 == 0 {
//             println!("{}", nam % 2)
//         }
//         nam += 1;
//     }

//     for i in 1..101 {
//         if i % 2 == 0 {
//             println!("{}", i)
//         }
//     }
//     println!("{}",nam)
//     // let age: i128 = 5;
//     // if age >= -18 {
//     //     println!("{}", age)
//     // } else {
//     //     println!("Пошел ка нахуй")
//     // }

//     // || - или , && - и

//     // if age > 17 {
//     //     println!()
//     // } else if age < 18 {
//     //     println!("Hagaga")
//     // } else if age > 19 {
//     //     println!("ghagagas")
//     // }


//     // let mut nam: i8 = 1;

//     // // Loop - бесконечный цикл
//     // loop {
//     //     if nam == 100 {
//     //         break;
//     //     }
//     //     nam += 1;
//     //     println!("{}", nam)
//     // }

//     // while nam < 127 {
//     //     println!("{}", nam);
//     //     nam += 1;
//     // }
// }

// fn main () {
//     // match - rust? switch

//     // Диапазон чисел , способ 1..12 не проконает , раньше работало 10...12/ Щас чтобы показать диапозон 12..=50 с Диапазоном 12 до =50  

//     let num = 12;
//     match num {
//         10 => {
//             println!("10");
//             println!("10");
//         },
//         12 => {
//             println!("10");
//             println!("12");
//         },
//         20 => {
//             println!("20");
//             println!("20");
//         },
//         _ => if num == 23 {
//             println!("23")
//         } else {
//             println!("21")
//         }
//     }

//     // let numx = 12;
//     // let numy = match numx {
//     //     10 => 1,
//     //     11 => 2,
//     //     _ => 3,
//     // };

//     // Чтобы сделать зависимость переменной от переменной, нужно использовать
    
//     let is_bo: bool = true;
//     let mut str_m: String = String::new();

//     match is_bo {
//         true => {
//             str_m = String::from("new Value");
//         },
//         false => {
//             str_m = String::from("old Value");
//         },
//     }

//     println!("{}", str_m);
// }

// Работа с юзер данными, вводом
// Импортирование библиотек
// use std::io;

// fn main () {
//     // io - i- input o - output
//     let mut name = String::new();

//     println!("Введите имя");

//     match io::stdin().read_line(&mut name) {
//         Ok(_) => {},
//         Err(e) => {
//             println!("Error server {}", e)
//         }
//     }
// }


use std::io;

fn main() {

    let mut a_str: String = String::new();
    let mut b_str: String = String::new();
    let mut c_str: String = String::new();

    println!("a - x2 , b -x , c - number");
    
    println!("a - x2");
    match io::stdin().read_line(&mut a_str) {
        Ok(_) => {},
        Err(e) => {
            println!("Error {}", e);
        }
    }
    
    println!("b-x");
    match io::stdin().read_line(&mut b_str) {
        Ok(_) => {},
        Err(e) => {
            println!("Error {}", e);
        }
    }
    
    println!("c - number");
    match io::stdin().read_line(&mut c_str) {
        Ok(_) => {},
        Err(e) => {
            println!("Error {}", e);
        }
    }

    let a: f64 = a_str.trim().parse().unwrap();
    let b: f64 = b_str.trim().parse().unwrap();
    let c: f64 = c_str.trim().parse().unwrap();

    let d: f64 = (b*b) - 4.0 * (a * c);

    
        if d > 0.0 {
            let x1: f64 = (-b + d.sqrt()) / (2.0 * a);
            let x2: f64 = (-b - d.sqrt()) / (2.0 * a);
            println!("x1={},\nx2={}\n", x1, x2);
        };

        if d == 0.0 {
            let x: f64 = -b / (2.0 * a);
            println!("x корень\n = {}", x);
        };

        if d < 0.0 {
            println!("no real roots");
        }
}