fn main() {
    // instantiate logbook
    let mut log_book = LogBook::new();

    // creation of students
    let undergraduate1 = Box::new(Undergraduate {
        first_name: String::from("Stephen"),
        last_name: String::from("Horseshoes"),
        student_id: 2234532,
        major: String::from("Computer Science"),
        year: String::from("Junior"),
        credit_hours: 19,
        average_grade: 93.5,
    });

    let undergraduate2 = Box::new(Undergraduate {
        first_name: String::from("Mario"),
        last_name: String::from("Nonato"),
        student_id: 2230683,
        major: String::from("Computer Science"),
        year: String::from("Junior"),
        credit_hours: 19,
        average_grade: 90,
    });

    let undergraduate3 = Box::new(Undergraduate {
        first_name: String::from("Brianna"),
        last_name: String::from("Ray"),
        student_id: 2243439,
        major: String::from("Computer Science"),
        year: String::from("Sophomore"),
        credit_hours: 22,
        average_grade: 78,
    });

    let graduate1 = Box::new(Graduate {
        first_name: String::from("Henley"),
        last_name: String::from("Atkins"),
        student_id: 2234567,
        major: String::from("Biotechnology"),
        thesis_topic: String::from("Gene Editing"),
        average_grade: 82,
        research_hours: 10,
    });

    let graduate2 = Box::new(Graduate {
        first_name: String::from("Idris"),
        last_name: String::from("Nichols"),
        student_id: 2236597,
        major: String::from("Mechanical Engineering"),
        thesis_topic: String::from("Renewable Energy"),
        average_grade: 90,
        research_hours: 15,
    });

    // add students to the logbook
    log_book.add_student(undergraduate1);
    log_book.add_student(undergraduate2);
    log_book.add_student(undergraduate3);
    log_book.add_student(graduate1);
    log_book.add_student(graduate2);

    // display the information of all the students
    println!("Student Log Book Information");
    println!("============================");
    log_book.display_all_students();
}

// traits of students (interfaces in java)
trait Student {
    fn get_first_name(&self) -> &str;
    fn get_last_name(&self) -> &str;
    fn get_student_id(&self) -> u32;
    fn get_average_grade(&self) -> f32;
    fn get_performance_status(&self) -> String;
    fn compute_tuition(&self) -> f32;
    fn display_info(&self);
}

// undergraduate structure (object in java)
struct Undergraduate {
    first_name: String,
    last_name: String,
    student_id: u32,
    major: String,
    year: String,
    credit_hours: u32,
    average_grade: f32,
}

// implementing traits to undergraduate structure (interface implementation in java)
impl Student for Undergraduate {
    fn get_first_name(&self) -> &str {
        &self.first_name
    }

    fn get_last_name(&self) -> &str {
        &self.last_name
    }

    fn get_student_id(&self) -> u32 {
        self.student_id
    }

    fn get_average_grade(&self) -> f32 {
        self.average_grade
    }

    fn get_performance_status(&self) -> f64 {
        if self.average_grade > 85.0 {
            "Excellent Performance".to_string()
        } else if self.average_grade > 75.0 {
            "Good Performance".to_string()
        } else {
            "Needs Improvement".to_string()
        }
    }

    fn compute_tuition(&self) -> f32 {
        let rate_per_credit = 300.0;
        let tuition = rate_per_credit * self.credit_hours as f32;
        tuition
    }

    fn display_info(&self) {
        println!(
            "Graduate Student: {}, {} (ID: {}) - Major: {} - Year: {} - Credit Hours: {}\
            - Average Grade: {:.2}",
            self.last_name, self.first_name, self.student_id,
            self.major, self.year, self.credit_hours, self.average_grade
        );
    }
}

// graduate structure (object in java)
struct Graduate {
    first_name: String,
    last_name: String,
    student_id: u32,
    major: String,
    thesis_topic: String,
    average_grade: f32,
    research_hours: u32,
}


// implementing traits to graduate structure (interface implementation in java)
impl Student for Graduate {
    fn get_first_name(&self) -> &str {
        &self.first_name
    }

    fn get_last_name(&self) -> &str {
        &self.last_name
    }

    fn get_student_id(&self) -> u32 {
        self.student_id
    }

    fn get_average_grade(&self) -> f32 {
        self.average_grade
    }

    fn get_performance_status(&self) -> f64 {
        if self.average_grade > 85.0 {
            "Excellent Research".to_string()
        } else if self.average_grade > 75.0 {
            "Good Research".to_string()
        } else {
            "Needs More Work".to_string()
        }
    }

    fn compute_tuition(&self) -> f32 {
        let base_tuition = 4000.0;
        let research_fee = 150.0 * self.research_hours as f32;
        let tuition = base_tuition + research_fee;
        tuition
    }

    fn display_info(&self) {
        println!(
            "Graduate Student: {}, {} (ID: {}) - Major: {} - Thesis Topic: {} - Avg. Grade: {:.2}\
            - Research Hours: {}",
            self.last_name, self.first_name, self.student_id, self.major,
            self.thesis_topic, self.average_grade, self.research_hours
        );
    }
}

struct LogBook {
    students: HashMap<u32, Box<dyn Student>>,
}

impl LogBook {
    fn new() -> Self {
        LogBook {
            students: HashMap::new(),
        }
    }

    fn add_student(&mut self, student: Box<dyn Student>) {
        self.students.insert(student.get_student_id(), student);
    }

    fn get_student(&self, student_id: u32) -> Option<&Box<dyn Student>> {
        self.students.get(&student_id)
    }

    fn get_student_mut(&mut self, student_id: u32) -> Option<&mut Box<dyn Student>> {
        self.students.get_mut(&student_id)
    }

    fn display_all_students(&self) {
        for student in self.students.values() {
            student.display_info();
            println!("Tuition Fee: P{:.2}", student.compute_tuition());
            println!("---")
        }
    }
}