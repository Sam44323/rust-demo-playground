// defining the type for Object struct
#[derive(Debug)]
struct Object {
    width: u32,
    height: u32,
}

// adding method to a struct using impl keyword(basically creating a namespace based on the struct Object [like class])
// a struct can have multiple implementation(but neither same methods and associated methods)

impl Object {
    // when a method doesn't have any self parameter, it is known as an associated functions i.e. an instance of that function is not created for the object based on the struct then it acts like a constructor
    fn new(width: u32, height: u32) -> Object {
        Object { width, height }
    }
}

// implementation for methods
impl Object {
    fn show(&self) {
        println!(
            "{} x {} with area: {}",
            self.width,
            self.height,
            self.area()
        );
    }

    fn area(&self) -> u32 {
        // creating a new struct
        self.width * self.height // alternative is using return obj.width * obj.height; for returning
    }
}

fn take(v: Vec<i32>) {
    println!("This is v: {}", v[10] + v[100]);
}

fn cop(a: i32, b: i32) {
    println!("This is the value: {}", a + b);
}

// example of conditionals
fn conditionals() {
    let mut n = 4;
    if n < 5 {
        println!("true");
    } else {
        println!("false");
    }

    // using if-else add bindings to n(just like a ternary operator). return types need to be the same BTW
    n = if true { 48 } else { 30 };
    println!("{}", n);
}

// using the loop block example for looping

fn loop_example() {
    let mut c = 0;
    loop {
        println!("Nice");
        c += 1;
        if c >= 10 {
            break;
        }
    }
}

fn nested_loop_example_labels() {
    let mut c = 0;
    'a: loop {
        println!("a");
        'b: loop {
            println!("b");
            'c: loop {
                println!("c");
                c += 1;
                if c >= 10 {
                    break 'a; // example of breaking a higher loop
                }
            }
        }
    }
}

fn while_example() {
    let mut n = 10;

    while n != 0 {
        println!("{}", n);
        n -= 1;
    }
}

fn for_loop_example() {
    let a = vec![10, 20, 30, 40, 50, 60, 70, 80, 90];

    for i in a {
        println!("{}", i);
    }

    // using dots (.. exclusive, means except the last one and ..= is inclusive)
    for i in 1..10 {
        println!("{}", i);
    }

    for i in 1..=10 {
        println!("{}", i);
    }
}

// match is like switch but with more power such as pattern matching
fn match_example() {
    let x = 5;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("else"),
    }

    let age = 15;
    // example of multi case match conditions
    match age {
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        13..=19 => println!("this is a teen"),
        _ => println!("ain't a special"),
    }

    // pair pattern matching
    let value = (0, -4);
    match value {
        (0, y) => println!("y: {}", y), // if the left first value is 0
        (x, 0) => println!("x: {}", x), // if the right first value is 0
        _ => println!("this is nothing"),
    }

    // example of guards matching
    let another_value = (5, -5);
    match another_value {
        (x, y) if x == y => println!("equal"),
        (x, y) if x + y == 0 => println!("equal zero"),
        (x, _) if x % 2 == 0 => println!("x is even"), // _ means I don't care about the other value
        _ => println!("diff"),
    }
}

fn main() {
    /*
    using _ for ignoring the unused variables warning
    */
    // Everything is immutable by default
    let _x = 5;

    // Example of a mutable variable
    let mut _y = 10;

    // Example of type description addition (i8, u8, i16, u16, i32, u32, i64, u64, f32, f64, isize, usize)
    let _u: u32 = 34;

    // tuple declaration with certain types
    let t: (i32, f64, char) = (41, 6.14, 'j');
    // destructuring the tuple
    let (_, _, _x) = t; // _ means, I don't want that value (tuple can also be called by index such as t.0 or etc)

    // example of nested tuple
    let f_tuple = (1, 'a', false);
    let s_tuple = (4, f_tuple);
    println!("{}", s_tuple.0);

    // printing a sole tuple
    println!("{:?}", s_tuple.1);
    // added idiomatic pretty print for tuple
    println!("{:#?}", s_tuple.1);
    println!("{}", (s_tuple.1).1); // printing a nested tuple

    // example of an array ([i32;5] means 5 elements of size i32)
    let arr_xs: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?} {}", arr_xs, arr_xs.len());

    // taking a slice of the array
    let ys = &arr_xs[1..4]; // & stands for referencing
    println!("{:?}", ys);

    // string based on compound literal
    let string = "String!";
    println!("This is a compound string: {}", string);
    // string with an actual string type
    let actual_string = String::from("String!");
    println!("This is an actual string type: {}", actual_string);

    // converting a compound string literal to string type
    let convert_string = &string.to_string();
    println!("Converted the string: {}", convert_string);
    println!("Slice of a string: {}", &convert_string[0..4]);

    // concatenation of a string
    let string_one = String::from("This is Rust!");
    let string_sec = String::from("welcome to the party.");
    let concat_string = string_one + &string_sec;
    println!("Concatenated string: {}", concat_string);

    // example of moving ownership from one function to the other
    let mut v: Vec<i32> = Vec::new(); // creating a new vector mutable variable
    for i in 1..1000 {
        v.push(i);
    }
    take(v); // transferring ownership for the vector v from main() to take()

    // example of copying the ownership
    let a = 1;
    let b = 4;

    cop(a, b); // copying because after passing it to cop() function we are still using those values in main() function
    println!("Still have the ownership for a and b: {} {}", a, b);

    // instantiating a new struct based on the type object
    let o = Object {
        width: 10,
        height: 14,
    };
    o.show();
    let new_object = Object::new(4, 5);
    new_object.show();
    println!("{:?}", new_object);
    conditionals();
    loop_example();
    nested_loop_example_labels();
    while_example();
    for_loop_example();
    match_example();
}
