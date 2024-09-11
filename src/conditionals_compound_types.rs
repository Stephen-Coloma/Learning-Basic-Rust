pub fn sub_method(){
    conditionals();
    tuple_compound_data_type();
    arrays_compound_data_type();
    vectors();
}


fn conditionals(){
    /*Assuming that we already have the grade, we want to know what latin honors does it fall using decision flor*/
    const CUM_LAUDE: f64 = 87.0;
    const MAGNA_CUM_LAUDE: f64 = 91.0;
    const SUMMA_CUM_LAUDE: f64 = 94.0;

    let gwa: f64 = 88.9234;

    // If-else statements
    if gwa >= SUMMA_CUM_LAUDE {
        println!("With GWA of: {:.4}, your Latin honor is SUMMA CUM LAUDE", gwa);
    } else if gwa >= MAGNA_CUM_LAUDE {
        println!("With GWA of: {:.3}, your Latin honor is MAGNA CUM LAUDE", gwa);
    } else if gwa >= CUM_LAUDE {
        println!("With GWA of: {:.2}, your Latin honor is CUM LAUDE", gwa);
    } else {
        println!("With GWA of: {:.1}, you do not qualify for Latin honors", gwa);
    }

    //another way is using match -- similar to switch case in java
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

    //printing the content of the tuple
    println!("{:?}", student_tuple);

    // NOTE! tuples are not iterable

    println!("---------------------------------------\n");

    //ways to access the content of the tuple
    println!("Name: {}",student_tuple.0);
    println!("Year Level: {}",student_tuple.1);
    println!("Current GWA: {}",student_tuple.2);

    println!("---------------------------------------\n");
    //another cool way to destructure
    let (name, year_level, current_gwa) = student_tuple;
    println!("Name: {}", name);
    println!("Year Level: {}", year_level);
    println!("Current GWA: {}", current_gwa);
}

fn arrays_compound_data_type(){
    // arrays are iterable using the .iter() but loops will be discussed in a while
    let grades: [f64; 5] = [85.5, 90.0, 78.5, 88.0, 92.3]; // Array of 5 grades
    let total: f64 = grades.iter().sum();
    let average = total / grades.len() as f64;

    println!("Average Grade: {}", average);

    let first_grade: f64 = grades[0];
}

fn vectors(){
    let mut student_vector: Vec<&str> = Vec::new();

    // core methods of vectors
    student_vector.push("Stephen Coloma");
    student_vector.push("Leonhard Leung");
    student_vector.push("Marius Nonato");
    student_vector.push("Jerwin Kyle Ramos");

    println!("{:?}\n", student_vector);

    student_vector.pop(); //removes Jerwin Kyle Ramos
    println!("{:?}\n", student_vector);

    student_vector.remove(0); // removes Stephen Coloma
    println!("{:?}\n", student_vector);

    student_vector.clear(); //removes all elements
    println!("{:?}\n", student_vector);
}