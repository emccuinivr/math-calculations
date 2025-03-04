fn main() {
    let x = 5;
    let y = 3;
    println!("The result of x + y is {}", add(x, y));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
