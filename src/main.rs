use std::io;

fn main(){
    println!("please input a num!");

    // Array & Tuple
    /*let arr = [93_22_2,0xff,0o77];
    // underline means nothing. it is just like a common in and writing. For the better understanding
    // let tuple: (i32,i32,i32,u8) = (93_22_2,0xff,0o77,b'A');
    // simple way
    let tuple = (93_22_2,0xff,0o77,b'A');
    let (a,b,c,d) = tuple;
    //println!("{}",tuple); not permitted
    println!("{}、{}、{}、{}",a,b,c,d);
    println!("{}、{}、{}、{}",tuple.0,tuple.1,tuple.2,tuple.3);*/

    // formatted array
    /*let months:[&str;12] = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    for month in months{
        println!("{}",month);
    }*/
    /*let mut number=String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read the line");
    let number: isize = number
        .trim()
        .parse()
        .expect("index entered was not a number");
    println!("the number you entered is {}",number);*/

    let mut string = String::new();
    string += "                          Love            ";
    string = string
        .trim().parse()
        .expect("index transfer failed");
    println!("string value is {}",string);
}