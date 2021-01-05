use actix_cors::Cors;
use actix_multipart::Multipart;
use actix_web::{http, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use env::VarError;
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use std::io::Write;

// TLS
// use rustls::internal::pemfile::{certs, pkcs8_private_keys};
// use rustls::NoClientAuth;
// use rustls::ServerConfig;
// use std::fs::File;
// use std::io::BufReader;

// How to impl. responder for sqlx error?

#[derive(Debug)]
enum ServerSetupError {
    ReadEnvironmentVariable(std::env::VarError),
    DatabaseSetup(sqlx::Error),
    ServerStart(std::io::Error),
}
// There are a few of these. How can I get specific info on them? (Doesn't say which env var has the issue)
impl From<VarError> for ServerSetupError {
    fn from(error: VarError) -> Self {
        ServerSetupError::ReadEnvironmentVariable(error)
    }
}
impl From<sqlx::Error> for ServerSetupError {
    fn from(error: sqlx::Error) -> Self {
        ServerSetupError::DatabaseSetup(error)
    }
}
impl From<std::io::Error> for ServerSetupError {
    fn from(error: std::io::Error) -> Self {
        ServerSetupError::ServerStart(error)
    }
}

#[actix_web::main]
async fn main() -> Result<(), ServerSetupError> {
    dotenv().ok();
    let db_uri: String = env::var("DATABASE_URI")?;
    let port: String = env::var("PORT")?;

    let pool = PgPoolOptions::new().connect(&db_uri).await?;

    std::env::set_var("RUST_LOG", "actix_web=info");

    env_logger::init();

    // Setup TLS
    // let mut config = ServerConfig::new(NoClientAuth::new());
    // let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    // let key_file = &mut BufReader::new(File::open("key.pem").unwrap());
    // let cert_chain = certs(cert_file).unwrap();
    // let mut keys = pkcs8_private_keys(key_file).unwrap();
    // config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    HttpServer::new(move || {
        let cors = Cors::default().allow_any_origin(); // maybe don't do this forever
        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::new(
                r#"
%r %s
%b bytes (raw)
%D ms
%U
%{cookie}i

"#,
            ))
            .wrap(
                middleware::DefaultHeaders::new()
                    .header(http::header::SET_COOKIE, "msg=hi; SameSite=Strict"),
            )
            .wrap(middleware::Compress::new(
                http::header::ContentEncoding::Gzip,
            ))
            .data(pool.clone())
            .route(INDEX, web::get().to(index))
            .route(INSTRUCTION, web::post().to(create_instruction))
            .route(INSTRUCTION, web::put().to(update_instruction))
            .route(STEP, web::post().to(create_step))
            .route(STEP, web::delete().to(delete_step))
            .route(STEP, web::put().to(update_step))
            .service(img_upload)
    })
    // .bind_rustls(port, config)?         This is an error because of lib mismatch?
    .bind(port)?
    .run()
    .await?;
    Ok(())
}

const STEP: &'static str = "/step";
const INSTRUCTION: &'static str = "/instruction";
const INDEX: &'static str = "/";

async fn index() -> impl Responder {
    "hello there"
}

// Todo: convert this to a json request
async fn _instruction_page(
    db_pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
) -> impl Responder {
    // TODO: Do a transaction!
    let db_result: Result<InstructionDbRow, sqlx::Error> =
        sqlx::query_as("SELECT id, title FROM instruction WHERE id = $1")
            .bind(id)
            .fetch_one(&**db_pool)
            .await;

    let q = r#"
                SELECT
                    step.id,
                    step.title,
                    step.seconds
                FROM
                    step,
                    instruction_step
                WHERE
                    instruction_step.instruction_id = $1
                AND instruction_step.step_id = step.id
            "#;

    let _steps: Vec<StepDbRow> = sqlx::query_as(q)
        .bind(id)
        .fetch_all(&**db_pool)
        .await
        .expect("failed to fetch steps");

    match db_result {
        Ok(_row) => {}
        Err(sqlx::Error::RowNotFound) => {}
        Err(_) => {}
    }
    "tod"
}

async fn _delete_instruction(
    db_pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
) -> impl Responder {
    let resp: Result<_, sqlx::Error> = sqlx::query("DELETE FROM instruction WHERE id = $1")
        .bind(id)
        .execute(&**db_pool)
        .await;
    if let Err(db_err) = resp {
        return HttpResponse::InternalServerError()
            .body(format!("Internal server error. \n {}", db_err));
    }
    HttpResponse::Found()
        .header(http::header::LOCATION, "/user/instructions")
        .body("delete successful. redirecting you")
}

#[derive(Debug, sqlx::FromRow, Serialize)]
struct InstructionDbRow {
    id: i32,
    title: String,
}

fn validate_length(max: usize, min: usize, input: &str) -> Result<(), String> {
    match input.len() {
        len if len > max => Err(format!("Title too long. Max {} characters", max)),
        len if len < min => Err(format!("Title too short. Min {} character", min)),
        _ => Ok(()),
    }
}
#[derive(Deserialize, sqlx::FromRow, Serialize)]
struct UpdatedInstruction {
    id: i32,
    title: String,
}

async fn update_instruction(
    json: web::Json<UpdatedInstruction>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let trimmed_title = json.title.trim();

    if let Err(_msg) = validate_length(80, 1, trimmed_title) {
        // return HttpResponse::UnprocessableEntity().json(error_info);
        return HttpResponse::Ok().body("error");
    }

    // What goes in is what should come out...
    let updated: UpdatedInstruction =
        sqlx::query_as("UPDATE instruction SET title = $2 WHERE id = $1 RETURNING id, title")
            .bind(json.id)
            .bind(trimmed_title.clone())
            .fetch_one(&**db_pool)
            .await
            // could be more granular here...
            .expect("Failed to update instruction");

    HttpResponse::Ok().json(updated)
}

