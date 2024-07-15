use std::io;

fn main() {
    // mutability();
    // shadowing();
    // shadowing_1();
    // datatypes();
    // numeric_operations();
    // compound_types();
    age();
}

// mutability
fn mutability() {
    let mut n = 100;

    for i in 0..=n {
        n += i;
        println!("{i}");
    }
    print!("value of n is : {n}")
}
// ** datatype of a variable can be changes while shadowing but if we use mutable variable datatype of that variable can't be changed **
// shadowing
fn shadowing() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}

// shadowing - 1
fn shadowing_1() {
    // datatype can be changed
    let spaces = "   ";
    let spaces = spaces.len();
    print!("{spaces}")
    // datatype cannot be changed, as we are using a mut variable
    // let mut spaces_1 = "   ";
    // spaces_1 = spaces_1.len();
}

fn datatypes() {
    let guess: u32 = "42".parse().expect("Not a number!");
    print!("{guess}")

    // int
    // Length	  Signed	Unsigned
    // 8-bit	  i8	    u8
    // 16-bit	  i16	    u16
    // 32-bit	  i32	    u32
    // 64-bit	  i64	    u64
    // 128-bit	i128	  u128
    // arch	    isize	  usize
    // -(2*(n-1)) - 2*(n-1)-1

    // float
    // bool
    // char
}

fn numeric_operations() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;

    // remainder
    let remainder = 43 % 5;

    print!("{sum}, {difference}, {product}, {quotient}, {truncated}, {remainder}")
}

fn compound_types() {
    // tuple
    let tup = (500, 6.4, 1);

    // A tuple variable can be accessed with the name of the tuple followed by a '.' and the index
    print!("First value of the tuple tup is : {}\n", tup.0);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    // array
    let a = [1,2,3,4,5];
    let b = [17;3];

    print!("{}, {}", a[0], b[0]);
}

fn age() {
  println!("please enter your age : ");

  let mut age = String::new();

  io::stdin()
        .read_line(&mut age)
        .expect("Failed to read your age");

  let age: u32 = age.trim()
                .parse()
                .expect("Age you entered was not a number");
  if age > 18 {
    println!("You are eligible to vote as your age is {age} and is above 18.");
  } else {
    print!("You are not eligible to vote as your age is {age} and is below 18");
  }
}