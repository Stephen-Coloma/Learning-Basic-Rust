// Basic Coding in Rust - Hannah

// Constants
const TOTAL_SUBJECTS: f32 = 8.0;

pub fn compute_grade() {
    // Student details
    let student_name = "Hannah";
    let student_year = "BSCS - 2";
    let student_course = "Computer Science";

    // Grades for each course
    let grade_cfe_104: f32 = 93.0;
    let grade_cs_221_lec: f32 = 89.0;
    let grade_cs_221l_lab: f32 = 89.0;
    let grade_cs_222_lec: f32 = 89.0;
    let grade_cs_222l_lab: f32 = 89.0;
    let grade_cs_223: f32 = 89.0;
    let grade_gcworld: f32 = 99.0;
    let grade_gethics: f32 = 92.0;

    // Compute total grade
    let total_grade = grade_cfe_104 + grade_cs_221_lec + grade_cs_221l_lab
        + grade_cs_222_lec + grade_cs_222l_lab + grade_cs_223 + grade_gcworld
        + grade_gethics;

    // Compute average grade
    let average_grade = total_grade / TOTAL_SUBJECTS;

    // Display student info and grades
    println!("Student Name: {}", student_name);
    println!("Year Grade: {}", student_year);
    println!("Course: {}", student_course);
    println!("\nSubject Grades:");
    println!("CFE 104: {:.2}", grade_cfe_104);
    println!("CS 221 (LEC): {:.2}", grade_cs_221_lec);
    println!("CS 221L (LAB): {:.2}", grade_cs_221l_lab);
    println!("CS 222 (LEC): {:.2}", grade_cs_222_lec);
    println!("CS 222L (LAB): {:.2}", grade_cs_222l_lab);
    println!("CS 223: {:.2}", grade_cs_223);
    println!("GCWORLD: {:.2}", grade_gcworld);
    println!("GETHICS: {:.2}", grade_gethics);

    // Display calculated grade average
    println!("\nCalculated Grade Average: {:.2}", average_grade);
}
