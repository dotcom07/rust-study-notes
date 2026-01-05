

// mod front_of_house {
//     // 부모 모듈 내 아이템은 자식 모듈 내 비공개 아이템을 사용할 수 없지만
//     // 자식 모듈 내 아이템은 부모 모듈 내 아이템을 사용할 수 있음
//     pub mod hosting {
//         pub fn add_to_waitlist() {
//             println!("📝 [Hosting] 대기자 명단에 이름을 추가합니다.");
//         }
//         fn seat_at_table() {}
//     }

//     pub mod serving {
//         // [모듈 간 소통] 주방(back_of_house)에서 호출할 서빙 기능들
//         pub fn serve_appetizer(order: &str) {
//             println!("🥗 [Serving] 에피타이저로 '{}'가 나왔습니다.", order);
//         }
        
//         pub fn serve_soup() {
//             println!("🍲 [Serving] 따뜻한 아스파라거스 스프가 나왔습니다.");
//         }

//         pub fn serve_main_dish(toast: &str, fruit: &str) {
//             println!("🍽️ [Serving] 메인: '{}' 토스트와 제철 과일 '{}'입니다.", toast, fruit);
//         }
//     }
// }

// mod back_of_house {
//     // [경로 개념] super를 사용해 부모 모듈로 올라가서 형제 모듈 접근
//     use super::front_of_house::serving;
//     // [외부 의존성 사용]
//     use backyard::garden::vegetables::Asparagus;

//     // [구조체 공개 규칙]
//     // 레스토랑에서 고객이 식사와 같이 나올 빵 종류를 선택하고
//     // 셰프가 계절과 재고 상황에 맞춰서 식사에 포함할 과일을 정하는 상황을 묘사
//     pub struct Breakfast {
//         pub toast: String,      // 공개(Public): 손님이 변경 가능
//         seasonal_fruit: String, // 비공개(Private): 셰프가 결정
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }

//         pub fn seasonal_fruit(&self) -> &str {
//             &self.seasonal_fruit // &self.seasonal_fruit[..]와 동일
//         }
//     }

//     // [열거형 공개 규칙]
//     // enum 앞에 pub을 붙이면 내부의 모든 배리언트(Soup, Salad)가 공개
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }

//     // [로직 통합] 코스 요리 전체를 준비하는 함수
//     pub fn cook_full_course(toast_order: &str) -> Breakfast {
//         println!("\n👨‍🍳 [Chef] 풀 코스 준비를 시작합니다.");

//         // 1. 에피타이저 준비 (Enum 사용)
//         let appetizer = Appetizer::Salad; // 오늘은 샐러드로 결정
//         match appetizer {
//             Appetizer::Soup => serving::serve_appetizer("Soup"),
//             Appetizer::Salad => serving::serve_appetizer("Salad"),
//         }

//         // 2. 스페셜 스프 준비 (외부 크레이트 Backyard 사용)
//         let ingredient: Asparagus = backyard::supply_asparagus();
//         println!("🔪 [Chef] 재료 손질 중... {}", ingredient.describe());
//         serving::serve_soup();

//         // 3. 메인 조식 준비 (Struct 사용)
//         let meal = Breakfast::summer(toast_order);

//         // 계산서(객체) 반환
//         meal
//     }

//     fn fix_incorrect_order() {
//         cook_order();
//         // [상대 경로 super] 내 부모 모듈(crate)에 있는 deliver_order 함수를 호출
//         super::deliver_order();
//     }

//     fn cook_order() {}
// }

// [외부 크레이트 개념] 정원(Backyard)에서 재료(아스파라거스)를 가져옴
use backyard;

// 이렇게 하고 밑에서 pub use를 안 하면
// table에서 restaurant::front_of_house::hosting::put_order();로 써야함
// 즉, 외부(table)는 모든 public 하위 모듈을 직접 따라 들어갈 수 있음
pub mod front_of_house;
mod back_of_house;

// [pub use] 다시 내보내기: 외부(table)에서 hosting에 바로 접근 가능하게 함
// re-exporting
// 근데 위에 pub mod를 안 하면, table에서 restaurant::hosting::put_order();로 쓰면 됨
// 즉, 내부에는 front_of_house라는 구조가 있지만
// 외부에는 ‘hosting만 있는 것처럼’ 보이게 하는 것
pub use crate::front_of_house::hosting;

// 방법 3: 경로 단축을 위해 use 문 사용
// 마치 심볼릭 링크처럼 크레이트 루트에 정의한 거처럼 사용 가능
use crate::front_of_house::serving;

// [통합 시나리오] 손님이 레스토랑을 이용하는 전체 과정
// 수정사항: 인자(toast_choice)를 추가하여 Table에서 빵을 선택할 수 있게 변경
pub fn eat_at_restaurant(toast_choice: &str) {
    // 1. 호스팅 (절대/상대/pub use 경로 사용)
    hosting::add_to_waitlist();

    // 2. 주문 및 서빙 (호밀빵 주문)
    // 주방에서 풀 코스를 요리해서 내오고, 결과물(meal)을 받음
    // 수정사항: 전달받은 toast_choice를 주방장에게 전달
    let mut meal = back_of_house::cook_full_course(toast_choice);

    // 3. 식사 중 변경 요청 (구조체 필드 접근성 테스트)
    // 먹고 싶은 빵 바꾸기 (toast는 pub이라 수정 가능)

    // [시나리오 수정] 서버가 더 좋은 빵을 추천하여 손님이 수락하는 상황
    println!("💁 [Server] 손님, 오늘의 잼에는 주문하신 '{}'보다 'Wheat'이 더 잘 어울립니다. 추천대로 바꿔드릴까요?", meal.toast);
    
    meal.toast = String::from("Wheat");
    println!("🗣️ [Guest] 아 그래요? 그럼 {} 빵으로 바꿔주세요.", meal.toast);

    // 방법 1 : 경로 단축 없이 사용 (절대 경로)
    crate::front_of_house::serving::serve_main_dish(&meal.toast, meal.seasonal_fruit());
    // 방법 2 : 상대 경로 사용
    front_of_house::serving::serve_main_dish(&meal.toast, meal.seasonal_fruit());
    // 방법 3 : use 문으로 경로 단축 후 사용
    serving::serve_main_dish(&meal.toast, meal.seasonal_fruit());


    // 다음 라인의 주석을 해제하면 컴파일되지 않습니다; 식사와 함께
    // 제공되는 계절 과일은 조회나 수정이 허용되지 않습니다
    // meal.seasonal_fruit = String::from("blueberries");
}

// 비공개 함수: 내부 배달용
fn deliver_order() {}

/*
crate
├── front_of_house
│   ├── hosting
│   └── serving
├── back_of_house (Appetizer, Breakfast, cook_full_course)
└── deliver_order
*/