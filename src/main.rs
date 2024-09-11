mod borrowing {
    pub mod borrowing;
}



mod shadowing {
    pub mod shadowing;
}

mod ownership {
    pub mod  ownership;
}

fn main() {
    println!("Hello, world!");
    println!();

    println!("Concept of Ownership");
    println!("=================================");
    ownership::ownership::main();
    println!("=================================");
    println!();

    println!("Concept of Borrowing");
    println!("=================================");
    borrowing::borrowing::main();
    println!("=================================");
    println!();

    println!("Concept of Shadowing");
    println!("=================================");
    shadowing::shadowing::main();
    println!("=================================");
    println!();

}