#[derive(Deserialize)]
struct InstructionCreateData {
    title: String,
}
async fn create_instruction(
    json: web::Json<InstructionCreateData>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let trimmed_title = json.title.trim();

    if let Err(_msg) = validate_length(80, 1, trimmed_title) {
        // let error_info = ErrorInfo {
        //     input: trimmed_title.to_string(),
        //     msg,
        // };
        // return HttpResponse::UnprocessableEntity().json(error_info);
        return HttpResponse::Ok().body("error");
    }

    let q = "
        INSERT INTO instruction (title)
        VALUES ($1)
        RETURNING id, title
    ";
    let created_instruction: InstructionDbRow = sqlx::query_as(q)
        .bind(trimmed_title)
        .fetch_one(&**db_pool)
        .await
        .expect("Failed to insert the row: {}");

    HttpResponse::Ok().json(created_instruction)
}

// out
#[derive(Serialize, sqlx::FromRow)]
struct StepDbRow {
    id: i32,
    title: String,
    seconds: i32,
}

// in
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct StepCreateData {
    instruction_id: i32,
    title: String,
    seconds: i32,
}
async fn create_step(
    json: web::Json<StepCreateData>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    // trim title input
    let trimmed_title = json.title.trim();

    // validate title length
    if let Err(_msg) = validate_length(80, 1, trimmed_title) {
        // let error_info = ErrorInfo {
        //     input: trimmed_title.to_string(),
        //     msg,
        //     // error 422 if there's a validation failure
        // };
        // return HttpResponse::UnprocessableEntity().json(error_info);
        return "todo";
    }

    // create a step, then a instruction-step. In the same transaction
    // so create a transaction

    let mut tx = db_pool
        .begin()
        .await
        .expect("failed to acquire database transaction");

    let q = "INSERT INTO step (title, seconds) VALUES ($1, $2) RETURNING id, title, seconds";

    let step: StepDbRow = sqlx::query_as(q)
        .bind(json.title.clone())
        .bind(json.seconds)
        .fetch_one(&mut tx)
        .await
        .expect("failed to insert step");

    let q2 = "INSERT INTO instruction_step (step_id, instruction_id) VALUES ($1, $2)";
    sqlx::query(q2)
        .bind(step.id)
        .bind(json.instruction_id)
        .execute(&mut tx)
        .await
        .expect("failed to insert instruction_step");

    tx.commit().await.expect("Failed to commit");

    // return HttpResponse::Ok().json(step);
    return "todo";
}

#[derive(Deserialize, sqlx::FromRow, Serialize)]
struct StepDeleteData {
    id: i32,
}

async fn delete_step(
    json: web::Json<StepDeleteData>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    // NOTE: this will delete ALL refrences that have to do with this step.. not what's wanted in the future, but good for now
    sqlx::query("DELETE FROM instruction_step WHERE step_id = $1")
        .bind(json.id)
        .execute(&**db_pool)
        .await
        .expect("failed to delete instruction step");

    let deleted_step: StepDeleteData =
        sqlx::query_as("DELETE FROM step WHERE id = $1 RETURNING id")
            .bind(json.id)
            .fetch_one(&**db_pool)
            .await
            .expect("failed to delete step");
    HttpResponse::Ok().json(deleted_step)
}

#[derive(Deserialize)]
struct StepUpdateData {
    id: i32,
    title: String,
    seconds: i32,
}
async fn update_step(
    json: web::Json<StepUpdateData>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let updated_step: StepDbRow = sqlx::query_as(
        r#"
        UPDATE step
        SET title = $1, seconds = $2
        WHERE id = $3 
        RETURNING id, title, seconds
        "#,
    )
    .bind(&json.title)
    .bind(json.seconds)
    .bind(json.id)
    .fetch_one(&**db_pool)
    .await
    .expect("failed to update step");
    HttpResponse::Ok().json(updated_step)
}

// TODO: convert to a json response
async fn _instructions_page(db_pool: web::Data<PgPool>) -> impl Responder {
    let _rows: Vec<InstructionDbRow> = sqlx::query_as("SELECT id, title FROM instruction")
        .fetch_all(&**db_pool)
        .await
        .expect("Failed to fetch instructions");

    // HttpResponse::Ok()
    //     .header(http::header::CACHE_CONTROL, "no-store, must-revalidate")
    //     .content_type("text/html")
    //     .body(body)
    "todo"
}

#[post("/img-upload")]
pub async fn img_upload(
    _db_pool: web::Data<PgPool>,
    mut payload: Multipart,
) -> Result<HttpResponse, actix_web::Error> {
    // iterate over multipart stream

    // Technically this all could be done in on request - saving the file and the database relation.
    // This would mean using multiple form fields

    // There should only be one field / file
    while let Ok(Some(mut field)) = payload.try_next().await {
        // what is the match syntax for `while let Ok(Some(_VALUE_)) = ...`? Does it stop on iteration?
        // What would make either of these fail? Malformed request?
        let content_type = field
            .content_disposition()
            .expect("Failed to read content disposition");

        // Is the file name a field?
        let filename = content_type
            .get_filename()
            .expect("Failed to read file name");

        let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename));

        // File::create is blocking, use threadpool
        let mut f = web::block(|| std::fs::File::create(filepath))
            .await
            .unwrap();

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write(&data).map(|_| f)).await?;
        }
    }

    Ok(HttpResponse::Ok().body("success"))
}
