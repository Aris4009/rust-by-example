fn main() {
    let css = Css {
        name: String::from("Tom"),
        username: String::from("Jerry"),
        university: String::from("PKU"),
        lang: String::from("US"),
    };
    println!("{}", comp_sci_student_greeting(&css));
}

struct Css {
    username: String,
    name: String,
    lang: String,
    university: String,
}
impl CompSciStudent for Css {
    fn git_username(&self) -> String {
        let s = String::from(self.username.as_str());
        s
    }
}

impl Programmer for Css {
    fn fav_language(&self) -> String {
        let s = String::from(self.lang.as_str());
        s
    }
}

impl Student for Css {
    fn university(&self) -> String {
        let s = String::from(self.university.as_str());
        s
    }
}

impl Person for Css {
    fn name(&self) -> String {
        let s = String::from(self.name.as_str());
        s
    }
}

trait Person {
    fn name(&self) -> String;
}

trait Student: Person {
    fn university(&self) -> String;
}

trait Programmer {
    fn fav_language(&self) -> String;
}
// CompSciStudent (computer science student，计算机科学的学生) 是 Programmer 和 Student 两者的子类。
// 实现 CompSciStudent 需要你同时 impl 了两个父 trait。
trait CompSciStudent: Programmer + Student {
    fn git_username(&self) -> String;
}

fn comp_sci_student_greeting(student: &dyn CompSciStudent) -> String {
    format!(
        "My name is {} and I attend {}. My favorite language is {}. My Git username is {}",
        student.name(),
        student.university(),
        student.fav_language(),
        student.git_username()
    )
}
