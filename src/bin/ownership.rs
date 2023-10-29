// String clone
// fn main() {
//     let mut s: String = String::from("hello");

//     let mut s2 = "hello";
//     println!("{}", s2);
//     s2 = "alsjhdfjklhasdjklfh";

//     s.push_str(", world!"); // push_str() appends a literal to a String

//     println!("{}", s); // This will print `hello, world!`
//     println!("{}", s2);

//     // If clone is not used, s3 would be invalidated
//     let s3 = String::from("hello");
//     let s4 = s3.clone();

//     println!("s3 = {}, s4 = {}", s3, s4);
// }

// Ownership flow
// fn main() {
//     let s = String::from("hello");  // s comes into scope

//     takes_ownership(s);             // s's value moves into the function...
//     // ... and so is no longer valid here
//     // println!("{}", s);

//     let x = 5;                      // x comes into scope

//     makes_copy(x);                  // x would move into the function,
//     // but i32 is Copy, so it's okay to still
//     // use x afterward

//     println!("{}", x);

// } // Here, x goes out of scope, then s. But because s's value was moved, nothing
//   // special happens.

// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
//   // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.

// Borrowing
// fn main() {
//   let mut s1 = String::from("hello");

//   let len = calculate_length(&mut s1);

//   println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &mut String) -> usize { // s is a reference to a String
//   s.push_str(", world");
//   s.len()
// } // Here, s goes out of scope. But because it does not have ownership of what
// it refers to, it is not dropped.

fn main() {
    let s1 = String::from("hello world");

    let first_word = get_first_word(&s1);
    println!("The first word in '{}' is '{}'.", s1, first_word);
}

fn get_first_word(whole_string: &str) -> &str {
    let v: Vec<&str> = whole_string.split(' ').collect();

    v[0]
}
