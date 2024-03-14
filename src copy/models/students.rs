// Students models
use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct Marks {
    pub math: i32,
    pub science: i32,
    pub english: i32,
    pub sanskrit: i32,
    pub hindi: i32,
    pub social_science: i32
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct Students {
    pub uuid: String,
    pub name: String,
    pub roll_no: String,
    pub class: String,
    pub marks: Marks
}

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct AddStudents {
    #[validate(length(min=5, message="Student name is required."))]
    pub name: String,
    #[validate(length(min=1, message="Student roll number is required."))]
    pub roll_no: String,
    #[validate(length(min=1, message="Student class is required."))]
    pub class: String,
    pub math: i32,
    pub science: i32,
    pub english: i32,
    pub sanskrit: i32,
    pub hindi: i32,
    pub social_science: i32
}

// #[derive(Serialize, Deserialize, Validate, Debug)]
// pub struct GetStudentDetails {
//     pub uuid: String
// }

impl Students {
    pub fn new(uuid: String, name: String, roll_no: String, class: String, marks: Marks) -> Students{
        Students {
            uuid,
            name,
            roll_no,
            class,
            marks
        }
    }
}

