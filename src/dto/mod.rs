use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

use crate::types::{LocationData, Subjects};

#[derive(Serialize, Deserialize)]
pub struct UserDTO {
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub class: i16,
    pub school: i32,
    pub email: String,
    pub location_data: LocationData,
}

#[derive(Serialize, Deserialize)]
pub struct TaskDTO {
    pub task: String,
    pub price: f64,
    pub subject: Subjects,
    pub target_finishing_time: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct SlotDTO {
    pub subject: Subjects,
    pub limit: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct SingDto {
    pub username: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct SlotLimitDTO {
    pub limit: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct EmailDTO {
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct PhoneDTO {
    pub phone: Option<String>,
}