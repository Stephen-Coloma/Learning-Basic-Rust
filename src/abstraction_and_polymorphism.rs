use indexmap::IndexMap;

// traits defined for students (interface in java)
trait Student {
    fn get_first_name(&self) -> &str;
    fn get_last_name(&self) -> &str;
    fn get_student_id(&self) -> u32;
    fn get_average_grade(&self) -> f32;
    fn get_performance_status(&self) -> String;
    fn compute_tuition(&self) -> f32;
    fn display_info(&self);
}

// structure to represent an undergraduate student (object in java)
struct Undergraduate {
    first_name: String,
    last_name: String,
    student_id: u32,
    major: String,
    year: String,
    credit_hours: u32,
    average_grade: f32,
}

// implementation of the student trait for undergraduate students (interface implementation in java)
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

    fn get_performance_status(&self) -> String {
        if self.average_grade > 85.0 {
            "Excellent Performance".to_string()
        } else if self.average_grade > 75.0 {
            "Good Performance".to_string()
        } else {
            "Needs Improvement".to_string()
        }
    }

    fn compute_tuition(&self) -> f32 {
        let rate_per_credit = 2000.0;
        let tuition = rate_per_credit * self.credit_hours as f32;
        tuition
    }

    fn display_info(&self) {
        println!(
            "Graduate Student: {}, {} \n\
            ID Number: {} \n\
            Major: {} \n\
            Year: {} \n\
            Credit Hours: {} \n\
            Average Grade: {:.2}",
            self.last_name, self.first_name, self.student_id,
            self.major, self.year, self.credit_hours, self.average_grade
        );
    }
}

// structure to represent a graduate student (object in java)
struct Graduate {
    first_name: String,
    last_name: String,
    student_id: u32,
    major: String,
    thesis_topic: String,
    average_grade: f32,
    research_hours: u32,
}


// implementation of the student trait for graduate students (interface implementation in java)
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

    fn get_performance_status(&self) -> String {
        if self.average_grade > 85.0 {
            "Excellent Research".to_string()
        } else if self.average_grade > 75.0 {
            "Good Research".to_string()
        } else {
            "Needs More Work".to_string()
        }
    }

    fn compute_tuition(&self) -> f32 {
        let base_tuition = 12450.0;
        let research_fee = 1500.0 * self.research_hours as f32;
        let tuition = base_tuition + research_fee;
        tuition
    }

    fn display_info(&self) {
        println!(
            "Graduate Student: {}, {} \n\
            ID Number: {} \n\
            Major: {} \n\
            Thesis Topic: {} \n\
            Avg. Grade: {:.2} \n\
            Research Hours: {}",
            self.last_name, self.first_name, self.student_id, self.major,
            self.thesis_topic, self.average_grade, self.research_hours
        );
    }
}

// structure to hold the information of various students
struct LogBook {
    students: IndexMap<u32, Box<dyn Student>>,
}

impl LogBook {
    fn new() -> Self {
        LogBook {
            students: IndexMap::new(), // Initialize an empty IndexMap
        }
    }

    // add student to the IndexMap
    fn add_student(&mut self, student: Box<dyn Student>) {
        self.students.insert(student.get_student_id(), student);
    }

    // retrieve a student based on their ID number
    fn get_student(&self, student_id: u32) -> Option<&Box<dyn Student>> {
        self.students.get(&student_id)
    }

    // retrieve a mutable reference to a student
    fn get_student_mut(&mut self, student_id: u32) -> Option<&mut Box<dyn Student>> {
        self.students.get_mut(&student_id)
    }

    // display all students in the log book
    fn display_all_students(&self) {
        for student in self.students.values() {
            student.display_info();
            println!("Performance Status: {}", student.get_performance_status());
            println!("Tuition Fee: P{:.2}", student.compute_tuition());
            println!("-------------------------------------------------------------------")
        }
    }
}

pub fn sub_method() {
    // instantiate logbook
    let mut log_book = LogBook::new();

    // creation of students
    let undergraduate1 = Box::new(Undergraduate {
        first_name: String::from("Stephen"),
        last_name: String::from("Coloma"),
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
        average_grade: 90f32,
    });

    let undergraduate3 = Box::new(Undergraduate {
        first_name: String::from("Brianna"),
        last_name: String::from("Ray"),
        student_id: 2243439,
        major: String::from("Computer Science"),
        year: String::from("Sophomore"),
        credit_hours: 22,
        average_grade: 78f32,
    });

    let graduate1 = Box::new(Graduate {
        first_name: String::from("Henley"),
        last_name: String::from("Atkins"),
        student_id: 2234567,
        major: String::from("Biotechnology"),
        thesis_topic: String::from("Gene Editing"),
        average_grade: 82f32,
        research_hours: 10,
    });

    let graduate2 = Box::new(Graduate {
        first_name: String::from("Jude"),
        last_name: String::from("Novak"),
        student_id: 2236597,
        major: String::from("Mechanical Engineering"),
        thesis_topic: String::from("Renewable Energy"),
        average_grade: 90f32,
        research_hours: 15,
    });

    // add students to the logbook
    log_book.add_student(undergraduate1);
    log_book.add_student(undergraduate2);
    log_book.add_student(undergraduate3);
    log_book.add_student(graduate1);
    log_book.add_student(graduate2);

    // display the information of all the students
    println!("                 Student Log Book Information");
    println!("===================================================================");
    log_book.display_all_students();
}