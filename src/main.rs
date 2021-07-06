use std::io;

fn main() {
    println!("insert a word");

    let mut guess = String::new(); //you can change the value
    //let guess2 = String::new(); // unchangeable

    // mut => mutable(changeable)
    // :: => associated
    // => let creat a valuable and we bound it to a new empty String

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    // io::stdin is equal to std::io::stdio if we didn't use std::io at first
    // then it reads the value and append it to the variable instead of overwrite
    // most importantly it returns the value using io::Result
    // & is reference which is the same as C
    // expect is the part that handles the error

    // without expect the code will be compiled with errors

    println!("You typed: {}", guess);
    // something like printf
}