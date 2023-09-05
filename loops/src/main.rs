

fn main() {
    // loop, while and for
    let mut cnt = 0;
    let result = loop {
        println!("Again {cnt}");
        cnt += 1;
        if cnt == 4 {
            break cnt * 8;            
        } // will run forever if no condition
    };
    println!("{result}"); // cnt * 8 is the return/last expression
    //______________________________________________________
    // if there is a loop within a loop we can use lables to specify for break and continue
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
    //______________________________________________________
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("Liftoff!!");

    //______________________________________________________
    // For loop is used to eleminate condtional statements
    let arr = [1,2,3,4,5];
    for n in arr{
        println!("{n}");
    }

    //______________________________________________________
    for q in 11..14 { // ascending to descending only for reverse use .rev()
        println!("{q}");
    }


}
