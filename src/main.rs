use actix_web::{get, http, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use dotenv::dotenv;
use serde::Deserialize;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use validator::Validate;
// TODO: add `asakama_acitx` integration, and use it. It'll automatically set content type for HTML responses.

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_uri = env::var("DATABASE_URL").expect("Failed to parse .env variable for database url");

    let pool = PgPoolOptions::new()
        .connect(&db_uri)
        .await
        .expect("Failed to build postgres pool: {}");

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(move || {
        // What is a closure?
        App::new()
            .wrap(middleware::Logger::new(
                r#"
%r %s
%b bytes (raw)
%D ms
%U

"#,
            ))
            .wrap(middleware::Compress::new(
                http::header::ContentEncoding::Gzip,
            ))
            .data(pool.clone())
            // TODO: static requests don't need to be async!
            // Also, responses can be static.
            .service(index)
            .service(css)
            .service(instructions_page)
            .service(create_instruction)
            .service(instruction_form)
            .service(instruction_page)
            .service(account_page)
            .service(upload_test)
            .service(save_file)
            .service(all_images)
            // TODO: add a not found page
            // "default_service"
    })
    .bind("localhost:8080")?
    .run()
    .await
}

// HOME PAGE
#[derive(Template)]
#[template(path = "index.html")]
struct IndexPage;
#[get("/")]
async fn index() -> impl Responder {
    let template = IndexPage;
    let body = template.render().expect("Template render error");
    HttpResponse::Ok().content_type("text/html").body(body)
}

// CSS FILE
#[get("/main.css")]
async fn css() -> impl Responder {
    // TODO: cache bust with etag (or something) on rebuild.
    // Very important for software update! Or else user will get broken stylesheet!
    HttpResponse::Ok()
        .content_type("text/css")
        .header(http::header::CACHE_CONTROL, "public, max-age=3600")
        .body(include_str!("../css/dev.css"))
}

// ACCOUNT PAGE
#[derive(Template)]
#[template(path = "account.html")]
struct AccountPage;

#[get("/account")] // user-id
async fn account_page() -> impl Responder {
    let t = AccountPage;
    let body = t.render().expect("Template render error");
    HttpResponse::Ok().content_type("text/html").body(body)
}

// CREATE INSTRUCTION FORM
#[derive(Deserialize, Validate)]
struct InstructionCreateFormData {
    #[validate(
        length(max = 80, message = "Too long. Max 80 characters"),
        length(min = 1, message = "Too short. Min 1 character")
    )]
    title: String,
}

fn render_instruction_form(title_error: Option<FieldError>) -> String {
    let body = InstructionCreateForm { title_error };
    body.render().expect("Template render error")
}

#[derive(Template)]
#[template(path = "instruction-form.html")]
struct InstructionCreateForm<'a> {
    title_error: Option<FieldError<'a>>,
}

// Could this be named just "/instruction?" Why/why not? Doesn't matter much rn.
#[get("/create-instruction")]
async fn instruction_form() -> impl Responder {
    let body = render_instruction_form(None);
    HttpResponse::Ok().content_type("text/html").body(body)
}

#[derive(Template)]
#[template(path = "instruction-page.html")]
struct InstructionPage<'a> {
    id: i32,
    title: &'a str,
}

// INSTRUCTION PAGE
#[get("/instruction/{id}")]
async fn instruction_page(
    db_pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
) -> impl Responder {
    let row: InstructionDbRow = sqlx::query_as("SELECT id, title FROM instruction WHERE id = $1")
        .bind(id)
        .fetch_one(&**db_pool)
        .await
        .expect("Failed to fetch instruction: {}");

    let t = InstructionPage {
        id: row.id,
        title: &row.title,
    };

    HttpResponse::Ok()
        .content_type("text/html")
        .body(t.render().expect("Template render failure"))
}

// How to do nested structs in askama?
// Can this error have it's own template?
#[derive(Template)]
#[template(path = "field_error.html")]
struct FieldError<'a> {
    field_value: &'a str,
    errors: Vec<&'a str>,
}

#[derive(Debug, sqlx::FromRow)]
struct InstructionDbRow {
    id: i32,
    title: String,
}

