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

    /*
    Difference between "String" and "&str"

        String:
            What it is?
                1. A heap-allocated, growable string
                2. Owns its data
                3. Mutable (if declared mut)
            Properties:
                1. Stored on heap
                2. Has ownership
                3. Can grow/shrink
                4. More memory overhead (capacity management)

        &str:
            What it is:
                1. A borrowed reference to a string
                2. Does NOT own the data
                3. Fixed size (cannot grow)
            Properties:
                1. Points to existing string data
                2. Stored as:
                    a. pointer
                    b. length
                3. Cannot modify contents
                4. Lightweight
    */

    // Conditionals ------------------------------------------------------------

    let age: i8 = 19;

    if age > 18 {
        println!("You are eligible!")
    } else {
        println!("You are not eligible!")
    }

    // Loops -------------------------------------------------------------------

    for i in 0..20 {
        print!("{}, ", i);
    }
    println!();

    // Iterating over array

    let arr: [i32; 5] = [2, 4, 3, 1, 20];

    for i in arr {
        print!("{}, ", i);
    }

    println!();

    // Iterating over string (Approach - 1)
    let itr_str: String = String::from("Your name is ANT!");

    for ch in itr_str.chars() {
        if ch != ' ' {
            print!("{}", ch);
        } else {
            break;
        }
    }

    println!();

    // Iterating over string (Approach - 2)

    let res: String = find_first_word(&itr_str);
    println!("{}", res);

    // Understanding Ownership -----------------------------------------------------------


    let str: String = String::from("I am Lalit");
    println!("{}", str);


    let new_str: String = str;
    println!("{}", new_str);

    // println!("{}", str); // This line will throw error

}

fn find_first_word(sentence: &str) -> String {
    let mut ans: String = String::new();

    for ch in sentence.chars() {
        if ch != ' ' {
            ans.push(ch);
        } else {
            break;
        }
    }
    return ans;
}
