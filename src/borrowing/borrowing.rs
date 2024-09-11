pub fn main(){
    println!("Welcome to Borrowing");

    let string1 = String::from("Hello World");
    let string2 = &string1; //using the & to borrow the value of string1

    println!("Content of string1: {}", string1);
    println!("Content of string2: {}", string2);

    //ownership in struct / objects
    // let cs_student =Student {
    //     name: String::from("Sanchie Earl Guzman"),
    //     id: 2232886
    // };
    //
    // let cs_student2 = &cs_student;
    //
    // println!("The value of cs_student is: {:?}", cs_student);
    // println!("The value of cs_student2 is: {:?}", cs_student2);


    // using reference to borrow a value from a variable
    // let full_name = String::from("GUZMAN, Sanchie Earl M.");
    // let last_name = get_lastname(&full_name);
    // println!("Full Name: {}", full_name);
    // println!("Last Name: {}", last_name);


}

#[derive(Debug)]
struct Student {
    name:String,
    id:i64
}

fn get_lastname(full_name:&String) -> &str{
    let piece : Vec<&str> = full_name.split(',').collect();

    return piece[0].trim();
}

