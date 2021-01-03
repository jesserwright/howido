use actix_cors::Cors;
use actix_multipart::Multipart;
use actix_web::{get, http, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use std::io::Write;

// Deps for TLS
// use rustls::internal::pemfile::{certs, pkcs8_private_keys};
// use rustls::NoClientAuth;
// use rustls::ServerConfig;
// use std::fs::File;
// use std::io::BufReader;

// How to impl. responder for sqlx error?

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_uri: String =
        env::var("DATABASE_URI").expect("Failed to parse database connection environment variable");
    let port: String = env::var("PORT").expect("Failed to parse port environment variable");

    let pool = PgPoolOptions::new()
        .connect(&db_uri)
        .await
        .expect("Failed to connect to postgres");

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
        // Setup CORS
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
            .service(upload_page)
    })
    // .bind_rustls(port, config)?         This is an error because of lib mismatch?
    .bind(port)?
    .run()
    .await
}

const STEP: &'static str = "/step";
const INSTRUCTION: &'static str = "/instruction";
const INDEX: &'static str = "/";

async fn index() -> impl Responder {
    "hello there"
}

// Todo: convert this to a json request
async fn instruction_page(
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

    let steps: Vec<StepDbRow> = sqlx::query_as(q)
        .bind(id)
        .fetch_all(&**db_pool)
        .await
        .expect("failed to fetch steps");

    match db_result {
        Ok(row) => {}
        Err(sqlx::Error::RowNotFound) => {}
        Err(_) => {}
    }
    "tod"
}

async fn delete_instruction(
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

    if let Err(msg) = validate_length(80, 1, trimmed_title) {
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

    if let Err(msg) = validate_length(80, 1, trimmed_title) {
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
    if let Err(msg) = validate_length(80, 1, trimmed_title) {
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

// convert to a json response
async fn instructions_page(db_pool: web::Data<PgPool>) -> impl Responder {
    let rows: Vec<InstructionDbRow> = sqlx::query_as("SELECT id, title FROM instruction")
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
    db_pool: web::Data<PgPool>,
    mut payload: Multipart,
) -> Result<HttpResponse, actix_web::Error> {
    // iterate over multipart stream
    let mut filename1: Option<String> = None;

    while let Ok(Some(mut field)) = payload.try_next().await {
        // What would make either of these fail? Malformed request?
        let content_type = field.content_disposition().unwrap();
        // Is the file name a field?
        let filename = content_type.get_filename().unwrap();
        filename1 = Some(filename.to_string());
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

#[get("/upload")]
pub fn upload_page() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form action="/img-upload" method="POST" enctype="multipart/form-data">
                <input type="file" multiple name="file"/>
                <button type="submit">Submit</button>
            </form>
        </body>
    </html>"#;
    HttpResponse::Ok().body(html)
}
