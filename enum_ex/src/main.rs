enum IpAddrKind {
    V4,
    V6,
}

struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}


enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit, // 연관된 데이터가 없음
    Move { x: i32, y: i32 }, // 이름이 있는 필드가 있는 익명 구조체
    Write(String), // 단일 String을 포함
    ChangeColor(i32, i32, i32), // 세 개의 i32를 포함
}

impl Message {
    fn call(&self) {
    }
    
}

// 러스트에는 널이 없지만, 값의 존재 혹은 부재의 개념을 표현할 수 있는 열거형
// enum Option<T> {
//     None,
//     Some(T),
// }

// =============================================================
// match

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --생략--
}

fn main() {
    // 열거형을 정의할 때의 식별자로 네임스페이스가 만들어지기 때문에 
    // :: 연산자를 사용하여 열거형의 각 variant에 접근
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // IpAddr::V4()는 String 인수를 입력받아서 
    // IpAddr 타입의 인스턴스 결과를 만드는 생성자 함수처럼 동작
    let home2 = IpAddr::V4(127, 0, 0, 1);
    let loopback2 = IpAddr::V6(String::from("::1"));


    let m = Message::Write(String::from("hello"));
    m.call();

    // Option<T> 열거형 사용 예시
    // Option은 프렐루드에 정의되어 있어 별도의 가져오기 없이 사용 가능 
    // (즉 Option:: 를 붙이지 않고도 사용 가능)
    let some_number = Some(5); // Option<i32>

    let some_char = Some('e'); // Option<char>

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y;; // 오류 발생: Option<i8>과 i8은 더할 수 없음
    let sum = x + y.unwrap(); // Option에서 값을 꺼내서 사용하려면 unwrap() 사용
    println!("Sum: {}", sum);


    // =============================================================
    // match
    let coin = Coin::Penny;
    let coin_value = value_in_cents(coin);
    println!("Coin value: {} cents", coin_value);
    let coin2 = Coin::Quarter(UsState::Alaska);
    let coin_value2 = value_in_cents(coin2);
    println!("Coin value: {} cents", coin_value2);

    // Option<T>를 이용하는 매칭
    let five = Some(5);
    let six = plus_one(five);
    let _none = plus_one(None);

    // 포괄 패턴과 _ 자리표시자
    // Catch-All Patterns and the _ Placeholder
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // catch-all 패턴 (other)은 모든 다른 값에 대해 실행
        other => move_player(other),
    }

    let dice_roll_2 = 9;
    match dice_roll_2 {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        // 어떠한 값이든지 매칭되지만, 그 값을 사용하지 않을 때는 _ 사용
        _ => reroll(), // or just _ => () : unit 타입 반환
    }

    // =============================================================
    // if let 구문
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (), // annoying boilerplate code
    }

    // if let을 사용하면 위의 match 구문을 더 간단히 표현 가능
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    let coin = Coin::Penny;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}


fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
    }
}
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}

fn route(ip_kind: IpAddrKind) {}