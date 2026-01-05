// garden 모듈 안에 vegetables라는 서브모듈이 있다는 선언
// 정의는 파일에 있음: backyard/src/garden/vegetables.rs
// 이 순간부터 컴파일러는 garden 모듈 전용 디렉터리를 찾음
// 그래서 폴더 이름도 반드시 garden/이어야 함
pub mod vegetables;

use vegetables::Asparagus;

// 텃밭 전체를 수확하여 리스트로 반환하는 함수
pub fn harvest_crops() -> Vec<Asparagus> {
    let mut crops = Vec::new();
    // 실제 로직: 여러 개의 아스파라거스를 생성
    crops.push(Asparagus::new(1, 5));
    crops.push(Asparagus::new(2, 10));
    crops
}