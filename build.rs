// build.rs

fn main() {
    // my_math.c 파일을 컴파일합니다.
    // 기본적으로 정적 라이브러리 (libmy_math.a)를 생성합니다.
    cc::Build::new()
        .file("my_math.c") // C 소스 파일 경로 (프로젝트 루트 기준)
        // 만약 src 디렉토리에 있다면 .file("src/my_math.c")
        .compile("my_math"); // 출력 라이브러리 이름 (libmy_math.a 생성)

    // Cargo에게 my_math 정적 라이브러리를 링크하도록 지시합니다.
    println!("cargo:rustc-link-lib=static=my_math");

    // Cargo에게 컴파일된 라이브러리가 어디 있는지 알려줍니다.
    // cc::Build는 기본적으로 OUT_DIR 환경 변수가 가리키는 곳에 라이브러리를 생성합니다.
    println!("cargo:rustc-link-search=native={}", std::env::var("OUT_DIR").unwrap());

    // --- 만약 공유 라이브러리(.dylib/.so)를 사용하고 싶다면 ---
    // 참고: 공유 라이브러리는 런타임 경로 설정 등 추가 작업이 필요할 수 있습니다.
    /*
    cc::Build::new()
        .file("my_math.c")
        .shared_flag(true) // 공유 라이브러리 빌드 시도 (플랫폼 의존적)
        .compile("my_math"); // 라이브러리 이름

    println!("cargo:rustc-link-lib=dylib=my_math"); // dylib 또는 so (플랫폼에 따라)
    println!("cargo:rustc-link-search=native={}", std::env::var("OUT_DIR").unwrap());
    */
}