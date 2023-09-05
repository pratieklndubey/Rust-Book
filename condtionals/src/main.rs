fn main() {
    let number  = 8;
    if number < 5{
        println!("Condition met");
    }
    else {
        println!("Condition didn't meet");
    }
    // ternary operations are performed using if and else
    let condition = true;
    let number = if condition {6i8} else {9i8}; //arms/options should be of the same type
    println!("{number}");
}
