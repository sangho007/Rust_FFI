# Rust_FFI

### C 코드 설명:

```c
#include <stdint.h> // int32_t 타입을 사용하기 위해 포함

// C 함수: 두 개의 32비트 정수를 더합니다.
int32_t add_integers(int32_t a, int32_t b) {
    return a + b;
}
```

* 이 C 코드는 `add_integers`라는 이름의 간단한 함수를 정의합니다.
* `#include <stdint.h>`: `int32_t`와 같이 크기가 명확하게 정의된 정수 타입을 사용하기 위해 표준 헤더 파일을 포함합니다. `int32_t`는 32비트 크기의 정수 타입을 의미합니다.
* `int32_t add_integers(int32_t a, int32_t b)`: 함수 `add_integers`는 두 개의 `int32_t` 타입의 매개변수 `a`와 `b`를 받아들여, 그 합을 `int32_t` 타입으로 반환합니다.
* `return a + b;`: 입력받은 두 정수의 합을 계산하여 반환합니다.

### Rust 코드 설명:

```rust
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
```

이 Rust 코드는 위에서 정의한 C 함수 `add_integers`를 호출하는 방법을 보여줍니다. 이는 PPT에서 설명하는 **Foreign Function Interface (FFI)** 와 **Unsafe Rust** 개념을 활용합니다.

* `use std::os::raw::c_int;`: C 언어의 `int` 타입과 호환되는 Rust 타입인 `c_int`를 가져옵니다. 주석에 명시된 대로 `c_int`는 일반적으로 Rust의 `i32`와 같으며, 이는 C의 `int32_t`와 호환됩니다.
* `#[link(name = "my_math")]`: Rust 컴파일러에게 `my_math`라는 이름의 외부 라이브러리(앞서 작성한 C 코드를 컴파일한 결과물)와 링크하도록 지시합니다.
* `unsafe extern "C" { ... }`: 외부 C 함수를 Rust 코드에서 사용하기 위해 선언하는 블록입니다.
    * `extern "C"`는 C 언어의 호출 규약(ABI)을 따르도록 지정합니다.
    * `fn add_integers(a: c_int, b: c_int) -> c_int;`는 호출하려는 C 함수 `add_integers`의 시그니처(이름, 매개변수 타입, 반환 타입)를 Rust에 알려줍니다.
    * 이 블록 자체는 `unsafe` 키워드로 표시되는데, 이는 외부 함수 호출이 Rust의 메모리 안전성 보장을 벗어날 수 있기 때문입니다.
* `fn main() { ... }`: Rust 프로그램의 진입점입니다.
* `let num1: c_int = 10;` / `let num2: c_int = 20;`: C 함수에 전달할 `c_int` 타입의 변수를 선언하고 초기화합니다.
* `unsafe { ... }`: 외부 함수 호출과 같이 Rust 컴파일러가 안전성을 보장할 수 없는 작업을 수행할 때 필요한 블록입니다. PPT에서도 외부 C 함수 호출 시 `unsafe` 블록 사용 예시를 보여줍니다.
    * `sum = add_integers(num1, num2);`: `unsafe` 블록 내에서 앞서 선언한 C 함수 `add_integers`를 호출하고 그 결과를 `sum` 변수에 저장합니다.
* `println!(...)`: 결과를 출력합니다.

요약하자면, 이 코드는 Rust의 FFI 기능을 사용하여 미리 컴파일된 C 라이브러리의 함수를 호출하는 예시입니다. Rust는 `extern "C"` 블록을 통해 C 함수 시그니처를 이해하고, `unsafe` 블록 내에서 해당 함수를 호출하여 C와 Rust 코드 간의 상호 운용성을 달성합니다.

<img width="563" alt="image" src="https://github.com/user-attachments/assets/34d87b29-dff9-4e31-9597-6091665bed0c" />
