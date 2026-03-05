fn main() {
    // Integer
    let x: i32 = 5;
    let y: i32 = 5;

    println!("x * y: {}", x * y);

    // Boolean -----------------------------------------------------------------------------
    let mut flag: bool = true;
    println!("Flag: {}", flag);

    flag = false;

    println!("Flag: {}", flag);

    // String ------------------------------------------------------------------------------
    let str: String = String::from("Learning Rust!");

    println!("String: {}", str);

    // Accessing character at an index in string
    let ch: Option<char> = str.chars().nth(100);

    // Checking character is abailable at "nth" index safely by pattern matching
    match ch {
        Some(c) => println!("{}", c),
        None => println!("No character is found!"),
    }

    // Conditionals ------------------------------------------------------------

    let age: i8 = 19;

    if age > 18 {
        println!("You are eligible!")
    } else {
        println!("You are not eligible!")
    }

}
