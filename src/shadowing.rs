
pub fn sub_method(){
    println!("Concept of Shadowing");
    println!("=================================");
    println!("Welcome to Shadowing");

    let student = String::from("Sanchie");
    println!("1. {}", student);

    let student = student + " Earl";
    println!("2. {}", student);

    let student = String::from("Jerwin");
    println!("3. {}", student);

    //changing the data type
    let student = false;
    println!("4. {}",student);

    println!("=================================");
    println!();
}