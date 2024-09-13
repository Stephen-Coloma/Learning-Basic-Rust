pub fn sub_method(){
    println!("\n\n CONDITIONAL STATEMENTS");
    println!("---------------------------------------------------------");
    conditionals();
    println!("---------------------------------------------------------\n\n");

    println!("TUPLES");
    println!("---------------------------------------------------------");
    tuple_compound_data_type();
    println!("---------------------------------------------------------\n\n");

    println!("ARRAYS");
    println!("---------------------------------------------------------");
    arrays_compound_data_type();
    println!("---------------------------------------------------------\n\n");

    println!("VECTORS");
    println!("---------------------------------------------------------");
    vectors();
    println!("---------------------------------------------------------\n\n");
}


fn conditionals(){
    /*Scenario: Compute for Latin Honors for a certain GWA*/
    const CUM_LAUDE: f64 = 87.0;
    const MAGNA_CUM_LAUDE: f64 = 91.0;
    const SUMMA_CUM_LAUDE: f64 = 94.0;

    let gwa: f64 = 88.9234;

    // If-else statements
    if gwa >= SUMMA_CUM_LAUDE {
        println!("With GWA of: {}, your Latin honor is SUMMA CUM LAUDE", gwa);
    } else if gwa >= MAGNA_CUM_LAUDE {
        println!("With GWA of: {}, your Latin honor is MAGNA CUM LAUDE", gwa);
    } else if gwa >= CUM_LAUDE {
        println!("With GWA of: {}, your Latin honor is CUM LAUDE", gwa);
    } else {
        println!("With GWA of: {}, you do not qualify for Latin honors", gwa);
    }

    //Match conditional statement -- similar to switch case in java
    let year_level: i8 = 3;

    match year_level {
        1 => println!("You are a Freshman."),
        2 => println!("You are a Sophomore."),
        3 => println!("You are a Junior."),
        4 => println!("You are a Senior."),
        _ => println!("Year level is not valid."),
    }
}

fn tuple_compound_data_type(){
    //a tuple is a compound data type that can store elements having different types
    let student_tuple: (&str, i8, f64) = ("Stephen Coloma", 3, 89.9);

    // Tuples are not iterable, meaning we can't use looping statements

    //ways to access the element of the tuple
    println!("Name: {}",student_tuple.0);
    println!("Year Level: {}",student_tuple.1);
    println!("Current GWA: {}",student_tuple.2);
}

fn arrays_compound_data_type(){
    // arrays in rust are fixed length and stores the same types
    /*Scenario: Compute average grade stored in arrays*/
    let grades: [f64; 5] = [85.5, 90.0, 78.5, 88.0, 92.3];
    let total: f64 = grades.iter().sum();
    let average = total / grades.len() as f64;

    println!("Average Grade: {}", average);

    //bracket symbol same with java
    let first_grade: f64 = grades[0];
    println!("First Grade: {}", first_grade);
}

fn vectors(){
    let mut student_vector: Vec<&str> = Vec::new();

    // core methods of vectors
    student_vector.push("Stephen Coloma");
    student_vector.push("Leonhard Leung");
    student_vector.push("Jerwin Kyle Ramos");

    println!("{:?}\n", student_vector);

    student_vector.pop(); //removes last element
    println!("{:?}\n", student_vector);

    student_vector.remove(0); // removes index 0
    println!("{:?}\n", student_vector);

    student_vector.clear(); //removes all elements
    println!("{:?}\n", student_vector);
}