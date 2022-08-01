use serde::{Deserialize,Serialize};
use chrono::NaiveDateTime;
use actix_web::web;


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Course {
   pub tutor_id: usize,
   pub course_id: Option<usize>,
   pub course_name: String,
   pub posted_time: Option<NaiveDateTime>,
}

impl From<web::Json<Course>> for Course {
    fn from(course: web::Json<Course>) -> Self {
        Course {
            tutor_id: course.tutor_id,
            course_id: course.course_id,
            course_name: course.course_name.clone(),
            posted_time: course.posted_time,
        }
    }
}