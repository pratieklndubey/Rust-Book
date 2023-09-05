fn number () -> i8 {
    // return 98; 
    8
}

fn plus_one(x: i8) -> i8 { 
    x+1 // i16, etc as return type won't work as the input is i8 and resultant will also be i8
}

fn main() {
    addition(5,7);
    let x = {let y = 6; y};
    println!("{x}");
    println!("{}",number());
    println!("{}",plus_one(89));
}

fn addition(x: i8, y: i8) {
    println!("{}",x+y);
}


