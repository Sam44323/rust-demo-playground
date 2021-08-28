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
    println!("{:?}", arr_xs);
}
