// 구조체 자체를 pub으로 해야 다른 모듈에서 볼 수 있음
// Debug를 derive하여 출력 가능하게 만듦
#[derive(Debug)]
pub struct Asparagus {
    pub id: u32,
    pub quantity: u32,
}

impl Asparagus {
    // 생성자: 외부에서 호출 가능해야 하므로 pub
    pub fn new(id: u32, quantity: u32) -> Self {
        Self { id, quantity }
    }

    // 메서드: 채소 정보를 문자열로 반환
    pub fn describe(&self) -> String {
        format!("아스파라거스 {}묶음 (ID: {})", self.quantity, self.id)
    }
}