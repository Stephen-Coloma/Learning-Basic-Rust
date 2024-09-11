pub fn sub_method(){
    println!("Concept of Ownership");
    println!("=================================");
    println!("Welcome to Ownership");


    // let string1 = String::from("Hello World");
    // let string2 = string1;
    //
    // println!("Content of string1: {}", string1);

    //====================================================

    // ownership by functions
    // let student1 = String::from("Sanchie");
    // add_student(student1);
    // println!("student1: {}", student1);

    //====================================================

    //ownership in struct / objects
    // let cs_student = Student{
    //     name: String::from("Sanchie Earl Guzman"),
    //     id: 2232886
    // };
    //
    // let cs_student2 = cs_student;
    //
    // println!("The value of cs_student is: {:?}", cs_student);


    // println!("=================================");
    // println!();
}

fn add_student(student: String){
    let mut students:Vec<String> = vec![String::from("Stephen"), String::from("Hannah"), String::from("Marius")];
    students.push(student);
    println!("{:?}", students);
}


#[derive(Debug)]
struct Student {
    name:String,
    id:i64
}
