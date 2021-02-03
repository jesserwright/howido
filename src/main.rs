use actix_cors::Cors;
use actix_multipart::Multipart;
use actix_web::{http, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use env::VarError;
use futures::{StreamExt, TryStreamExt};
use refinery::{self, config::Config};
use serde::{Deserialize, Serialize};
use sqlx;
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

// This is it:
// https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html

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

mod embeded {
    use refinery::embed_migrations;
    // What is this macro exposing to the module?
    embed_migrations!("./src/sql_migrations");
}

// Dev & prod should be different. But how?
// What is a dev database? How is it persisted? Docker runs the database - but where are the files?
// Same with images. That should be an external file system. Server can be nuked and everything should be OK.
#[actix_web::main]
async fn main() -> Result<(), ServerSetupError> {
    // Make sure this is called first. Does this put variables into the "global space?" - but only accessable through the env var thing?
    // Env vars are an OS feature, righ?

    // migrate is not working at the moment. Wrong folder maybe?
    // Also is it "ok" to have a migration system and a code base in the same area? pros/cons? Feels like an OK sacrifice.
    // It means the the DB and the application are versioned in the same repo, and the db is kept in sync with application code.
    // This makes developing within it easier, I think.
    // They are separate libs, which is fine I think. Modifying the db does require server restart.

    // This library is meant to be used on development or testing environments in which setting environment variables is not practical.
    // It loads environment variables from a .env file, if available, and mashes those with the actual environment variables provided by the operating system.

    dotenv().ok();

    // In the case of a production deployment, the production environment variables should be set.
    // These are secret, and perhaps could even be locally set.
    // Operating system configuration = how things fail big / are insecure.

    let db_uri: String = env::var("DATABASE_URI")?;
    let port: String = env::var("PORT")?;
    // Run migrations on server start.
    // TODO: error types
    let mut conn = Config::from_env_var("DATABASE_URI").unwrap();
    embeded::migrations::runner().run(&mut conn).unwrap();

    // This should be retried a few times.
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
            .route(HOWTO, web::post().to(create_howto))
            .route(HOWTO, web::put().to(update_howto))
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
const HOWTO: &'static str = "/howto";
const INDEX: &'static str = "/";

async fn index() -> impl Responder {
    "Hello there."
}

// Todo: convert this to a json request
async fn _howto_page(
    db_pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
) -> impl Responder {
    // TODO: Do a transaction!
    let db_result: Result<HowToDbRow, sqlx::Error> =
        sqlx::query_as("SELECT id, title FROM howto WHERE id = $1")
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
                    howto_step
                WHERE
                    howto_step.howto_id = $1
                AND howto_step.step_id = step.id
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

async fn _delete_howto(
    db_pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
) -> impl Responder {
    let resp: Result<_, sqlx::Error> = sqlx::query("DELETE FROM howto WHERE id = $1")
        .bind(id)
        .execute(&**db_pool)
        .await;
    if let Err(db_err) = resp {
        return HttpResponse::InternalServerError()
            .body(format!("Internal server error. \n {}", db_err));
    }
    HttpResponse::Found()
        .header(http::header::LOCATION, "/user/howtos")
        .body("delete successful. redirecting you")
}

#[derive(Debug, sqlx::FromRow, Serialize)]
struct HowToDbRow {
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
struct UpdatedHowTo {
    id: i32,
    title: String,
}

async fn update_howto(
    json: web::Json<UpdatedHowTo>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let trimmed_title = json.title.trim();

    if let Err(_msg) = validate_length(80, 1, trimmed_title) {
        // return HttpResponse::UnprocessableEntity().json(error_info);
        return HttpResponse::Ok().body("error");
    }

    // What goes in is what should come out...
    let updated: UpdatedHowTo = sqlx::query_as(
        r#"
UPDATE howto
SET title = $2
WHERE id = $1
RETURNING id, title
        "#,
    )
    .bind(json.id)
    .bind(trimmed_title.clone())
    .fetch_one(&**db_pool)
    .await
    // could be more granular here...
    .expect("Failed to update howto");

    HttpResponse::Ok().json(updated)
}

#[derive(Deserialize)]
struct CreateHowToInput {
    title: String,
}
async fn create_howto(
    json: web::Json<CreateHowToInput>,
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

    let created_howto: HowToDbRow = sqlx::query_as(
        r#"
INSERT INTO howto (title)
VALUES ($1)
RETURNING id, title
    "#,
    )
    .bind(trimmed_title)
    .fetch_one(&**db_pool)
    .await
    .expect("Failed to insert the row: {}");

    HttpResponse::Ok().json(created_howto)
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
    howto_id: i32,
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

    // create a step, then a howto-step. In the same transaction
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

    let q2 = "INSERT INTO howto_step (step_id, howto_id) VALUES ($1, $2)";
    sqlx::query(q2)
        .bind(step.id)
        .bind(json.howto_id)
        .execute(&mut tx)
        .await
        .expect("failed to insert howto_step");

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
    sqlx::query("DELETE FROM howto_step WHERE step_id = $1")
        .bind(json.id)
        .execute(&**db_pool)
        .await
        .expect("failed to delete howto step");

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

// In this case, maybe get the path ID?
// async fn howto_page(db_pool: web::Data<PgPool>) -> impl Responder {
//     let _rows: Vec<H> = sqlx::query_as("SELECT id, title FROM howto WHERE id = $1")
//     .bind()
//         .fetch_one(&**db_pool)
//         .await
//         .expect("Failed to fetch howtos");

    // HttpResponse::Ok()
    //     .header(http::header::CACHE_CONTROL, "no-store, must-revalidate")
    //     .content_type("text/html")
    //     .body(body)
    // "todo"
// }

// This is actually 'new step'
struct StepInput {
    title: String,
    how_to_id: i32,
    image: Image,
}

struct Image {
    filename: String,
    image_bytes: Vec<u8>,
}

// Need to know that all these things really exist before starting to save to FS or DB.

// Types of errors:
// 1. Request malformed {client system error}
// 2. Input: validation {user error}
// 3. Server error {server error}

#[post("/img-upload")]
pub async fn img_upload(
    db_pool: web::Data<PgPool>,
    mut payload: Multipart,
) -> Result<HttpResponse, actix_web::Error> {
    // Should accept `IMAGE_STAR, "image/*"` from mimetypes? Is that what a "route guard" is?

    let mut how_to_id: Option<i32> = None;
    let mut image: Option<Image> = None;
    let mut title: Option<String> = None;

    // Process input. All inputs must exist.
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .expect("Failed to read content disposition");

        let field_name = content_type.get_name().unwrap();

        match field_name {
            "howToId" => {
                while let Some(chunk) = field.next().await {
                    let value = chunk?;
                    how_to_id = Some(String::from_utf8_lossy(&value).parse::<i32>().unwrap());
                }
            }
            "title" => {
                while let Some(chunk) = field.next().await {
                    let value = chunk?;
                    title = Some(String::from_utf8_lossy(&value).into());
                }
                // TODO: check min/max length
            }
            "image" => {
                use sanitize_filename::sanitize;
                let filename = sanitize(content_type.get_filename().unwrap()); // "none error" is not implemented. Basically 'none' is an error...

                let mut image_bytes = Vec::new();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap(); // "failed to read input - network error"
                    image_bytes.extend_from_slice(&data[..]);
                }
                image = Some(Image {
                    filename,
                    image_bytes,
                });
            }
            _ => {
                ()
                // return Ok(HttpResponse::UnprocessableEntity()
                //     .body(format!("Field '{}' does not exist.", field_name)));
            }
        }
    }

    // Why is `image != None` not possible?
    let step_input = StepInput {
        // TODO: position field is needed as well
        image: image.expect("image not present"),
        how_to_id: how_to_id.expect("id not present"),
        title: title.expect("title not present"),
    };

    // Sweet. Input is now validated, and in memory. Time to persist it.

    // Now for the operations:

    // BEGIN transaction
    let mut tx = db_pool.begin().await.expect("failed to get db transaction");

    // Create step row w/file name & title
    let new_step: StepRow = sqlx::query_as(
        r#"
INSERT INTO step (title, image_filename)
VALUES ($1, $2)
RETURNING *
        "#,
    )
    .bind(&step_input.title)
    .bind(&step_input.image.filename)
    .fetch_one(&mut tx)
    .await
    .expect("db failed");

    const POSITION: i32 = 0;

    // Create howto_step with step_id and howto_id
    let new_howto_step: HowToStepRow = sqlx::query_as(
        r#"
INSERT INTO howto_step (howto_id, step_id, position)
VALUES ($1, $2, $3)
RETURNING *
        "#,
    )
    .bind(step_input.how_to_id)
    .bind(new_step.id)
    .bind(POSITION)
    .fetch_one(&mut tx)
    .await
    .expect("db failed");

    // Create file with image (if fail, manually fail/roll back the transaction)

    // How can this be based on an environment variable? Docker during dev, NFS during prod.
    let filepath = format!("./tmp/{}", &step_input.image.filename);
    println!("{}", filepath);
    let mut f = web::block(|| std::fs::File::create(filepath))
        .await
        .expect("could not create file");

    web::block(move || f.write(&step_input.image.image_bytes).map(|_| f))
        .await
        .expect("file system write error");
    // Is transaction automatically aborted if the function throws an error? Are all values in the function then 'dropped'?

    // COMMIT transaction
    tx.commit().await.expect("failed to commit transaction");

    let r = CreateStepResponse {
        position: new_howto_step.position,
        howto_id: new_howto_step.howto_id,
        step_id: new_step.id,
        title: new_step.title,
        image_filename: new_step.image_filename,
    };

    // Return the position, stepid, howtoid, image file name, title.
    Ok(HttpResponse::Ok().json(r))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateStepResponse {
    position: i32,
    howto_id: i32,
    step_id: i32,
    title: String,
    image_filename: String,
}

#[derive(sqlx::FromRow)]
struct HowToStepRow {
    position: i32,
    howto_id: i32,
    // step_id: i32,
}

#[derive(sqlx::FromRow)]
struct StepRow {
    id: i32,
    title: String,
    image_filename: String,
}
