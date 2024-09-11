// A "superclass" for students
struct Student {
    name: String,
    age: u32,
    tuition: f64,
}

impl Student {
    fn new(name: &str, age: u32, tuition: f64) -> Student {
        Student {
            name: name.to_string(),
            age,
            tuition,
        }
    }

    // Encapsulated function that checks remaining balance
    fn is_remaining_balance(&self) -> f64 {
        self.tuition
    }

    // General function that checks tuition
    fn check_tuition(&self) {
        println!(
            "{}'s current remaining tuition balance: ${}",
            self.name, self.is_remaining_balance()
        );
    }
}

// Scholar that inherits student
struct Scholar {
    student: Student,
    discount: f64,
    maintaining_grade: f64,
}

impl Scholar {
    fn new(name: &str, age: u32, tuition: f64, discount: f64, maintaining_grade: f64) -> Scholar {
        Scholar {
            student: Student::new(name, age, tuition),
            discount,
            maintaining_grade,
        }
    }

    // Encapsulated function that checks discount
    fn check_discount(&self) -> f64 {
        self.discount
    }

    // Encapsulated function that checks if the scholar maintains their grade
    fn check_maintaining_grade(&self) -> bool {
        self.maintaining_grade >= 85.0
    }

    // Encapsulated function that checks if the scholar has privileges
    fn is_privileged(&self) -> bool {
        self.check_maintaining_grade() && self.discount > 0.0
    }

    // Scholar's specific function that calculates discounted tuition
    fn check_scholar_tuition(&self) {
        let discounted_tuition = self.student.tuition - (self.student.tuition * self.discount);
        println!(
            "{}'s tuition after discount: ${}, Privileged: {}",
            self.student.name,
            discounted_tuition,
            self.is_privileged()
        );
    }
}

// Regular student that inherits student
struct RegularStudent {
    student: Student,
}

impl RegularStudent {
    fn new(name: &str, age: u32, tuition: f64) -> RegularStudent {
        RegularStudent {
            student: Student::new(name, age, tuition),
        }
    }

    // Inherited function that checks tuition
    fn check_tuition(&self) {
        self.student.check_tuition();
    }
}

// Irregular student that inherits student
struct IrregularStudent {
    student: Student,
    additional_units: u32,
}

impl IrregularStudent {
    fn new(name: &str, age: u32, tuition: f64, additional_units: u32) -> IrregularStudent {
        IrregularStudent {
            student: Student::new(name, age, tuition),
            additional_units,
        }
    }

    // Additional cost for irregular students
    fn calculate_additional_tuition(&self) -> f64 {
        self.student.tuition + (self.additional_units as f64 * 100.0)
    }

    // Inherited function to check tuition
    fn check_tuition(&self) {
        let total_tuition = self.calculate_additional_tuition();
        println!(
            "{}'s total tuition with additional units: ${}",
            self.student.name, total_tuition
        );
    }
}

// Main function to test the inheritance
pub fn sub_method() {
    let regular = RegularStudent::new("Alice", 20, 5000.0);
    regular.check_tuition();

    let scholar = Scholar::new("Bob", 22, 5000.0, 0.3, 88.0);
    scholar.check_scholar_tuition();

    let irregular = IrregularStudent::new("Charlie", 21, 5000.0, 5);
    irregular.check_tuition();
}
