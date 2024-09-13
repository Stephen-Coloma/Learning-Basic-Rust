// Struct for a student
struct Student {
    name: String,
    age: u32,
    tuition: f64,
    units: u32,
}


impl Student {
    // Constructor for Student
    fn new(name: &str, age: u32, tuition: f64, units: u32) -> Student {
        Student {
            name: String::from(name),
            age,
            tuition,
            units,
        }
    }
    // Encapsulated methods
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_tuition(&self) -> f64 {
        self.tuition * self.units as f64
    }
    fn get_remaining_balance(&self, payment:f64) -> f64 {
        self.get_tuition()-payment
    }
    fn get_units(&self) -> u32 {
        self.units
    }
}

// Struct for a scholar
struct Scholar {
    student: Student,  // Scholar contains a Student (composition)
    discount: f64,
    maintaining_grade: f64,
}

impl Scholar {
    // Constructor for Scholar
    fn new(student: Student, discount: f64, maintaining_grade: f64) -> Scholar {
        Scholar {
            student,
            discount,
            maintaining_grade,
        }
    }
    //Encapsulated methods
    fn get_name(&self) -> &str {
        &self.student.get_name()
    }
    fn get_tuition(&self)->f64{ self.student.get_tuition() }
    fn get_discounted_tuition(&self)->f64 {
        self.student.get_tuition() - (self.student.get_tuition() * self.discount)
    }
    fn get_remaining_balance(&self, payment:f64) -> f64 {
        self.get_discounted_tuition()-payment
    }

    fn get_discount(&self) -> f64 {
        self.discount
    }
    fn get_maintaining_grade(&self) -> f64{
        self.maintaining_grade
    }
    fn check_maintaining_grade(&self, current_grade: f64) -> bool {
        current_grade >= self.maintaining_grade
    }

}

// Struct for an irregular student that inherits from Student
struct IrregularStudent {
    student: Student,
    additional_units: u32,
}

impl IrregularStudent {
    // Constructor for IrregularStudent
    fn new(student: Student, additional_units: u32) -> IrregularStudent {
        IrregularStudent {
            student,
            additional_units,
        }
    }
    fn get_name(&self) -> &str {
        &self.student.get_name()
    }
    fn get_additional_units(&self) -> u32 {
        self.additional_units
    }
    fn get_total_tuition(&self) -> f64 {
        let total_units = self.student.get_units() + self.get_additional_units();
        self.student.get_tuition() * total_units as f64
    }
}
pub fn sub_method() {

    println!("--------------------------Regular----------------------------------");
    let student = Student::new("Alice", 20, 100000.0, 12);
    println!("{}'s tuition: {}", student.get_name(), student.get_tuition());
    println!("Remaining balance after payment of 3000 pesos: ${}", student.get_remaining_balance(3000.0));



    println!("--------------------------Scholar----------------------------------");
    let student_scholar = Student::new("Stephen", 21, 100000.0, 12);
    let scholar = Scholar::new(student_scholar, 0.2, 85.0);
    println!("{}'s discounted tuition is: {}", scholar.get_name(), scholar.get_discounted_tuition());
    println!("Remaining balance after payment of 3000 pesos: ${}", scholar.get_remaining_balance(3000.0));
    println!("{}'s maintaining grade: {}", scholar.get_name(), scholar.get_maintaining_grade());
    println!("Does {} pass with a grade of 84? {}", scholar.get_name(), scholar.check_maintaining_grade(84.0));



    println!("-----------------------Irregular----------------------------------");
    let student_irreg = Student::new("Sanchie", 20, 100000.0, 12);
    let irreg_student = IrregularStudent::new(student_irreg, 3);
    println!("{}'s total tuition with additional units: ${}",irreg_student.get_name(), irreg_student.get_total_tuition());
}



