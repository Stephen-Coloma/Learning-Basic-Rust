pub fn sub_method(){
    println!("Concept of Shadowing");
    println!("=================================");
    println!("Welcome to Shadowing");

    let student = String::from("Sanchie");
    println!("{}", student);

    let student = student + " Earl";
    println!("{}", student);

    let student = student + " Guzman";
    println!("{}", student);

    //changing the data type
    let student = false;
    println!("{}",student);

    // //reusing variables in different scopes
    // let i = 3;
    //
    // {
    //     let i = i * 5;
    //     println!("The value of inner i is {}", i);
    // }
    //
    // println!("The value of outer i is {}", i);
    println!("=================================");
    println!();
}