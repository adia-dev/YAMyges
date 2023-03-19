use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgendaResponse {
    response_code: i64,
    version: String,
    result: Vec<AgendaCourse>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgendaCourse {
    reservation_id: i64,
    // #[serde(default)]
    rooms: Option<Vec<Room>>,
    #[serde(rename = "type")]
    kind: String,
    modality: String,
    author: i64,
    create_date: Value,
    start_date: i64,
    end_date: i64,
    state: String,
    comment: Option<String>,
    classes: Value,
    name: String,
    discipline: Discipline,
    teacher: String,
    promotion: String,
    prestation_type: i64,
    is_electronic_signature: bool,
    links: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Room {
    links: Vec<Value>,
    room_id: i64,
    name: String,
    floor: String,
    campus: String,
    color: String,
    latitude: String,
    longitude: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Discipline {
    coef: Value,
    ects: Value,
    name: Option<String>,
    teacher: String,
    trimester: Option<String>,
    year: Option<i64>,
    links: Vec<Value>,
    has_documents: Value,
    has_grades: Value,
    nb_students: i64,
    rc_id: Option<i64>,
    school_id: Option<i64>,
    student_group_id: Option<i64>,
    student_group_name: Option<String>,
    syllabus_id: Value,
    teacher_id: Option<i64>,
    trimester_id: Option<i64>,
}
