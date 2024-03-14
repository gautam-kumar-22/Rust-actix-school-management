// Pizza-data-trait

use crate::models::{Students};
use crate::{db::Database};
use actix_web::web::Data;
use async_trait::async_trait;
use surrealdb::{Error};

#[async_trait]
pub trait StudentDataTrait {
    async fn get_all_students(db: &Data<Database>) -> Option<Vec<Students>>;
    async fn add_students(db: &Data<Database>, new_student: Students) -> Option<Students>;
    // async fn get_student_details(db: &Data<Database>, student_id: String) -> Option<Students>;
    // async fn update_student(db: &Data<Database>, uuid: String) -> Option<Students>;
}

#[async_trait]
impl StudentDataTrait for Database {
    async fn get_all_students(db: &Data<Database>) -> Option<Vec<Students>> {
        let result = db.client.select("student").await;
        match result {
            Ok(all_students) => Some(all_students),
            Err(_) => None
        }
    }

    // async fn get_student_details(db: &Data<Database>, student_id: String) -> Option<Students>  {
    //     let result = db.client.select(("student", student_id)).await;
    //     match result {
    //         Ok(found_student) => Some(found_student.expect("Student not found")),
    //         Err(_) => None
    //     }
    // }

    async fn add_students(db: &Data<Database>, new_student: Students) -> Option<Students> {
        let created_student = db
                            .client
                            .create(("student", new_student.uuid.clone()))
                            .content(new_student)
                            .await;
        match created_student {
            Ok(created) => created,
            Err(_) => None,
        }
                
    }

    // async fn update_pizza(db: &Data<Database>, uuid: String) -> Option<Pizza> {
    //     let find_pizza: Result<Option<Pizza>, Error> = db.client.select(("pizza", &uuid)).await;

    //     match find_pizza {
    //         Ok(found) => {
    //             match found {
    //                 Some(_found_pizza) => {
    //                     let updated_pizza: Result<Option<Pizza>, Error> = db
    //                         .client
    //                         .update(("pizza", &uuid))
    //                         .merge(Pizza {
    //                             uuid,
    //                             pizza_name: String::from("Sold")
    //                         })
    //                         .await;
    //                     match updated_pizza {
    //                         Ok(updated) => updated,
    //                         Err(_) => None,
    //                     }
    //                 },
    //                 None => None,
    //             }
    //         },
    //         Err(_) => None,
    //     }
    // }


}
