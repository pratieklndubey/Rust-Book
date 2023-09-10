fn find_element(array: &[i32], target: i32) -> Option<Result<i32, &'static str>> {
    for (index, &element) in array.iter().enumerate() {
        if element == target {
            return Some(Ok(index.try_into().unwrap())); // Return index as a Result if the target is found
        }
    }
    Some(Err("Element not found")) // Return an error message if the target is not found
}

fn main() {
    let numbers = vec![10, 20, 30, 40, 50];
    let target = 80;

    match find_element(&numbers, target) {
        Some(result) => match result {
            Ok(index) => println!("Element found at index: {}", index+1),
            Err(message) => println!("{}", message),
        },
        None => println!("No error occurred"),
    }

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);

println!("{:?}, {:?}, {:?}", five, six, none);
match six {
    Some(i) => println!("{}",i+2), //can't write expressions inside the braces
    None => println!("No Value"),
}

let dice_roll = 9;
    match dice_roll {
        3 => println!("Add Fancy Hat"),
        7 => println!("Remove Fancy Hat"),
        other => println!("Move {other} spaces on the board") // the match list should be exhaustive
        //  and can use '_' for catch all if don't need to use the value as it doesn't bind
    }

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}


