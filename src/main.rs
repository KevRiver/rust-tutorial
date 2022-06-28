fn main() {
    let immutable_box = Box::new(3i32);
    println!("immutable_box contains {}", immutable_box);

    let mut mutable_box = immutable_box;
    println!("mutable_box contains {}", mutable_box);

    //println!("immutable_box contains {}", immutable_box);
    // ^위 코드는 컴파일 에러를 발생시킴

    *mutable_box = 4;
    println!("now mutable_box contains {}", mutable_box);
}
