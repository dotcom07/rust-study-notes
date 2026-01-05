// [개념: 외부 패키지 사용하기]
// Cargo.toml에 정의된 restaurant(로컬)와 rand(crates.io) 크레이트 가져오기
use restaurant;
use rand::Rng;

// [개념: 중첩 경로 (Nested Paths)]
// std::io와 std::io::Write를 한 줄로 줄여서 가져오기
// 중복되는 부분을 줄여 코드를 깔끔하게 만듭니다.
use std::io::{self, Write};

// [개념: as 키워드]
// 이름이 같거나 너무 길 때 새로운 이름(별칭) 부여
use std::fmt::Result as FmtResult;

// [개념: 글롭 (Glob) 연산자]
// collections 모듈 내의 *모든* 공개 아이템(HashMap, HashSet 등)을 가져옵니다.
// 테스트 코드나 프렐루드(prelude) 외에는 이름 충돌 위험으로 주의해서 사용해야 합니다.
use std::collections::*;

fn main() {
    println!("=== 🍽️ Table: 저녁 식사 시나리오 시작 ===");
    restaurant::hosting::put_order();
    
    // 1. [외부 크레이트 활용] 랜덤 테이블 번호 생성
    let mut rng = rand::thread_rng();
    let table_num = rng.gen_range(1..=20);
    println!("🎲 [System] {}번 테이블로 안내되었습니다.", table_num);

    // 2. [라이브러리 활용] 레스토랑 코스 요리 주문
    // 우리가 만든 restaurant 라이브러리의 공개 함수 호출
    // 흐름: Table -> Restaurant(주방) -> Backyard(수확) -> Restaurant(요리) -> Table(서빙)
    // ✨ 수정됨: 이제 손님이 원하는 빵 종류("Rye")를 주문 시에 전달합니다.
    restaurant::eat_at_restaurant("Rye");

    println!("\n=== 🧾 Bill Calculation ===");

    // 3. [Glob 활용] HashMap 바로 사용
    // use std::collections::*; 덕분에 HashMap::new()로 바로 접근 가능
    let mut bill = HashMap::new();
    bill.insert("Full Course", 120);
    bill.insert("Wine", 30);

    for (item, price) in &bill {
        println!("💰 {}: ${}", item, price);
    }

    println!("=== ✅ 식사 종료 ===");
}