fn main() {
    // Primitive types: integer, float, char, bool
    // let x: u8 = b'A';
    // let x: i/u-8/16/32/64/128/size = 2
    // let x: f/32/64 = 5.4
    // let x: bool = true;    
    let x: char = '😻';
    // annotate only when required, it is optional
    println!("The value of x is: {x}");
    // ------------------------------------------
    // Compund Types: tuples & arrays
    // Tuples
    let tup = (500, 6.4, 1); 
    let five_hundred = tup.0;
    let six_point_four = tup.1;    
    let one = tup.2;
    let (x,y,z) = tup;
    println!("{five_hundred}, {six_point_four}, {one}");
    println!("{x}, {y}, {z}");
    // Arrays
    // access single value using []
    let a = [3;5];
    println!("{:?}",a);

}