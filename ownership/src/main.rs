fn main() {

    {
    // 문자열 리터럴은 고정된 크기의 불변 데이터
    let s = "hello";
    } // 이 스코프가 종료되었고, s가 더 이상 유효하지 않음 (drop)

    // 이중 콜론 ::은 우리가 함수를 사용할 때 string_from 같은 함수명을 사용하지 않고 
    // String 타입에 있는 특정된 from 함수라는 것을 지정할 수 있게 해주는 네임스페이스 연산자
    
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    println!("===========================");
    // Copy가 가능한 타입 (정수, 부동 소수점 숫자, 불리언, 문자 등)
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    println!("===========================");
    let s3 = String::from("hello");  // s가 스코프 안으로 들어옴

    takes_ownership(s3);             // s의 값이 함수로 이동
                                    // 따라서 여기서는 더 이상 유효하지 않음

    let x = 5;                      // x가 스코프 안으로 들어옴

    makes_copy(x);                  // x가 함수로 이동되지만,
                                    // i32는 Copy이므로 앞으로 계속 x를
                                    // 사용할 수 있음

    println!("x = {}", x);


    // 참조와 대여

    // 불변 참조자 (immutable reference)
    println!("===========================");
    let si= String::from("hello");
    let len = calculate_length(&si);

    println!("The length of '{}' is {}.", si, len);

    println!("===========================");
    // 가변 참조자 (mutable reference)
    let mut s4 = String::from("hello");
    println!("s4 before change: {}", s4);
    change(&mut s4);
    println!("s4 after change: {}", s4);


    // 슬라이스

    let mut s = String::from("hello world");
    let word : usize = first_word(&s);
    s.clear(); // 근데 만약에 이렇게 문자열을 비움, 이제 s는 빈 문자열
    // 즉, 논리적인 dangling reference, usize와 s의 타입 레벨의 연결이 없어서
    // 컴파일러가 잡아내지 못함


    // 문자열 슬라이스 (string slice)
    let s = String::from("hello world");
    let hello = &s[..5]; // 0부터 시작하면 생략 가능
    let world = &s[6..]; // 끝까지 가면 생략 가능
    println!("hello: {}, world: {}", hello, world);
    
    // s ── immutable borrow ──▶ word
    // 이 상태가 word의 스코프가 끝날 때까지 유지
    let word = first_word_slice(&s);

    // 대여 규칙 : 특정 대상의 불변 참조자가 이미 존재할 경우에는 가변 참조자를 만들 수 없음
    // clear 함수는 String의 길이를 변경해야 하니 가변 참조자가 필요 (fn clear(&mut self))
    s.clear();
    println!("first word: {}", word);


    let my_string = String::from("hello world");

    // `first_word`는 `String`의 일부 혹은 전체 슬라이스에 대해 작동합
    let word = first_word_slice_robust(&my_string[0..6]);
    let word = first_word_slice_robust(&my_string[..]);
    // 또한 `first_word`는 `String`의 전체 슬라이스와 동일한 `String`의
    // 참조자에 대해서도 작동합
    let word = first_word_slice_robust(&my_string);
    let my_string_literal = "hello world";

    // `first_word`는 문자열 리터럴의 일부 혹은 전체 슬라이스에 대해 작동합
    let word = first_word_slice_robust(&my_string_literal[0..6]);
    let word = first_word_slice_robust(&my_string_literal[..]);

    // 문자열 리터럴은 *곧* 문자열 슬라이스이므로,
    // 아래의 코드도 슬라이스 문법 없이 작동
    let word = first_word_slice_robust(my_string_literal);
} 

fn takes_ownership(some_string: String) { // some_string이 스코프 안으로 들어옴
    println!("{}", some_string);
} // 여기서 some_string이 스코프 밖으로 벗어나고 `drop`이 호출
  // 메모리가 해제

fn makes_copy(some_integer: i32) { // some_integer가 스코프 안으로 들어옴
    println!("{}", some_integer);
} // 여기서 some_integer가 스코프 밖으로 벗어남 별다른 일이 발생하지 않음



// 참조와 대여

fn calculate_length(s: &String) -> usize {
    s.len()
} // 여기서 s가 스코프 밖으로 벗어남 
  // 하지만 참조하는 것을 소유하고 있진 않으므로,
  // 버려지지는 않음

// fn change(some_string: &String) {
//     some_string.push_str(", world"); // 오류! 불변 참조자는 값을 변경할 수 없음
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 슬라이스

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes(); // 문자열을 바이트 배열로 변환

    // iter 메서드는 컬렉션의 각 요소를 반환
    // enumerate 메서드는 iter의 각 결괏값을 튜플로 감싸 반환
    // 반환하는 튜플은 첫 번째 요소가 인덱스, 두 번째 요소가 해당 요소의 참조자
    // (usize, &u8) : iter()가 주는 건 참조 (&u8)
    // &item 패턴이 값을 복사해서 꺼냄
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    // 이렇게 하면 item의 타입은 &u8
    for (i, item) in bytes.iter().enumerate() {
        // 따라서 비교하려면 item을 역참조 해야 함
        if *item == b' ' {
            return i;
        }
    }
    s.len() // 공백이 없으면 문자열 전체 길이를 반환
}

// 문자열 슬라이스 (string slice)

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word_slice_robust(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}