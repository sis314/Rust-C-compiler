#[derive(Debug, PartialEq, Eq)]
enum A {
    NUM(i32),
}
fn main() {
    let a = A::NUM(24);
    let b = A::NUM(12);
    let c = A::NUM(24);
    println!("{} {}", a == b, a == c);
}
