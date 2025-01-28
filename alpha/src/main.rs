fn main() {
    let mut s = String::from("value");
    let s_ref = &mut s;

    

    // s = s_again(s); // uncommenting this will make an error. This is because s will be moved and the reference will be dangling

    // let s1 = s; // here s is moved to s1 hence s no longer points to the string value

    some_string(s_ref); // so here s is moved to the variable declared in the function parameters

    let x = 3;

    some_number(x);

    println!("{s}");

    let y = x; // this is basically a copy hence both x and y points to individual 5 value to memory
    
}


fn some_string(s: &mut String) {
    println!("{s}");
    s.push_str("string");
}

fn some_number(x: i32){
    println!("{x}");
}

fn s_again(s: String) -> String {
    s
}
