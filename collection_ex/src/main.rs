fn main() {
    // 방법 1: 빈 벡터 생성
    let v: Vec<i32> = Vec::new();
    // 방법 2:
    // vec! 매크로는 제공된 값들을 저장한 새로운 Vec을 생성
    let v = vec![1, 2, 3];

    let mut v = Vec::new();

    // 컴파일러는 push를 보는 시점에 타입을 결정
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // 벡터 요소 읽기
    // 방법 1: 인덱싱
    let third: &i32 = &v[2];
    println!("The third element is {}", third);


    // 러스트에는 널이 없지만, 값의 존재 혹은 부재의 개념을 표현할 수 있는 열거형
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    // 방법 2: get 메서드
    // get 함수는 Option<T> 타입을 반환
    // 요소가 존재하지 않아도 패닉이 없음
    let third: Option<&i32> = v.get(2);
    
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    
    
    let mut v = vec![1, 2, 3, 4, 5];

    // let first = &v[0]; // 불변 참조자

    // let mut first = v[0]; -> 이건 변수에 mut가 붙는 것이라
    // first라는 바인딩을 다른 값으로 재할당할 수 있다는 뜻

    let first_mut = &mut v[0]; // 가변 참조자
    *first_mut += 10;

    // 불변 참조가 남아있는 동안에는 가변 참조자를 만들 수 없음
    // 사실 가변 참조가 남아있어도 불변 참조자를 만들 수 없음
    println!("The first element is: {}", first_mut); // -> 대신 여기에 하면 가능 (borrow lifetime 종료)
    v.push(6); // v에 대한 가변 borrow를 요구 -> 만약 연속된 가상 메모리 공간이 없다면?
    // 새로운 메모리 공간을 할당하고 기존 데이터를 복사해야 함
    // 그럼 원래 있던 first or first_mut 참조자는 더 이상 유효하지 않음
    
    // println!("The first element is: {first_mut}"); -> 여기는 불가능

    // 벡터 순회
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    println!("=============");
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{i}");
    }

    println!("=============");
    let mut v = vec![100, 32, 57];
    let len = v.len();
    let mut idx = 0;
    while idx < len {
        // 매 반복마다 '서로 다른 한 원소'에 접근
        let i: &mut i32 = &mut v[idx];
        *i += 50;
        println!("{:p}", i);
        println!("{i}"); // == println!("{}", *i);
        idx += 1;
    }

    // 열거형을 이용해 여러 타입 저장하기
    // 러스트가 컴파일 타임에 벡터 내에 저장될 타입이 무엇인지 알아야 하는 이유는 
    // 각 요소를 저장하기 위해 얼마만큼의 힙 메모리가 필요한지 알아야 하기 때문
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}