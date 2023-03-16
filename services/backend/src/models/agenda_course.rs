use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AgendaCourseResponse {
    response_code: i32,
    version: String,
    result: AgendaCourse,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgendaCourse {
    author: i32,
    // classes: Option<T>
    // comment: Option<T>
    // create_date: Option<T>
    discipline: AgendaCourseDiscipline,
    end_date: i64,
    start_date: i64,
    is_electronic_signature: bool,
    modality: String,
    name: String,
    prestation_type: i32,
    promotion: String,
    reservation_id: i32,
    rooms: Option<Vec<AgendaCourseRoom>>,
    state: String,
    teacher: String,
    // #[serde(rename = "type")]
    // kind: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgendaCourseRoom {
    room_id: i64,
    name: String,
    floor: String,
    campus: String,
    color: String,
    latitude: String,
    longitude: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgendaCourseDiscipline {
    // coef: Null,
    // ects: Null,
    // has_documents: Null,
    // has_grades: Null,
    // links: Array [],
    name: String,
    nb_students: i32,
    rc_id: i32,
    school_id: i32,
    student_group_id: i32,
    student_group_name: String,
    // syllabus_id: Null,
    teacher: String,
    teacher_id: i32,
    trimester: String,
    trimester_id: i32,
    year: i32,

}
