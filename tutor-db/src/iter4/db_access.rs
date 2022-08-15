use super::errors::EzTutorError;
use super::models::Course;
use sqlx::PgPool;

pub async fn get_courses_for_tutor_db(
    pool: &PgPool,
    tutor_id: i32,
) -> Result<Vec<Course>, EzTutorError> {
    let course_rows = sqlx::query!(r#"select course_id, tutor_id, course_name, posted_time from ezy_course_c5 where course_id = $1"#, tutor_id)
    .fetch_all(pool)
    .await?;

    let courses: Vec<Course> = course_rows
        .iter()
        .map(|r| Course {
            course_id: r.course_id,
            tutor_id: r.tutor_id,
            course_name: r.course_name.to_owned(),
            posted_time: r.posted_time,
        })
        .collect();
    match courses.len() {
        0 => Err(EzTutorError::NotFound("Courses not found for tutor".into())),
        _ => Ok(courses),
    }
}

pub async fn get_course_details_db(
    pool: &PgPool,
    tutor_id: i32,
    course_id: i32,
) -> Result<Course, EzTutorError> {
    // Prepare SQL statement
    let course_row = sqlx::query!(
        "SELECT tutor_id, course_id, course_name, posted_time
    FROM ezy_course_c5 where tutor_id = $1 and course_id = $2",
        tutor_id,
        course_id
    )
    .fetch_one(pool)
    .await;
    if let Ok(course_row) = course_row {
        // Execute query
        Ok(Course {
            course_id: course_row.course_id,
            tutor_id: course_row.tutor_id,
            course_name: course_row.course_name.clone(),
            posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
        })
    } else {
        Err(EzTutorError::NotFound("Course id not found".into()))
    }
}

pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Course {
    let course_row = sqlx::query!(
        "insert into ezy_course_c5 (course_id,tutor_id, course_name)
   values ($1,$2,$3) returning tutor_id, course_id,course_name, posted_time",
        new_course.course_id,
        new_course.tutor_id,
        new_course.course_name
    )
    .fetch_one(pool)
    .await
    .unwrap();
    //Retrieve result
    Course {
        course_id: course_row.course_id,
        tutor_id: course_row.tutor_id,
        course_name: course_row.course_name.clone(),
        posted_time: Some(chrono::NaiveDateTime::from(course_row.posted_time.unwrap())),
    }
}
