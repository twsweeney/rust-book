fn main() {
    let input: i32 = 24;
    let x1 = add_v1(input);
    let x2 = add_v2(input);


    println!("The value of x  version 1 is: {x1}");
    println!("The value of x  version 2 is: {x2}");
}

fn add_v1(x: i32) -> i32 {
    x +3 // works since it returns a type of i32 since this is an expression
}

fn add_v2(x: i32) -> i32 {
    let output: i32 = x + 3;
    return output
}