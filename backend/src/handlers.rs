use axum::{extract::{State, Path}, http::StatusCode, response::IntoResponse, Json};
use sqlx::{PgPool, Row};
use chrono::NaiveDate;
use serde_json::json;
use std::str::FromStr;
use uuid::Uuid;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Deserialize, Serialize};

use crate::models::{
    CreateDailyLogInput, Unit, User, LoginInput, CreateUserInput, 
    AuthResponse, CreateUnitInput, Employee, CreateEmployeeInput
};

pub async fn get_employees(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let employees = sqlx::query_as::<_, Employee>(
        "SELECT id, nik, name, company, position, department, status, join_date, 
         contract_type, ktp_number, full_address, village, district, city, province, 
         origin_status, gender, marital_status, religion, birth_place, birth_date, 
         education, email, phone_number, simper_number, simper_expiry, created_at 
         FROM employees ORDER BY name ASC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error fetching employees: {:?}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database Error: {}", e))
    })?;

    Ok(Json(employees))
}

pub async fn register_employee(
    State(pool): State<PgPool>,
    Json(input): Json<CreateEmployeeInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let join_date = match input.join_date {
        Some(ref d) if !d.is_empty() => NaiveDate::from_str(d).ok(),
        _ => None
    };
    let birth_date = match input.birth_date {
        Some(ref d) if !d.is_empty() => NaiveDate::from_str(d).ok(),
        _ => None
    };
    let simper_expiry = match input.simper_expiry {
        Some(ref d) if !d.is_empty() => NaiveDate::from_str(d).ok(),
        _ => None
    };

    sqlx::query(
        "INSERT INTO employees (
            nik, name, company, position, department, status, join_date,
            contract_type, ktp_number, full_address, village, district, city, province,
            origin_status, gender, marital_status, religion, birth_place, birth_date,
            education, email, phone_number, simper_number, simper_expiry
         ) 
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25)
         ON CONFLICT (nik) DO UPDATE SET
            name = EXCLUDED.name,
            company = EXCLUDED.company,
            position = EXCLUDED.position,
            department = EXCLUDED.department,
            status = EXCLUDED.status,
            join_date = EXCLUDED.join_date,
            contract_type = EXCLUDED.contract_type,
            ktp_number = EXCLUDED.ktp_number,
            full_address = EXCLUDED.full_address,
            village = EXCLUDED.village,
            district = EXCLUDED.district,
            city = EXCLUDED.city,
            province = EXCLUDED.province,
            origin_status = EXCLUDED.origin_status,
            gender = EXCLUDED.gender,
            marital_status = EXCLUDED.marital_status,
            religion = EXCLUDED.religion,
            birth_place = EXCLUDED.birth_place,
            birth_date = EXCLUDED.birth_date,
            education = EXCLUDED.education,
            email = EXCLUDED.email,
            phone_number = EXCLUDED.phone_number,
            simper_number = EXCLUDED.simper_number,
            simper_expiry = EXCLUDED.simper_expiry"
    )
    .bind(&input.nik)
    .bind(&input.name)
    .bind(&input.company)
    .bind(&input.position)
    .bind(&input.department)
    .bind(&input.status)
    .bind(join_date)
    .bind(&input.contract_type)
    .bind(&input.ktp_number)
    .bind(&input.full_address)
    .bind(&input.village)
    .bind(&input.district)
    .bind(&input.city)
    .bind(&input.province)
    .bind(&input.origin_status)
    .bind(&input.gender)
    .bind(&input.marital_status)
    .bind(&input.religion)
    .bind(&input.birth_place)
    .bind(birth_date)
    .bind(&input.education)
    .bind(&input.email)
    .bind(&input.phone_number)
    .bind(&input.simper_number)
    .bind(simper_expiry)
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Employee registration error: {}", e);
        (StatusCode::CONFLICT, format!("Data error: {}", e))
    })?;

    Ok(StatusCode::CREATED)
}

