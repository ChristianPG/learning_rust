fn main() {
    play_with_vectors();
}

fn play_with_vectors() {
    // Create a new empty vector of type i32
    let _empty_vector: Vec<i32> = Vec::new();

    // Create a new filled Vector of type i32
    let _filled_vector= vec![1, 2, 3];

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
