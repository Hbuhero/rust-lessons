fn main() {
    let x: i32 = -4;

    let y: i32 = 54;

    let addition_answer = addition(x, y);
    let subtraction_answer = subtraction(x, y);

    println!("{addition_answer}, {subtraction_answer}"); 
}

fn addition(x: i32, y: i32) -> i32{
    return x + y;
}

fn subtraction(x: i32, y: i32) -> i32 {
    x - y
}
