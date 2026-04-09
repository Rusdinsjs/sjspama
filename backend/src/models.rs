use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, Deserialize, FromRow)]
pub struct Unit {
    pub id: Uuid,
    pub cn_unit: String,
    pub model_unit: String,
    pub type_unit: String,
    pub status: String,
    pub hull_number: Option<String>,
    pub ct_number: Option<String>,
    pub ct_expired: Option<NaiveDate>,
    pub bast_number: Option<String>,
    pub skbp_pajak: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Employee {
    pub id: Uuid,
    pub nik: Option<String>,
    pub name: String,
    pub company: Option<String>,
    pub position: Option<String>,
    pub department: Option<String>,
    pub status: Option<String>,
    pub join_date: Option<NaiveDate>,
    pub contract_type: Option<String>,
    pub ktp_number: Option<String>,
    pub full_address: Option<String>,
    pub village: Option<String>,
    pub district: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub origin_status: Option<String>,
    pub gender: Option<String>,
    pub marital_status: Option<String>,
    pub religion: Option<String>,
    pub birth_place: Option<String>,
    pub birth_date: Option<NaiveDate>,
    pub education: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub simper_number: Option<String>,
    pub simper_expiry: Option<NaiveDate>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Deserialize)]
pub struct CreateEmployeeInput {
    pub nik: Option<String>,
    pub name: String,
    pub company: String,
    pub position: String,
    pub department: String,
    pub status: String,
    pub join_date: Option<String>,
    pub contract_type: Option<String>,
    pub ktp_number: Option<String>,
    pub full_address: Option<String>,
    pub village: Option<String>,
    pub district: Option<String>,
    pub city: Option<String>,
    pub province: Option<String>,
    pub origin_status: Option<String>,
    pub gender: Option<String>,
    pub marital_status: Option<String>,
    pub religion: Option<String>,
    pub birth_place: Option<String>,
    pub birth_date: Option<String>,
    pub education: Option<String>,
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub simper_number: Option<String>,
    pub simper_expiry: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: String,
    pub role: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CreateUserInput {
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct DailyLog {
    pub id: Uuid,
    pub unit_id: Uuid,
    pub date: NaiveDate,
    pub shift: i16,
    pub operator_name: String,
    pub work_location: Option<String>,
    pub hm_start: f64,
    pub hm_stop: f64,
    pub total_hm: f64,
    pub wh: f64,
    pub stb: f64,
    pub bd: f64,
}

#[derive(Deserialize)]
pub struct CreateDailyLogInput {
    pub unit_id: Uuid,
    pub date: String,
    pub shift: i16,
    pub operator_name: String,
    pub work_location: Option<String>,
    pub hm_start: f64,
    pub hm_stop: f64,
    pub wh: f64,
    pub stb: f64,
    pub bd: f64,
    pub stb_start: Option<f64>,
    pub stb_stop: Option<f64>,
    pub stb_remarks: Option<String>,
    pub bd_start: Option<f64>,
    pub bd_stop: Option<f64>,
    pub bd_remarks: Option<String>,
    pub bd_job_desc: Option<String>,
    pub bd_status: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateUnitInput {
    pub cn_unit: String,
    pub model_unit: String,
    pub type_unit: String,
    pub status: String,
    pub hull_number: Option<String>,
    pub ct_number: Option<String>,
    pub ct_expired: Option<String>,
    pub bast_number: Option<String>,
    pub skbp_pajak: Option<String>,
}
