// fn main() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }

// 상수는 let 키워드 대신 const 키워드로 선언하며, 값의 타입은 반드시 명시
// const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// fn main() {
    
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");

//     println!("Constant value: {THREE_HOURS_IN_SECONDS}");
// }

use std::io; // io 라이브러리를 스코프로 가져옴


fn main() {
    let x = 5;

    let x = x + 1; // shadowing

    {
        let x = x * 2; // shadowing in inner scope
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("Number of spaces: {spaces}");

    // let mut spaces = "   ";
    // spaces = spaces.len(); // error: expected `&str`, found `usize`

    // 러스트는 정적 타입의 (statically typed) 언어
    // 모든 변수의 타입이 컴파일 시점에 반드시 정해져 있어야 함


    // 스칼라 타입 : 정수, 부동 소수점 숫자, boolean, 문자

    // 정수형
    
    let a: u32 = 1_000; // 32비트 부호 없는 정수 
    let b: i64 = -20; // 64비트 부호 있는 정수
    let c: i8 = 0b1011_0000u8 as i8; // 이 값은 176이라는 수가 아니라 u8 비트 패턴이고, 그 비트를 그대로 i8로 재해석
    println!("a: {}, b: {}, c: {}", a, b, c);

    // floating-point

    let d = 2.0; // f64
    let e: f32 = 3.0; // f32
    println!("d: {}, e: {}", d, e);


    // 수치 연산

    let sum = 5 + 10;

    let difference = 95.5 - 4.3;

    let product = 4 * 30;

    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // 결괏값은 -1

    let remainder = 43 % 5;
    println!("sum: {}, difference: {}, product: {}, quotient: {}, truncated: {}, remainder: {}", sum, difference, product, quotient, truncated, remainder);

    // boolean
    let t = true;
    let f: bool = false; // 타입 명시적 선언
    println!("t: {}, f: {}", t, f);

    // char
    let c1 = 'z'; // 4 bytes
    let z: char = 'ℤ'; // 명시적인 타입 어노테이션
    let heart_eyed_cat = '😻';

    println!("c1: {}, z: {}, heart_eyed_cat: {}", c1, z, heart_eyed_cat);

    // compound types
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x1, y1, z1) = tup; // destructuring
    println!("The value of y1 is: {}", y1);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("five_hundred: {}, six_point_four: {}, one: {}", five_hundred, six_point_four, one);

    let unit_tuple: () = (); // unit type
    println!("unit_tuple: {:?}", unit_tuple);

    // array
    let ar = [1, 2, 3, 4, 5];
    let aa = [3; 5]; // [3, 3, 3, 3, 3]
    println!("ar: {:?}, aa: {:?}", ar, aa);

    let first = ar[0];
    let second = ar[1];
    println!("first: {}, second: {}", first, second);

    // println!("Please enter an array index.");
    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line");

    // let index : usize = index.trim().parse().expect("Index entered was not a number");

    // let element = ar[index]; // 런타임 에러 발생 가능

    // println!("The value of the element at index {index} is: {element}");


    another_function(plus_one(five()), five());

    expression_example();


    let number = 3;
    // JavaScript 같은 언어와 달리 러스트는 부울린 타입이 아닌 값을 부울린 타입으로 자동 변환하지 않음
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // let 구문에서 if 사용하기
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    
    // loops
    
    loops();

}


fn another_function(x: i32, y: i32) {
    println!("The value of x is: {x}");
    println!("The value of y is: {y}"); 
}


// 표현식 (expression)값을 평가(evaluate) 해서 결과를 만든다
// 구문 (statement)	어떤 동작을 수행하지만 값을 반환하지 않는다

/*

코드 블록 { ... } 자체도 표현식

{
    statement1;
    statement2;
    expression
}


Rust에서는 “값을 반환하는 표현식”은 반드시 블록의 마지막에만 올 수 있음

*/

fn expression_example() {
    let y = {
        let x = 3;
        x + 1 // 세미콜론(;)이 없으므로 표현식
    };

    println!("The value of expression_example y is: {y}"); // 출력 값은 4

    let yy = if true {
        println!("hello");
        5
    } else {
        6
    };
}

fn five() -> i32 {
    5 //  러스트에서 함수의 반환 값은 함수 본문의 마지막 표현식의 값과 동일
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn loops() {
        let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");



    // 중첩된 루프와 레이블

    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");

        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 7 {
                break; // 내부 루프만 종료
            }
            if count == 2 {
                break 'counting_up; // 외부 루프를 종료
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");


    // while를 이용한 반복문
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");

    
    
    // for를 이용한 컬렉션에 대한 반복문

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

        
    for number in (1..5) {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}