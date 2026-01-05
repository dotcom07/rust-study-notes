// 하나의 패키지 안에 src/lib.rs(라이브러리)와 src/main.rs(바이너리)가 공존할 수 있음
// 이제 backyard는 실행도 되고, 라이브러리로 납품도 됨


// [backyard/src/main.rs]
use backyard::supply_asparagus; // 내 라이브러리 함수 사용

fn main() {
    let crop = supply_asparagus();
    println!("농장 주인: 오늘 수확물은 {}입니다.", crop.describe());
}