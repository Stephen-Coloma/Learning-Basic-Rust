pub fn sub_method(){
    println!("Concept of Borrowing");
    println!("=================================");
    println!("Welcome to Borrowing");

    let string1 = String::from("Hello World");
    let string2 = &string1; //using the & to borrow the value of string1

    println!("Content of string1: {}", string1);
    println!("Content of string2: {}", string2);

    //====================================================

    // using reference to borrow a value from a variable
    // let mut full_name = String::from("GUZMAN, Sanchie Earl M.");
    // println!("Full Name: {}", full_name);
    //
    // let modified_name = &mut full_name;
    // modified_name.clear();
    // modified_name.push_str("MANALO, Sanchie Earl");
    //
    // println!("Value of full_name after modification: {}",full_name);


    println!("=================================");
    println!();
}

#[derive(Debug)]
struct Student {
    name:String,
    id:i64
}

