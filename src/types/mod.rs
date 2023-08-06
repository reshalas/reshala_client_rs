use chrono::{NaiveDateTime, NaiveDate, NaiveTime};
use serde::{Deserialize, Serialize};

pub mod slot;
pub mod slots_component;
pub mod tasks_component;
pub mod task;
pub mod transaction;
pub mod user;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Subjects {
    Math,
    Geometry,
    RussianLang,
    UzbekLang,
    EnglishLang,
    WorldHistory,
    HistoryOfUzbekistan,
    Biology,
    Chemistry,
    Drowing,
    Physics,
    Literature,
    Economy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    username: String,
    password: String,
    frirst_name: String,
    last_name: String,
    class: i16,
    school: i32,
    email: String,
    phone_number: Option<String>,
    slots_component: SlotsComponent,
    raiting: Vec<i16>,
    tasks_component: TasksComponent,
    location_data: LocationData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationData {
    pub town: Option<String>,
    pub country: Option<String>,
    pub village: Option<String>,
    pub city: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SlotsComponent {
    slots: Vec<Slot>,
    owner_username: String
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TasksComponent {
    tasks: Vec<Task>,
    owner_username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slot {
    id: String,
    owner_username: String,
    subject: Subjects,
    limit: Option<i32>,
    pending_transactions: Option<Vec<Transaction>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    id: String,
    sender_slot: String,
    recipient_username: String,
    task: String,
    price: f64,
    passed: bool,
    time_of_rigestration: NaiveTime,
    date_of_rigestration: NaiveDate,
    time_of_ending: Option<NaiveTime>,
    date_of_ending: Option<NaiveDate>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Task {
    id: String,
    owner_username: String,
    task: String,
    price: f64,
    subject: Subjects,
    publish_time: NaiveDateTime,
    target_finishing_time: NaiveDateTime,
}