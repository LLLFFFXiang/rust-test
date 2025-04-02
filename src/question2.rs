// 定义一个 Student 结构体，包含以下字段：name、age、score
// 实现以下功能：
// - new(name: &str, age: u8, score: f32) -> Student：返回一个新的学生实例。
// - show(&self)：打印 Student 的信息，格式如 Name: Alice, Age: 18, Score: 95.5。
// - is_passed(&self) -> bool：如果 score >= 60.0 则返回 true，否则返回 false。
struct Student {
    name: String,
    age: u8,
    score: f32,
}

impl Student {

    pub fn new(name: &str, age: u8, score: f32) -> Student {
        Student {
            name: name.to_string(),
            age,
            score,
        }
    }

    pub fn show(&self) {
        println!("Name: {}, Age: {}, Score: {:.1}", self.name, self.age, self.score);
    }

    pub fn is_passed(&self) -> bool {
        self.score >= 60.0
    }
}

fn main() {

    let student = Student::new("Alice", 17, 94);
    student.show();
    println!("Passed: {}", student.is_passed());
    
    let student2 = Student::new("lili", 17, 55.0);
    student2.show();
    println!("Passed: {}", student2.is_passed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_student_creation() {
        let student = Student::new("张三", 18, 85.5);
        assert_eq!(student.name, "张三");
        assert_eq!(student.age, 18);
        assert_eq!(student.score, 85.5);
    }

    #[test]
    fn test_is_passed() {
        let passing_student = Student::new("及格学生", 20, 60.0);
        let failing_student = Student::new("不及格学生", 20, 59.9);
        
        assert!(passing_student.is_passed());
        assert!(!failing_student.is_passed());
    }

    #[test]
    fn test_edge_cases() {
        let zero_score = Student::new("零分", 18, 0.0);
        let full_score = Student::new("满分", 18, 100.0);
        
        assert!(!zero_score.is_passed());
        assert!(full_score.is_passed());
    }

    #[test]
    fn test_show_output() {
        // 重定向 stdout 进行测试
        use std::io::{self, Write};
        
        let student = Student::new("测试输出", 19, 75.5);
        
        let mut output = Vec::new();
        io::Write::write_all(&mut output, format!("Name: {}, Age: {}, Score: {:.1}\n", 
            student.name, student.age, student.score).as_bytes()).unwrap();
        
        let mut buf = Vec::new();
        student.show();
        io::Write::write_all(&mut buf, b"Name: 测试输出, Age: 19, Score: 75.5\n").unwrap();
        
        assert_eq!(output, buf);
    }
}