pub async fn update_employee(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(input): Json<CreateEmployeeInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let join_date = match input.join_date {
        Some(ref d) if !d.is_empty() => NaiveDate::from_str(d).ok(),
        _ => None
    };
    let birth_date = match input.birth_date {
        Some(ref d) if !d.is_empty() => NaiveDate::from_str(d).ok(),
        _ => None
    };
    let simper_expiry = match input.simper_expiry {
        Some(ref d) if !d.is_empty() => NaiveDate::from_str(d).ok(),
        _ => None
    };

    sqlx::query(
        "UPDATE employees SET 
            nik = $1, name = $2, company = $3, position = $4, department = $5, status = $6, join_date = $7,
            contract_type = $8, ktp_number = $9, full_address = $10, village = $11, district = $12, 
            city = $13, province = $14, origin_status = $15, gender = $16, marital_status = $17, 
            religion = $18, birth_place = $19, birth_date = $20, education = $21, email = $22, 
            phone_number = $23, simper_number = $24, simper_expiry = $25
         WHERE id = $26"
    )
    .bind(&input.nik)
    .bind(&input.name)
    .bind(&input.company)
    .bind(&input.position)
    .bind(&input.department)
    .bind(&input.status)
    .bind(join_date)
    .bind(&input.contract_type)
    .bind(&input.ktp_number)
    .bind(&input.full_address)
    .bind(&input.village)
    .bind(&input.district)
    .bind(&input.city)
    .bind(&input.province)
    .bind(&input.origin_status)
    .bind(&input.gender)
    .bind(&input.marital_status)
    .bind(&input.religion)
    .bind(&input.birth_place)
    .bind(birth_date)
    .bind(&input.education)
    .bind(&input.email)
    .bind(&input.phone_number)
    .bind(&input.simper_number)
    .bind(simper_expiry)
    .bind(id)
    .execute(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::OK)
}

pub async fn delete_employee(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    sqlx::query("DELETE FROM employees WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn register_unit(
    State(pool): State<PgPool>,
    Json(input): Json<CreateUnitInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let ct_expired = match input.ct_expired {
        Some(ref d) if !d.is_empty() => NaiveDate::from_str(d).ok(),
        _ => None
    };

    sqlx::query(
        "INSERT INTO units (cn_unit, model_unit, type_unit, status, hull_number, ct_number, ct_expired, bast_number, skbp_pajak) 
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
    )
    .bind(&input.cn_unit)
    .bind(&input.model_unit)
    .bind(&input.type_unit)
    .bind(&input.status)
    .bind(&input.hull_number)
    .bind(&input.ct_number)
    .bind(ct_expired)
    .bind(&input.bast_number)
    .bind(&input.skbp_pajak)
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Unit creation error: {}", e);
        (StatusCode::CONFLICT, "Unit CN already exists or data error".to_string())
    })?;

    Ok(StatusCode::CREATED)
}

pub async fn delete_unit(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    sqlx::query("DELETE FROM units WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()))?;
    
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn get_units(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let units = sqlx::query_as::<_, Unit>(
        "SELECT id, cn_unit, model_unit, type_unit, status, hull_number, ct_number, ct_expired, bast_number, skbp_pajak FROM units"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
    })?;

    Ok(Json(units))
}

pub async fn create_daily_log(
    State(pool): State<PgPool>,
    Json(input): Json<CreateDailyLogInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let total_hm = input.hm_stop - input.hm_start;
    let mohh = 12.0;

    if input.wh + input.stb + input.bd > mohh {
        return Err((StatusCode::BAD_REQUEST, "Total of WH + STB + BD cannot exceed MOHH".to_string()));
    }

    let parsed_date = NaiveDate::from_str(&input.date).map_err(|_| {
        (StatusCode::BAD_REQUEST, "Invalid date format, expected YYYY-MM-DD".to_string())
    })?;

    sqlx::query("
        INSERT INTO daily_logs (
            unit_id, date, shift, operator_name, work_location, 
            hm_start, hm_stop, total_hm, wh, stb, bd, 
            stb_start, stb_stop, stb_remarks, 
            bd_start, bd_stop, bd_remarks, bd_job_desc, bd_status,
            remarks
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)
    ")
    .bind(input.unit_id)
    .bind(parsed_date)
    .bind(input.shift)
    .bind(input.operator_name)
    .bind(input.work_location)
    .bind(input.hm_start)
    .bind(input.hm_stop)
    .bind(total_hm)
    .bind(input.wh)
    .bind(input.stb)
    .bind(input.bd)
    .bind(input.stb_start)
    .bind(input.stb_stop)
    .bind(input.stb_remarks)
    .bind(input.bd_start)
    .bind(input.bd_stop)
    .bind(input.bd_remarks)
    .bind(input.bd_job_desc)
    .bind(input.bd_status)
    .bind(input.remarks)
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error during insert: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
    })?;

    Ok((StatusCode::CREATED, Json(json!({"success": true, "message": "Log Created Successfully"}))))
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(input): Json<LoginInput>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_record = sqlx::query("SELECT id, name, email, password_hash, role, created_at FROM users WHERE email = $1")
        .bind(&input.email)
        .fetch_optional(&pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "Database error"}))))?
        .ok_or((StatusCode::UNAUTHORIZED, Json(json!({"message": "Invalid email or password"}))))?;

    let user_password_hash: String = user_record.get("password_hash");
    let parsed_hash = PasswordHash::new(&user_password_hash)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "Internal error"}))))?;

    Argon2::default()
        .verify_password(input.password.as_bytes(), &parsed_hash)
        .map_err(|_| (StatusCode::UNAUTHORIZED, Json(json!({"message": "Invalid email or password"}))))?;

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let user_id: Uuid = user_record.get("id");
    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret_key_change_me".as_ref()),
    )
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"message": "Internal error"}))))?;

    let user = User {
        id: user_id,
        name: user_record.get("name"),
        email: user_record.get("email"),
        role: user_record.get("role"),
        created_at: user_record.get("created_at"),
    };

    Ok(Json(AuthResponse { token, user }))
}