#[post("/create-instruction")]
async fn create_instruction(
    form: web::Form<InstructionCreateFormData>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    if let Err(err) = form.validate() {
        let mut field_error = FieldError {
            field_value: &form.title,
            errors: vec![],
        };
        for e in err.errors() {
            let (_, val_error_kind) = e;
            if let validator::ValidationErrorsKind::Field(field_errs) = val_error_kind {
                for field_err in field_errs {
                    if let Some(msg) = &field_err.message {
                        let m: &str = &msg;
                        field_error.errors.push(m);
                    }
                }
            }
        }

        let body = render_instruction_form(Some(field_error));
        return HttpResponse::UnprocessableEntity()
            // DO NOT store previous form (is this necessary?)
            // .header(http::header::CACHE_CONTROL, "no-store")
            .content_type("text/html")
            .body(body);
    }
    let trimmed_title = &form.title.trim();

    let row: InstructionDbRow = sqlx::query_as(
        r#"
            INSERT INTO instruction (title)
            VALUES ($1)
            RETURNING id, title
        "#,
    )
    .bind(trimmed_title)
    // TODO: what is this double derefrence thing?
    .fetch_one(&**db_pool)
    .await
    // TODO: show the user some error here. Like an error page.
    .expect("Failed to insert the row: {}");

    HttpResponse::Found()
        .content_type("text/html")
        // Redirect to the instruction page
        .header(http::header::LOCATION, format!("/instruction/{}", row.id))
        .body("Instruction succesfully created. Redirecting to instructions page.")
}

// INSTRUCTIONS PAGE
#[derive(Template)]
#[template(path = "instructions.html")]
struct InstructionsPage<'a> {
    user: &'a str,
    instructions: Vec<InstructionPageRow>,
}
#[derive(Debug, Template)]
#[template(path = "instruction-page-row.html")]
struct InstructionPageRow {
    id: i32,
    title: String,
}

#[get("/user/instructions")]
async fn instructions_page(db_pool: web::Data<PgPool>) -> impl Responder {
    let rows: Vec<InstructionDbRow> = sqlx::query_as("SELECT id, title FROM instruction")
        .fetch_all(&**db_pool)
        .await
        .expect("Failed to fetch instructions");

    let page_rows = rows
        .iter()
        .map(|r| InstructionPageRow {
            id: r.id,
            title: r.title.clone(),
        })
        .collect();

    let template = InstructionsPage {
        user: "Jesse",
        instructions: page_rows,
    };

    let body = format!(" {}", template.render().unwrap());
    HttpResponse::Ok()
        // What are the caching defaults? Is the redundant?
        .header(http::header::CACHE_CONTROL, "no-store, must-revalidate")
        .content_type("text/html")
        .body(body)
}

// Multipart file upload endpoint.
// Adopted from: https://github.com/actix/examples/blob/master/multipart/src/main.rs
use actix_multipart::Multipart;
use futures::{StreamExt, TryStreamExt};
use std::io::Write;

#[post("/img-upload")]
async fn save_file(
    db_pool: web::Data<PgPool>,
    mut payload: Multipart,
) -> Result<HttpResponse, actix_web::Error> {
    // iterate over multipart stream
    let mut filename1: Option<String> = None;
    
    while let Ok(Some(mut field)) = payload.try_next().await {
        // What would make either of these fail? Malformed request?
        let content_type = field.content_disposition().unwrap();
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

    // Store in database under the step
    sqlx::query("INSERT INTO step (filename) VALUES ($1)")
        .bind(&filename1)
        .execute(&**db_pool)
        .await
        .expect("db error: {}");

    // What does into do to this request?
    Ok(HttpResponse::Found()
        .header(http::header::LOCATION, "/upload")
        .body("REDIR"))
}

#[get("/upload")]
fn upload_test() -> HttpResponse {
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

#[derive(Template)]
#[template(path = "images-page.html")]
struct ImagesPage<'a> {
    filenames: Vec<&'a str>,
}

#[derive(sqlx::FromRow)]
struct StepRow {
    filename: String,
}
// Render all the files as images.
#[get("/images")]
async fn all_images(db_pool: web::Data<PgPool>) -> HttpResponse {
    // Get all file names

    let steps: Vec<StepRow> = sqlx::query_as("SELECT filename FROM step")
        .fetch_all(&**db_pool)
        .await
        .expect("Db error: {}");

    // render
    let t = ImagesPage {
        filenames: steps.iter().map(|s| s.filename.as_str()).collect(),
    };
    HttpResponse::Ok().body(t.render().expect("template render error"))
}

// A file server
// #[get("/img/*")]