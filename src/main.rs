use actix_cors::Cors;
use actix_multipart::Multipart;
use actix_web::{http, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use askama::Template;
use dotenv::dotenv;
use futures::{StreamExt, TryStreamExt};
use lazy_static::lazy_static;
use rustls::internal::pemfile::{certs, pkcs8_private_keys};
use rustls::NoClientAuth;
use rustls::ServerConfig;
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::Write;

lazy_static! {
    static ref CSS_ROUTE: String = {
        // Make a hash of the prod css content, and change the file name if the content is changed.
        // This causes the browser to always fetch fresh CSS.
        let css = include_bytes!("../build.css");
        let mut hasher = Sha1::new();
        hasher.update(css);
        let hash = hasher.digest().to_string();
        format!("/main.{}.css", hash)
    };
    static ref CSS_FILE: &'static str = include_str!("../build.css");
}

// How to impl. responder for sqlx error?
// Dart CLI program for multipart file upload on localhost
// Start with the end in mind - what does the static web page look like?

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
    let mut config = ServerConfig::new(NoClientAuth::new());
    let cert_file = &mut BufReader::new(File::open("cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("key.pem").unwrap());
    let cert_chain = certs(cert_file).unwrap();
    let mut keys = pkcs8_private_keys(key_file).unwrap();
    config.set_single_cert(cert_chain, keys.remove(0)).unwrap();

    HttpServer::new(move || {
        // This is cors can it be tested with the REST client?
        let cors = Cors::default();
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
            .route(INDEX, web::get().to(idx))
            .route(&CSS_ROUTE, web::get().to(css))
            .route(INSTRUCTIONS_PAGE, web::get().to(instructions_page))
            .route(INSTRUCTION_FORM, web::get().to(instruction_form))
            .route(INSTRUCTION_RESOURCE, web::get().to(instruction_page))
            .route(INSTRUCTION, web::post().to(create_instruction))
            .route(INSTRUCTION, web::put().to(update_instruction))
            .route(
                &(INSTRUCTION_RESOURCE.to_owned() + "/delete"),
                web::post().to(delete_instruction),
            )
            .route(
                "/update-instruction-form/{id}",
                web::get().to(update_instruction_form),
            )
            .route(STEP, web::post().to(create_step))
            .route(STEP, web::delete().to(delete_step))
            .route(STEP, web::put().to(update_step))
            // Images
            .default_service(web::route().to(not_found))
    })
    .bind_rustls(port, config)?
    .run()
    .await
}

const STEP: &'static str = "/step";
const INSTRUCTIONS_PAGE: &'static str = "/user/instructions";
const INSTRUCTION_FORM: &'static str = "/create-instruction";
const INSTRUCTION_RESOURCE: &'static str = "/instruction/{id}";
const INSTRUCTION: &'static str = "/instruction";
const INDEX: &'static str = "/";

fn idx() -> HttpResponse {
    let body = base_page(BasePageProps {
        title: String::from("Home"),
        page_content: String::from("Page content"),
        js: None,
    });
    HttpResponse::Ok().content_type("text/html").body(body)
}

#[derive(Template)]
#[template(source = "The route <b>{{uri}}</b> is not available.", ext = "html")]
struct NotFoundPage<'a> {
    uri: &'a str,
}

fn not_found(req: HttpRequest) -> HttpResponse {
    let uri = req.uri().to_string();
    let t = NotFoundPage { uri: &uri };
    let body = base_page(BasePageProps {
        title: "404 Not Found".to_string(),
        page_content: t.render().expect("template render error"),
        js: None,
    });
    HttpResponse::NotFound()
        .content_type("text/html")
        .body(body)
}

fn css() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css")
        .header(http::header::CACHE_CONTROL, "max-age=31536000") // cache for a year
        .body(CSS_FILE.to_string())
}

fn instruction_form() -> HttpResponse {
    let body = render_instruction_form(None, None); // None is no error info
    HttpResponse::Ok().content_type("text/html").body(body)
}

async fn update_instruction_form(web::Path(id): web::Path<i32>) -> impl Responder {
    let body = render_instruction_form(None, Some(&format!("I'm a title. Update me. ID: {}", id)));
    HttpResponse::Ok().content_type("text/html").body(body)
}

#[derive(Template)]
#[template(path = "instruction-page.html")]
struct InstructionPageTemplate<'a> {
    title: &'a str,
    steps: Vec<StepDbRow>,
}

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
        Ok(row) => {
            let t = InstructionPageTemplate {
                title: &row.title.clone(),
                steps,
            };
            let body = base_page(BasePageProps {
                title: row.title.to_owned(),
                page_content: t.render().expect("template render error"),
                js: Some(include_str!("../templates/instruction-page.js")),
            });
            HttpResponse::Ok().content_type("text/html").body(body)
        }
        Err(sqlx::Error::RowNotFound) => {
            let body = base_page(BasePageProps {
                title: "Instruction not found".to_string(),
                page_content: "This instruction does not exist or has been deleted".to_string(),
                js: None,
            });
            HttpResponse::NotFound()
                .content_type("text/html")
                .body(body)
        }
        Err(_) => {
            let body = base_page(BasePageProps {
                title: "Internal Server Error 500".to_string(),
                page_content: "We're sorry for the inconvenience".to_string(),
                js: None,
            });
            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(body)
        }
    }
}