pub async fn get_all_users(
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let users = sqlx::query_as::<_, User>(
        "SELECT id, name, email, role, created_at FROM users ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
    })?;

    Ok(Json(users))
}

pub async fn register_user(
    State(pool): State<PgPool>,
    Json(input): Json<CreateUserInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(input.password.as_bytes(), &salt)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Encoding error".to_string()))?
        .to_string();

    sqlx::query("INSERT INTO users (name, email, password_hash, role) VALUES ($1, $2, $3, $4)")
    .bind(&input.name)
    .bind(&input.email)
    .bind(&password_hash)
    .bind(&input.role)
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Registration error: {}", e);
        (StatusCode::CONFLICT, "Email already exists".to_string())
    })?;

    Ok(StatusCode::CREATED)
}

pub async fn delete_user(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string()))?;
    
    Ok(StatusCode::NO_CONTENT)
}

pub async fn update_unit(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(input): Json<CreateUnitInput>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let ct_expired = match input.ct_expired {
        Some(ref d) if !d.is_empty() => NaiveDate::from_str(d).ok(),
        _ => None
    };

    sqlx::query(
        "UPDATE units SET cn_unit = $1, model_unit = $2, type_unit = $3, status = $4, hull_number = $5, ct_number = $6, ct_expired = $7, bast_number = $8, skbp_pajak = $9 WHERE id = $10"
    )
    .bind(&input.cn_unit)
    .bind(&input.model_unit)
    .bind(&input.type_unit)
    .bind(&input.status)
    .bind(&input.hull_number)
    .bind(&input.ct_number)
    .bind(ct_expired)
    .bind(&input.bast_number)
    .bind(&input.skbp_pajak)
    .bind(id)
    .execute(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Update error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
    })?;
    
    Ok(StatusCode::OK)
}

pub async fn get_unit_logs(
    State(pool): State<PgPool>,
    Path(unit_id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let logs = sqlx::query(
        "SELECT id, date, shift, operator_name, 
         hm_start::DOUBLE PRECISION as hm_start, 
         hm_stop::DOUBLE PRECISION as hm_stop, 
         total_hm::DOUBLE PRECISION as total_hm, 
         wh::DOUBLE PRECISION as wh, 
         stb::DOUBLE PRECISION as stb, 
         bd::DOUBLE PRECISION as bd, 
         remarks 
         FROM daily_logs 
         WHERE unit_id = $1 
         ORDER BY date DESC, shift DESC LIMIT 50"
    )
    .bind(unit_id)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        tracing::error!("Logs fetch error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
    })?
    .into_iter()
    .map(|r: sqlx::postgres::PgRow| {
        use sqlx::Row;
        json!({
            "id": r.get::<Uuid, _>("id"),
            "date": r.get::<NaiveDate, _>("date"),
            "shift": r.get::<i16, _>("shift"),
            "operator_name": r.get::<String, _>("operator_name"),
            "hm_start": r.get::<f64, _>("hm_start"),
            "hm_stop": r.get::<f64, _>("hm_stop"),
            "total_hm": r.get::<f64, _>("total_hm"),
            "wh": r.get::<f64, _>("wh"),
            "stb": r.get::<f64, _>("stb"),
            "bd": r.get::<f64, _>("bd"),
            "remarks": r.get::<Option<String>, _>("remarks")
        })
    })
    .collect::<Vec<_>>();

    Ok(Json(logs))
}
