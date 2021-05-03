use actix_cors::Cors;
use actix_files::Files;
use actix_multipart::Multipart;
use actix_web::{
    http::{self},
    middleware, web, App, HttpResponse, HttpServer, Responder, ResponseError,
};
use dotenv::dotenv;
use std::env;
use env::VarError;
use futures::{StreamExt, TryStreamExt};
use log;
use refinery::{self, config::Config};
use serde::{Deserialize, Serialize};
use sqlx;
use sqlx::postgres::{PgPool, PgPoolOptions};
use syslog;
#[macro_use]
extern crate lazy_static;

// let x = include_str!("../client/build/index.html");
// let x = include_str!("../client/build/dist/index.js");
const CSS: &'static str = include_str!("../client/build/dist/index.css");
use sha1::{Digest, Sha1};

lazy_static! {
    static ref CSS_HASH: &'static str = {
        let _hasher = Sha1::digest(CSS.as_bytes());
        ""
    };
}

// This type is reflected on client.
#[derive(Debug)]
pub enum ServerError {
    DatabaseError(String),
    FileSystemError(String),
}

// pub struct InputError {
//     field: String,
//     error_msg: String,
//     title length?
//     field error as a string?
//     incomplete structure for multiple fields? (all must exist?)
//     invalid id (id of _table/struct_ does not exist)
//     email
//     ...
// }

impl std::fmt::Display for ServerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self) // just dump in the server error. This is how the error will be displayed
    }
}

impl ResponseError for ServerError {}

impl From<sqlx::Error> for ServerError {
    fn from(error: sqlx::Error) -> Self {
        ServerError::DatabaseError(error.to_string())
    }
}

#[derive(Debug)]
enum ServerSetupError {
    ReadEnvironmentVariable(std::env::VarError),
    DatabaseSetup(sqlx::Error),
    ServerStart(std::io::Error),
}
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
    embed_migrations!("./src/sql_migrations");
}

// What does this look like when "desugared?"
#[actix_web::main]
async fn main() -> Result<(), ServerSetupError> {
    // pull in all variables from .env, and if there aren't any, discard the error.
    // [environment variables should be ]
    dotenv().ok();

    let db_uri: String = env::var("DATABASE_URI")?;
    let port: String = env::var("PORT")?;
    let mut conn = Config::from_env_var("DATABASE_URI").unwrap();
    embeded::migrations::runner().run(&mut conn).unwrap();

    // setup directory for images
    use std::fs;
    fs::create_dir_all("./tmp/")
        .expect("failed to setup tmp directory for images");

    let pool = PgPoolOptions::new().connect(&db_uri).await?;

    std::env::set_var("RUST_LOG", "actix_web=info,debug");

    // What is this magic?
    // env_logger::init();
    // on mac, run: `tail -f /var/log/system.log` to see system log

    // maybe log in JSON? then tools can pull the JSON. Makes enough sense, because the API is JSON
    syslog::init(
        syslog::Facility::LOG_SYSLOG,
        log::LevelFilter::Debug,
        Some("How I Do"),
    )
    .expect("failed to setup logging");

    #[derive(Serialize)]
    struct Hello {
        msg: String,
    }
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_methods(vec!["GET", "POST", "DELETE"])
            .allow_any_origin();
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
            .wrap(middleware::Compress::new(
                http::header::ContentEncoding::Gzip,
            ))
            .data(pool.clone())
            .route("/", web::get().to(index_html)) // should be the static web app for prod
            .route("/dist/index.js", web::get().to(index_js)) // should be the static web app for prod
            .route("/dist/index.css", web::get().to(index_css)) // should be the static web app for prod
            .route("/test-err", web::get().to(test_err))
            .service(
                web::scope("/api")
                    .service(
                        Files::new("/images", "./tmp").show_files_listing(),
                    )
                    .default_service(web::to(|| {
                        HttpResponse::Ok().json(Hello {
                            msg: String::from("hello from the other side"),
                        })
                    }))
                    .route("/how-to", web::post().to(create_howto))
                    .route("/how-to", web::put().to(update_howto))
                    .route("/how-to/{id}", web::get().to(howto_page))
                    .route("/step", web::post().to(create_step))
                    .route("/step/{id}", web::delete().to(delete_step))
                    .route("/step", web::put().to(update_step))
                    .route("/img-upload", web::post().to(img_upload)),
            )
            .default_service(web::to(index_html))
    })
    // .bind_rustls(port, config)? This is an error because of lib mismatch?
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await?;
    Ok(())
}

