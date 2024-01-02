use std::collections::HashMap;

fn main() {
    play_with_vectors();
    play_with_strings();
    play_with_hashmaps();
}

fn play_with_vectors() {
    // Create a new empty vector of type i32
    let _empty_vector: Vec<i32> = Vec::new();

    // Create a new filled Vector of type i32
    let _filled_vector = vec![1, 2, 3];

    // Create a new empty and untyped vector
    let mut untyped_vector = Vec::new();
    // Now it will have the type i32
    untyped_vector.push(1);

    // Create a filled vector
    let accessed_vector = vec![1, 2, 3, 4, 5];
    // This method could panic if the index is out of scope
    let second: &i32 = &accessed_vector[1];
    println!("The second element is {second}");
    // This method will NOT panic because it can be controlled by the match
    let third: Option<&i32> = accessed_vector.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Iterate over a vector to print its content
    let iterable_vector = vec![100, 200, 300];
    for i in &iterable_vector {
        println!("Iterating vector: {i}")
    }

    // Iterate over mutable references in a mutable vector
    let mut mutable_vector = vec![100, 32, 57];
    for i in &mut mutable_vector {
        // Use the * dereference operator to get to the value in i before we can use the += operator
        *i += 50;
    }

    // Use an enum to store different type values inside the same vector
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn play_with_strings() {
    // Create an empty String
    let mut _s = String::new();

    // Create a String from a string literal
    let literal = "literal example";
    let _s = literal.to_string();
    // Using to_string directly on the literal
    let _s = "another literal".to_string();
    // Using from instead of to_string
    let mut s = String::from("yet another literal");

    // Append a string slice using push_str
    s.push_str(" with a slic");

    // Append one character using push
    s.push('e');

    // Combine two strings using the + operator(it uses the add method behind the scenes)
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _s3 = s1 + &s2; // note s1 has been moved here and can no longer be used

    // Combining more than two Strings using the format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let _s = format!("{s1}-{s2}-{s3}");

    // Get a slice of a String using bytes range
    let hello = "Здравствуйте";
    // Each letter takes 2 bytes to store. Others characters could take more or less
    let _s = &hello[0..4]; // s will now have "Зд" in it

    // Iterate over the chars in a String
    for c in hello.chars() {
        println!("{c}");
    }

    // Iterate over the bytes in a String
    for b in "Зд".bytes() {
        println!("{b}");
    }
}

fn play_with_hashmaps() {
    // Create an empty hash map
    let mut scores = HashMap::new();

    // Add two record to it
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Access one of them using a key
    let team_name = String::from("Blue");
    let _score = scores.get(&team_name).copied().unwrap_or(0);

    // iterate over the hash map
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Overwrite a value
    scores.insert(String::from("Blue"), 25);

    // Insert a new value only if the key doesn't exist
    scores.entry(String::from("Yellow")).or_insert(75);
    scores.entry(String::from("Red")).or_insert(50);

    // Update a value based on previous value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
