// [경로 개념] super를 사용해 부모 모듈로 올라가서 형제 모듈 접근
use super::front_of_house::serving;
// [외부 의존성 사용]
use backyard::garden::vegetables::Asparagus;

// [구조체 공개 규칙]
// 레스토랑에서 고객이 식사와 같이 나올 빵 종류를 선택하고
// 셰프가 계절과 재고 상황에 맞춰서 식사에 포함할 과일을 정하는 상황을 묘사
pub struct Breakfast {
    pub toast: String,      // 공개(Public): 손님이 변경 가능
    seasonal_fruit: String, // 비공개(Private): 셰프가 결정
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }

    pub fn seasonal_fruit(&self) -> &str {
        &self.seasonal_fruit // &self.seasonal_fruit[..]와 동일
    }
}

// [열거형 공개 규칙]
// enum 앞에 pub을 붙이면 내부의 모든 배리언트(Soup, Salad)가 공개
pub enum Appetizer {
    Soup,
    Salad,
}

// [로직 통합] 코스 요리 전체를 준비하는 함수
pub fn cook_full_course(toast_order: &str) -> Breakfast {
    println!("\n👨‍🍳 [Chef] 풀 코스 준비를 시작합니다.");

    // 1. 에피타이저 준비 (Enum 사용)
    let appetizer = Appetizer::Salad; // 오늘은 샐러드로 결정
    match appetizer {
        Appetizer::Soup => serving::serve_appetizer("Soup"),
        Appetizer::Salad => serving::serve_appetizer("Salad"),
    }

    // 2. 스페셜 스프 준비 (외부 크레이트 Backyard 사용)
    let ingredient: Asparagus = backyard::supply_asparagus();
    println!("🔪 [Chef] 재료 손질 중... {}", ingredient.describe());
    serving::serve_soup();

    // 3. 메인 조식 준비 (Struct 사용)
    let meal = Breakfast::summer(toast_order);

    // 계산서(객체) 반환
    meal
}

fn fix_incorrect_order() {
    cook_order();
    // [상대 경로 super] 내 부모 모듈(crate)에 있는 deliver_order 함수를 호출
    super::deliver_order();
}

fn cook_order() {}