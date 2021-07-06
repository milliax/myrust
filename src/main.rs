//use std::io;

fn main() {
    let mut x = 1;
    // we need mut to let variable mutable
    println!("x is set to {}", x);
    x = 6;
    println!("x is changed to {}", x);
    x = x * 6;
    println!("x is changed to {}", x);
    x = x / 7;
    println!("x is changed to {}", x);

    // in rust variables are a little different from c
    // rust force all variable set to constant (immutable)
    // if we need value to be changed we need to add "mut" when declared

    let foo = "empty";
    println!("foo is set to '{}'",foo);
    let foo = foo.len();
    println!("foo is {}", foo);
    let foo = "I am confused";
    println!("foo is now '{}'",foo);
    // foo is first set to a string
    // then another foo is set to a number
    // then foo is set to a string again
    // Why, it just change the whole thing?
    // This process is called shadowing
    // the new foo will be replaced by another new foo
    // because rust thinks they are different
    // though simply declare a variable can' t be modified
    // you can just declare another one with the same name and replace it

    const FOO:&str = "constant";
    println!("FOO is {}",FOO);
    // const FOO:&str = "another";
    // The correct way for constant, you can never overwrite it
    // constant names in rust is UPPER_CASE in convention
    let mut foo2 = "empty";
    println!("foo2 is {}",foo2);
    foo2 = "things";
    println!("foo2 is {}",foo2);
    //fOO = 100;
    // you can not change a str to num
    //println!("fOO is {}",fOO);

}