async fn index_html() -> impl Responder {
    let x = include_str!("../client/build/index.html");
    // TODO: set eTag to the respective hash
    HttpResponse::Ok().content_type("text/html").body(x)
}
async fn index_js() -> impl Responder {
    let x = include_str!("../client/build/dist/index.js");
    HttpResponse::Ok()
        .content_type("application/javascript")
        .body(x)
}
async fn index_css() -> impl Responder {
    let x = include_str!("../client/build/dist/index.css");
    HttpResponse::Ok().content_type("text/css").body(x)
}

mod hey {
    pub fn hey() {
        log::debug!("hey");
    }
}

async fn test_err() -> Result<HttpResponse, ServerError> {
    hey::hey();
    Err(ServerError::DatabaseError("This is a db error".into()))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct HowToPageProps {
    how_to: HowToDbRow,
    steps: Vec<StepDbRow>,
}

async fn howto_page(
    db_pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
) -> Result<HttpResponse, ServerError> {
    // TODO: Do a transaction!
    // This is getting the how_to?
    let how_to_query = r#"
SELECT id, title
FROM howto
WHERE id = $1
"#;
    let how_to: HowToDbRow = sqlx::query_as(how_to_query)
        .bind(id)
        .fetch_one(&**db_pool)
        .await?;

    let steps_query = r#"
SELECT
    step.id,
    step.title,
    step.image_filename
FROM
    step,
    howto_step
WHERE
    howto_step.howto_id = $1
AND howto_step.step_id = step.id
ORDER BY step.id
"#;
    // TODO: query ****SHOULD NOT **** order by id, but rather by an "order" number

    let steps: Vec<StepDbRow> = sqlx::query_as(steps_query)
        .bind(id)
        .fetch_all(&**db_pool)
        .await?;

    Ok(HttpResponse::Ok().json(HowToPageProps { how_to, steps }))
}

async fn _delete_howto(
    db_pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
) -> impl Responder {
    let resp: Result<_, sqlx::Error> =
        sqlx::query("DELETE FROM howto WHERE id = $1")
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
        len if len > max => {
            Err(format!("Title too long. Max {} characters", max))
        }
        len if len < min => {
            Err(format!("Title too short. Min {} character", min))
        }
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

#[derive(Serialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
struct StepDbRow {
    id: i32,
    title: String,
    image_filename: String,
}

// in
#[derive(Deserialize)]
struct StepCreateData {
    howto_id: i32,
    title: String,
    seconds: i32,
}
async fn create_step(
    json: web::Json<StepCreateData>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, ServerError> {
    // result could be one of these two things, then a bunch of other things within that.
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
        return Ok(HttpResponse::Ok().body("todo"));
    }

    // create a step, then a howto-step. In the same transaction
    // so create a transaction

    let mut tx = db_pool.begin().await?;

    let q = r#"
INSERT INTO step (title)
VALUES ($1, $2)
RETURNING id, title
"#;

    let step: StepDbRow = sqlx::query_as(q)
        .bind(json.title.clone())
        .bind(json.seconds)
        .fetch_one(&mut tx)
        .await?;

    let q2 = r#"
INSERT INTO howto_step (step_id, howto_id)
VALUES ($1, $2)
"#;
    sqlx::query(q2)
        .bind(step.id)
        .bind(json.howto_id)
        .execute(&mut tx)
        .await
        .expect("failed to insert howto_step");

    tx.commit().await?;

    // return HttpResponse::Ok().json(step);
    return Ok(HttpResponse::Ok().body("ok"));
}

#[derive(Deserialize, sqlx::FromRow, Serialize)]
struct StepDeleteData {
    id: i32,
    image_filename: String,
}

async fn delete_step(
    web::Path(id): web::Path<i32>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    // NOTE: this will delete ALL references that have to do with this step.. not what's wanted in the future, but good for now
    sqlx::query(
        r#"
DELETE FROM howto_step
WHERE step_id = $1
"#,
    )
    .bind(id)
    .execute(&**db_pool)
    .await
    .expect("failed to delete howto step");

    let deleted_step: StepDeleteData = sqlx::query_as(
        r#"
DELETE FROM step
WHERE id = $1
RETURNING id, image_filename
"#,
    )
    .bind(id)
    .fetch_one(&**db_pool)
    .await
    .expect("failed to delete step");

    // TODO: delete from file system, async

    use std::fs;
    let s = deleted_step.image_filename.clone();
    let e = web::block(move || {
        fs::remove_file(format!("./tmp/{}", deleted_step.image_filename))
    })
    .await;

    e.expect("failed to delete the image");

    HttpResponse::Ok().json(s)
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
    #[allow(dead_code)]
    image_bytes: Vec<u8>,
}

// Need to know that all these things really exist before starting to save to FS or DB.

use image::{self, GenericImageView};

const IMAGE_SIZE: u32 = 1080;

pub async fn img_upload(
    db_pool: web::Data<PgPool>,
    mut payload: Multipart,
) -> Result<HttpResponse, ServerError> {
    // Should accept `IMAGE_STAR, "image/*"` from mime-types? Is that what a "route guard" is?

    let mut how_to_id: Option<i32> = None;
    let mut image: Option<Image> = None;
    let mut title: Option<String> = None;

    // Process input. All inputs must exist.
    // So this is like a loop that reads from events coming in from memory, but maybe? Only if they are there?
    // What if they never show up? How long does it wait?
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field
            .content_disposition()
            .expect("Failed to read content disposition");

        let field_name = content_type.get_name().unwrap();

        match field_name {
            "howToId" => {
                while let Some(chunk) = field.next().await {
                    let value = chunk.expect("failed to read chunk");
                    // Route validation is done on server side.
                    how_to_id = Some(
                        String::from_utf8_lossy(&value)
                            .parse::<i32>()
                            .expect("Failed to parse howto id"),
                    );
                }
            }
            "title" => {
                while let Some(chunk) = field.next().await {
                    let value = chunk.expect("failed to read chunk");
                    title = Some(String::from_utf8_lossy(&value).into());
                }
                // TODO: check min/max length
            }
            "image" => {
                // Security note: We're not storing the filename, so it does not need sanitizing.

                // Basically a buffer, right? A buffer is a temp area in memory?
                let mut image_bytes = Vec::new();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap(); // "failed to read input - network error" - but what is this error actually?
                    image_bytes.extend_from_slice(&data[..]);
                }
                // Crop and resize the image here
                // let mut img = image::open("toasting.jpg").unwrap();
                // use load_from_memory
                let mut img =
                    image::load_from_memory(&image_bytes[..]).unwrap();

                let (w, h) = img.dimensions();

                // Landscape
                if w > h {
                    let x_offset = (w - h) / 2;
                    img = img.crop(x_offset, 0, h, h);
                }
                // Portrait
                if h > w {
                    let y_offset: u32 = (h - w) / 2;
                    img = img.crop(0, y_offset, w, w);
                }

                img = img.resize(
                    IMAGE_SIZE,
                    IMAGE_SIZE,
                    image::imageops::FilterType::Lanczos3,
                );

                // name the file a random number, like a UUID
                // save this as the name of the file.
                use uuid::Uuid;
                let img_uuid = Uuid::new_v4().to_simple().to_string();

                img.save(format!("./tmp/{}.jpg", &img_uuid)).unwrap();

                image = Some(Image {
                    filename: format!("{}.jpg", img_uuid),
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
    let mut tx = db_pool.begin().await?;

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
    .await?;

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
    .await?;

    // How can this be based on an environment variable? Docker during dev, NFS during prod.
    // let filepath = format!("./tmp/{}", &step_input.image.filename);

    // // create a file handle
    // let mut f = web::block(|| std::fs::File::create(filepath))
    //     .await
    //     .expect("could not create file");

    // // write to that file handle in a thread pool.
    // web::block(move || f.write(&step_input.image.image_bytes).map(|_| f))
    //     .await
    //     .expect("file system write error");
    // Is transaction automatically aborted if the function throws an error? Are all values in the function then 'dropped'?

    let err = ServerError::DatabaseError("err".into());
    log::debug!("{}", err);

    // COMMIT transaction
    tx.commit().await?;

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
