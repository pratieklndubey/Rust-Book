struct  Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height*self.width
    }   
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 40,
    };
    println!("The area of this rect is {} sq pixels.",rect.area());
    println!("Can rect2 fit in rect? {}", rect.can_hold(&rect2));
    println!("Can rect3 fit in rect? {}", rect.can_hold(&rect3));
}
