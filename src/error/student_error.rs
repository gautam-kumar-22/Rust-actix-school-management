use derive_more::Display;
use actix_web:: {
    http:: {header:: ContentType, StatusCode},
    HttpResponse, ResponseError
};



#[derive(Debug, Display)]
pub enum StudentError {
    NoStudentFound,
    StudentCreationFailure,
    NoSuchStudentFound
}


impl ResponseError for StudentError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            StudentError::NoStudentFound => StatusCode::NOT_FOUND,
            StudentError::StudentCreationFailure => StatusCode::INTERNAL_SERVER_ERROR,
            StudentError::NoSuchStudentFound => StatusCode::NOT_FOUND,
        }
    }

}

