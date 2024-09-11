mod conditionals_compound_types;
mod basics;
mod functions_control_flow;

mod borrowing {
    pub mod borrowing;
}

mod shadowing {
    pub mod shadowing;
}

mod ownership {
    pub mod  ownership;
}

mod inheritance{
    pub mod inheritance_and_encapsulation;
}

fn main() {
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

    inheritance::inheritance_and_encapsulation::main();
}