// TODO: actually create a route and button for this
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
    // !! This was copy pasted from another method endpoint !!

    let trimmed_title = json.title.trim();

    if let Err(msg) = validate_length(80, 1, trimmed_title) {
        let error_info = ErrorInfo {
            input: trimmed_title.to_string(),
            msg,
        };
        return HttpResponse::UnprocessableEntity().json(error_info);
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
        let error_info = ErrorInfo {
            input: trimmed_title.to_string(),
            msg,
        };
        return HttpResponse::UnprocessableEntity().json(error_info);
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
        let error_info = ErrorInfo {
            input: trimmed_title.to_string(),
            msg,
            // error 422 if there's a validation failure
        };
        return HttpResponse::UnprocessableEntity().json(error_info);
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

    return HttpResponse::Ok().json(step);
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

#[derive(Template)]
#[template(path = "instructions.html")]
struct InstructionsPageTemplate<'a> {
    title: &'a str,
    instructions: Vec<InstructionPage<'a>>,
}
async fn instructions_page(db_pool: web::Data<PgPool>) -> impl Responder {
    let rows: Vec<InstructionDbRow> = sqlx::query_as("SELECT id, title FROM instruction")
        .fetch_all(&**db_pool)
        .await
        .expect("Failed to fetch instructions");

    let template_rows: Vec<InstructionPage> = rows
        .iter()
        // TODO: if the row is the same as the template, just direct map the same structure!
        .map(|row| InstructionPage {
            id: row.id,
            title: &row.title,
        })
        .collect();

    let page_content = InstructionsPageTemplate {
        title: "Jesse's Instructions",
        instructions: template_rows,
    };

    let body = base_page(BasePageProps {
        title: "Instructions".to_string(),
        page_content: page_content.render().expect("render error"),
        js: Some(include_str!("../templates/create-instruction.js")),
    });

    HttpResponse::Ok()
        .header(http::header::CACHE_CONTROL, "no-store, must-revalidate")
        .content_type("text/html")
        .body(body)
}

#[derive(Template, Serialize)]
#[template(path = "form-error.html")]
struct ErrorInfo {
    msg: String,
    input: String,
}

#[derive(Template)]
#[template(path = "new-instruction.html")]
struct NewInstructionFormTemplate<'a> {
    action_uri: &'a str,
    value: &'a str,
    error: &'a str,
}

fn render_instruction_form(error_info: Option<ErrorInfo>, title_value: Option<&str>) -> String {
    let err_fragment = match error_info {
        Some(info) => info.render().expect("template render error"),
        None => "".to_string(),
    };

    let t = NewInstructionFormTemplate {
        action_uri: INSTRUCTION,
        value: title_value.unwrap_or(""),
        error: &err_fragment,
    };
    let body = base_page(BasePageProps {
        title: "New Instruction".to_string(),
        page_content: t.render().expect("Template render error"),
        js: None,
    });
    body
}

// #[derive(Template)]
// #[template(path = "instruction-row.html")]
struct InstructionTemplate<'a> {
    id: i32,
    title: &'a str,
}

struct BasePageProps<'a> {
    title: String,
    page_content: String,
    js: Option<&'a str>,
}

#[derive(Template)]
#[template(path = "base.html")]
struct BasePageTemplate<'a> {
    props: BasePageProps<'a>,
    css_route: &'a str,
}

fn base_page(props: BasePageProps) -> String {
    let template = BasePageTemplate {
        props,
        css_route: &CSS_ROUTE.to_string(),
    };
    template.render().expect("Template render error")
}

// #[post("/img-upload")]
// pub async fn save_file(
//     db_pool: web::Data<PgPool>,
//     mut payload: Multipart,
// ) -> Result<HttpResponse, actix_web::Error> {
//     // iterate over multipart stream
//     let mut filename1: Option<String> = None;

//     while let Ok(Some(mut field)) = payload.try_next().await {
//         // What would make either of these fail? Malformed request?
//         let content_type = field.content_disposition().unwrap();
//         // Is the file name a field?
//         let filename = content_type.get_filename().unwrap();
//         filename1 = Some(filename.to_string());
//         let filepath = format!("./tmp/{}", sanitize_filename::sanitize(&filename));

//         // File::create is blocking, use threadpool
//         let mut f = web::block(|| std::fs::File::create(filepath))
//             .await
//             .unwrap();

//         // Field in turn is stream of *Bytes* object
//         while let Some(chunk) = field.next().await {
//             let data = chunk.unwrap();
//             // filesystem operations are blocking, we have to use threadpool
//             f = web::block(move || f.write(&data).map(|_| f)).await?;
//         }
//     }

//     // Store in database under the step
//     // sqlx::query("INSERT INTO step (filename) VALUES ($1)")
//     //     .bind(&filename1)
//     //     .execute(&**db_pool)
//     //     .await
//     //     .expect("db error: {}");

//     Ok(HttpResponse::Found()
//         .header(http::header::LOCATION, "/upload")
//         .body("REDIR"))
// }

// #[get("/upload")]
// pub fn upload_test() -> HttpResponse {
//     let html = r#"<html>
//         <head><title>Upload Test</title></head>
//         <body>
//             <form action="/img-upload" method="POST" enctype="multipart/form-data">
//                 <input type="file" multiple name="file"/>
//                 <button type="submit">Submit</button>
//             </form>
//         </body>
//     </html>"#;
//     HttpResponse::Ok().body(html)
// }

#[derive(sqlx::FromRow)]
struct StepRow {
    filename: String,
}
// Render all the files as images.
#[get("/images")]
pub async fn all_images(db_pool: web::Data<PgPool>) -> HttpResponse {
    // Get all file names
    let steps: Vec<StepRow> = sqlx::query_as("SELECT filename FROM step")
        .fetch_all(&**db_pool)
        .await
        .expect("Db error: {}");

    let images: String = steps
        .iter()
        .map(|s| format!(r#"<img src="/img/{}">"#, &s.filename))
        .collect();

    HttpResponse::Ok().body(images)
}
