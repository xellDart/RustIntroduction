fn main() {
    let r = plus_one(5);
    let u = plus_two(5);
    let s = compare_value_one();
    let t = compare_value_two();
    let w = compare_value_three();

    println!("The value of r is: {}", r);
    println!("The value of u is: {}", u);
    println!("The value of s is: {}", s);
    println!("The value of t is: {}", t);
    println!("The value of t is: {}", w);
}
// Return x
fn plus_one(x: i32) -> i32 {
    x + 1
}

// Another form
fn plus_two(x: i32) -> i32 {
    print!("Return x with prev instruction");
    x + 1
}

fn compare_value_one() -> i32 {
    let condition = true;
    if condition {
        return 5
    } else {
        return 6
    };
}

fn compare_value_two() -> i32 {
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    number
}

fn compare_value_three() -> i32 {
    let condition = true;
    if condition {
        return 5
    } 
    return 6
}

// Output: 
Return x with prev instructionThe value of r is: 6
The value of u is: 6
The value of s is: 5
The value of t is: 5
The value of t is: 5
