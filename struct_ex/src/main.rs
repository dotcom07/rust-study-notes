struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 외부 속성 (outer attribute) 
// 구조체에  디버깅 정보를 출력하는 기능을 적용하려면 명시적인 동의가 필요
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 이 impl 블록 내의 모든 것은 Rectangle 타입과 연관
impl Rectangle {
    // &self == self: &Self

    // 메서드는 다른 매개변수가 그런 것처럼 self의 소유권을 가져올 수도,
    // 지금처럼 self를 불변으로 빌려올 수도, 가변으로 빌려올 수도 있음

    // 연관 함수 (associated function)
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    //self를 첫 매개변수로 갖지 않는 (따라서 메서드가 아닌) 연관 함수를 정의할 수도 있음
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}


// 명명된 필드가 없는 구조체 (tuple struct)
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 필드가 없는 유사 유닛 구조체 (unit-like struct)
struct AlwaysEqual;


fn main() {
    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
    };
    println!("Username: {}", user1.username);

    // 일부 필드만 가변으로 만들 수는 없음
    let mut user2 = User {
        active: user1.active,
        username: user1.username, // 기존 인스턴스에서 소유권이 이동 
        email: String::from("user2@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    // println!("Username: {}", user1.username); // 오류: user1.username의 소유권이 user2로 이동했기 때문
    println!("User2's email before: {}", user2.email);

    user2.email = String::from("user2@example.com");
    println!("User2's email after: {}", user2.email);

    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    println!("User3's username: {}", user3.username);

    // =================================================
    let email = String::from("user23@example.com");
    let username = String::from("user23");

    let user4 = build_user(email, username);
    println!("User4's username: {}", user4.username);


    // =================================================
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    color_print(&black);
    point_print(&origin);

    // =================================================
    let subject = AlwaysEqual;
    assert!(check_permission(subject, AlwaysEqual));


    // =================================================
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // {} 내에 :?를 추가하는 건 println!에 
    // Debug라는 출력 형식을 사용하고 싶다고 전달하는 것

    println!("rect1 is {:#?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    let scale = 2;
    let rect2 = Rectangle {
        // 이는 표현식의 소유권을 가져와서
        // 코드에서 dbg! 매크로를 호출한 파일 및 라인 번호를 결괏값과 함께 출력하고 
        // 다시 소유권을 반환
        // 에러 콘솔 스트림(stderr)에 출력
        width: dbg!(30 * scale),
        height: 50,
    };

    //dbg!가 rect1의 소유권을 가져가는 것은 원치 않으므로 참조자 사용
    dbg!(&rect2);

    // =================================================
    // 메서드
    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );

    if rect2.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // 구조체의 네임스페이스 안에 있는 연관 함수 호출
    let sq = Rectangle::square(3);
}

fn build_user(email: String, username: String) -> User {
    User { // 마지막 표현식에 구조체를 반환하게 할 수 있음
        active: true,
        username: username,
        email, // 필드 초기화 단축 문법 (field init shorthand)
        sign_in_count: 1,
    }
}

fn color_print(color: &Color) {
    println!("Color - R: {}, G: {}, B: {}", color.0, color.1, color.2);
}

fn point_print(&Point(x, y, z): &Point) {
    println!("Point - X: {}, Y: {}, Z: {}", x, y, z);
}

// 트레이트 -> 추후 10장에서 다룸
impl PartialEq for AlwaysEqual {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

fn check_permission<T: PartialEq>(user: T, admin: T) -> bool {
    if user == admin {
        true
    } else {
        false
    }
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}