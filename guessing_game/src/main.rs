use std::io; // io 라이브러리를 스코프로 가져옴
use rand::Rng; // Rng는 난수 생성기를 구현한 메서드들을 정의한 트레이트 (trait)
use std::cmp::Ordering;



fn main() {
    println!("Guess the number!");

    
    // 러스트에서 변수는 기본적으로 불변 (immutable)
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");


    loop {

        println!("Please input your guess.");
    
        // 변경 가능하도록 하려면 변수명 앞에 mut를 추가 (가변(mutable))
        // ::는 new가 String 타입의 연관 함수 (associated function) 임을 나타넴
        // new 함수는 비어있는 새 문자열을 생성
        // 객체가 아니고, 타입에 연관된 함수
        // String이라는 타입의 namespace에 속한 함수
        let mut guess = String::new();

        io::stdin()
            // 참조자도 변수처럼 기본적으로 불변임
            // read_line은 우리가 인수로 넘긴 문자열에 사용자가 입력한 것을 저장할 뿐만 아니라 
            // 하나의 Result 값을 돌려줌 ( enum 타입 )
            .read_line(&mut guess) // read_line 메서드를 호출
            // 가능한 상태 값 = 배리언트 (variant)
            // Result의 variant는 Ok와 Err
            // Result 타입의 값(value) 에 대해 expect 메서드를 호출할 수 있음
            .expect("Failed to read line");


        println!("You guessed: {}", guess);
        //OR
        // println!("You guessed: {guess}");

        // shadowing : guess의 값을 새로운 값으로 가리는 (shadow) 것을 허용
        // parse method : 문자열을 숫자로 변환
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("do not input invalid value");
                continue;
            },
        };

        // Ordering은 열거형이고 Less, Greater, Equal이라는 variant를 가지고 있음
        // match 표현식(Expression) -> 평가(evaluate)되면 값이 반환됨
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => 
            {
            println!("You win!");
            break;
            },
        }
    }
}