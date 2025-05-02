// C 표준 라이브러리의 int32_t 타입에 해당하는 Rust 타입을 사용합니다.
use std::os::raw::c_int; // 일반적으로 c_int는 i32와 같습니다.

// 링크할 C 라이브러리 이름을 지정합니다.
#[link(name = "my_math")] // 컴파일된 C 라이브러리 이름 (확장자 제외)
unsafe extern "C" {
    // C 함수의 시그니처를 Rust에 선언합니다.
    // C의 int32_t는 Rust의 i32 또는 std::os::raw::c_int와 호환됩니다.
    fn add_integers(a: c_int, b: c_int) -> c_int;
}

fn main() {
    let num1: c_int = 10;
    let num2: c_int = 20;
    let sum: c_int;

    // C 함수 호출은 안전하지 않으므로 unsafe 블록 안에서 수행합니다.
    unsafe {
        sum = add_integers(num1, num2);
    }

    println!("{}와 {}의 합 (C 함수 결과): {}", num1, num2, sum);
}