// defining the type for Object struct
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
}
