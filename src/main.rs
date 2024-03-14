// Include libraries & packages
use actix_web::{web::Path, get, post, patch, delete, Responder, App, HttpServer, HttpResponse, web::Json};
mod models;
use crate::models::students::{Students, Marks, AddStudents, GetStudentDetails};
use actix_web::web::Data;
use validator::Validate;
use uuid::Uuid;
mod error;
use error::StudentError;
mod db;
use crate::db::{student_data_trait::StudentDataTrait, Database};

#[get("/students/")]
async fn get_all_students(db: Data<Database>) -> Result<Json<Vec<Students>>, StudentError> {
    let students = Database::get_all_students(&db).await;
    match students {
        Some(found_students) => Ok(Json(found_students)),
        None => Err(StudentError::NoSuchStudentFound),
    }
}

#[get("/student-details/{uuid}/")]
async fn get_student_details(path: Path<(String,)>, db: Data<Database>) -> Result<Json<Students>, StudentError> {
    let uuid = path.into_inner().0; // Extract UUID from the path parameter
    let students = Database::get_student_details(&db, uuid).await;
    match students {
        Some(found_students) => Ok(Json(found_students)),
        None => Err(StudentError::NoSuchStudentFound),
    }
}

#[delete("/students/{student_id}/")]
async fn delete_student(db: Data<Database>, student_id: Path<(String,)>) -> Result<HttpResponse, StudentError> {
    let uuid = student_id.into_inner().0; // Extract UUID from the path parameter
    match Database::delete_student(&db, uuid).await {
        Ok(_) => Ok(HttpResponse::NoContent().finish()), // Return 204 No Content if deletion is successful
        Err(_) => Err(StudentError::NoStudentFound), // Return error if deletion fails
    }
}

#[post("/add-students/")]
async fn add_students(body: Json<AddStudents>, db: Data<Database>) -> Result<Json<Students>, StudentError> {
    let is_valid = body.validate();
    match is_valid {
        Ok(_) => {
            let mut buffer = Uuid::encode_buffer();
            let new_uuid = Uuid::new_v4().simple().encode_lower(&mut buffer);
            let name = body.name.clone();
            let roll_no = body.roll_no.clone();
            let class = body.class.clone();
            let math = body.math.clone();
            let science = body.science.clone();
            let english = body.english.clone();
            let sanskrit = body.sanskrit.clone();
            let hindi = body.hindi.clone();
            let social_science = body.social_science.clone();
            let marks = Marks {
                math,
                science,
                english,
                sanskrit,
                hindi,
                social_science
            };
            let new_student = Database::add_students(&db,
                Students::new(
                    String::from(new_uuid),
                    name,
                    roll_no,
                    class,
                    marks
                )
            ).await;
            match new_student {
                Some(created) => {
                    Ok(Json(created))
                },
                None => Err(StudentError::StudentCreationFailure)
            }
        },
        Err(_) => {
            Err(StudentError::StudentCreationFailure)
        }
    }
}


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    println!("Server running at 127.0.0.1:8080");

    let db = Database::init()
        .await
        .expect("Error connecting to database");
    let db_data = Data::new(db);

    
    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .service(get_all_students)
            .service(add_students)
            .service(get_student_details)
            .service(delete_student)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
