// Define the Student struct
struct Student {
    id: u32,
    name: String,
    grade: f64,
}
impl Student {
    // Create a new student
    fn new(id: u32, name: String, grade: f64) -> Student {
        Student { id, name, grade }
    }

    // Update the student's grade
    fn update_grade(&mut self, new_grade: f64) {
        self.grade = new_grade;
    }
}
// Define the StudentManager struct
struct StudentManager {
    students: Vec<Student>,
}

impl StudentManager {
    // Create a new student manager
    fn new() -> StudentManager {
        StudentManager { students: Vec::new() }
    }

    // Add a new student
    fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }

    // Remove a student by ID
    fn remove_student(&mut self, id: u32) {
        self.students.retain(|student| student.id != id);
    }

    // Update a student's grade
    fn update_student_grade(&mut self, id: u32, new_grade: f64) {
        for student in self.students.iter_mut() {
            if student.id == id {
                student.update_grade(new_grade);
                break;
            }
        }
    }

    // Print all students
    fn print_students(&self) {
        for student in &self.students {
            println!("ID: {}, Name: {}, Grade: {}", student.id, student.name, student.grade);
        }
    }
}
fn main() {
    let mut manager = StudentManager::new();

    // Add some students
    manager.add_student(Student::new(1, "Amel chebil".to_string(), 100.0));
    manager.add_student(Student::new(2, "ali abc".to_string(), 90.0));
    manager.add_student(Student::new(3, "ila cba".to_string(), 99.0));

    // Print all students
    println!("All Students:");
    manager.print_students();

    // Update a student's grade
    manager.update_student_grade(1, 95.0);

    // Print all students again
    println!("Updated Students:");
    manager.print_students();

    // Remove a student
    manager.remove_student(2);

    // Print all students again
    println!("Students after removal:");
    manager.print_students();
}
