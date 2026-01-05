// [모듈 간 소통] 주방(back_of_house)에서 호출할 서빙 기능들
pub fn serve_appetizer(order: &str) {
    println!("🥗 [Serving] 에피타이저로 '{}'가 나왔습니다.", order);
}

pub fn serve_soup() {
    println!("🍲 [Serving] 따뜻한 아스파라거스 스프가 나왔습니다.");
}

pub fn serve_main_dish(toast: &str, fruit: &str) {
    println!("🍽️ [Serving] 메인: '{}' 토스트와 제철 과일 '{}'입니다.", toast, fruit);
}