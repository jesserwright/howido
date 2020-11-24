use actix_web::{http, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use askama::Template;
use dotenv::dotenv;
use lazy_static::lazy_static;
use serde::Deserialize;
use sha1::Sha1;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use v_htmlescape::escape as e;

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_uri = env::var("DATABASE_URL").expect("Failed to parse .env variable for database url");

    let pool = PgPoolOptions::new()
        .connect(&db_uri)
        .await
        .expect("Failed to connect to postgres");

    std::env::set_var("RUST_LOG", "actix_web=info");

    env_logger::init();

    HttpServer::new(move || {
        App::new()
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
            // .service(upload_test)
            // .service(save_file)
            // .service(all_images)
            .route(INDEX, web::get().to(idx))
            .route(&CSS_ROUTE, web::get().to(css))
            .route(INSTRUCTIONS_PAGE, web::get().to(instructions_page))
            .route(INSTRUCTION_FORM, web::get().to(instruction_form))
            .route(INSTRUCTION_RESOURCE, web::get().to(instruction_page))
            .route(INSTRUCTION, web::post().to(create_instruction))
            // This is not so good, to be appending delete. But whatever. Forms don't allow the DELETE http method.
            .route(
                &(INSTRUCTION_RESOURCE.to_owned() + "/delete"),
                web::post().to(delete_instruction),
            )
            .route(
                "/update-instruction-form/{id}",
                web::get().to(update_instruction_form),
            )
            .default_service(web::route().to(not_found))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
const INSTRUCTIONS_PAGE: &'static str = "/user/instructions";
const INSTRUCTION_FORM: &'static str = "/create-instruction";
const INSTRUCTION_RESOURCE: &'static str = "/instruction/{id}";
const INSTRUCTION: &'static str = "/instruction";
const INDEX: &'static str = "/";

fn idx() -> HttpResponse {
    let body = base_page(BasePageProps {
        title: String::from("Home"),
        page_content: String::from("Page content"),
    });
    HttpResponse::Ok().content_type("text/html").body(body)
}
fn not_found(req: HttpRequest) -> HttpResponse {
    let uri = req.uri();
    let body = base_page(BasePageProps {
        title: "404 Not Found".to_string(),
        page_content: format!(
            r#"The route <b>"{}"</b> is not available."#,
            e(&uri.to_string())
        ),
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

async fn instruction_page(
    db_pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
) -> impl Responder {
    let db_result: Result<InstructionDbRow, sqlx::Error> =
        sqlx::query_as("SELECT id, title FROM instruction WHERE id = $1")
            .bind(id)
            .fetch_one(&**db_pool)
            .await;
    match db_result {
        Ok(row) => {
            let body = base_page(BasePageProps {
                title: row.title,
                page_content: "(steps)".to_string(),
            });
            HttpResponse::Ok().content_type("text/html").body(body)
        }
        Err(sqlx::Error::RowNotFound) => {
            let body = base_page(BasePageProps {
                title: "Instruction not found".to_string(),
                page_content: "This instruction does not exist or has been deleted".to_string(),
            });
            HttpResponse::Gone().content_type("text/html").body(body)
        }
        Err(_) => {
            let body = base_page(BasePageProps {
                title: "Internal Server Error 500".to_string(),
                page_content: "We're sorry for the inconvenience".to_string(),
            });
            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(body)
        }
    }
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

#[derive(Debug, sqlx::FromRow)]
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

#[derive(Deserialize)]
struct InstructionCreateFormData {
    title: String,
}
async fn create_instruction(
    form: web::Form<InstructionCreateFormData>,
    db_pool: web::Data<PgPool>,
) -> impl Responder {
    let title = &form.title.trim();

    if let Err(msg) = validate_length(80, 1, title) {
        let body = render_instruction_form(
            Some(ErrorInfo {
                input: title.to_string(),
                msg,
            }),
            None,
        );
        return HttpResponse::UnprocessableEntity()
            .header(http::header::CACHE_CONTROL, "no-store, must-revalidate")
            .content_type("text/html")
            .body(body);
    }

    let row: InstructionDbRow = sqlx::query_as(
        r#"
            INSERT INTO instruction (title)
            VALUES ($1)
            RETURNING id, title
        "#,
    )
    .bind(title)
    .fetch_one(&**db_pool)
    .await
    .expect("Failed to insert the row: {}");

    HttpResponse::Found()
        .content_type("text/html")
        .header(http::header::LOCATION, format!("/instruction/{}", row.id))
        .body("Instruction succesfully created. Redirecting to instructions page.")
}

async fn instructions_page(db_pool: web::Data<PgPool>) -> impl Responder {
    let rows: Vec<InstructionDbRow> = sqlx::query_as("SELECT id, title FROM instruction")
        .fetch_all(&**db_pool)
        .await
        .expect("Failed to fetch instructions");
    let page_rows: String = rows
        .iter()
        .map(|row| render_instruction_row(row.id, &row.title))
        .collect();
    let body = base_page(BasePageProps {
        title: "Instructions".to_string(),
        page_content: format!("<ul>{}</ul>", page_rows),
    });

    HttpResponse::Ok()
        .header(http::header::CACHE_CONTROL, "no-store, must-revalidate")
        .content_type("text/html")
        .body(body)
}

#[derive(Template)]
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
    });
    body
}

#[derive(Template)]
#[template(path = "instruction-row.html")]
struct InstructionRowTemplate<'a> {
    id: i32,
    title: &'a str,
}

fn render_instruction_row(id: i32, title: &str) -> String {
    let t = InstructionRowTemplate { id, title };
    t.render().unwrap()
}

struct BasePageProps {
    title: String,
    page_content: String, // UNSAFE RENDER
}
#[derive(Template)]
#[template(path = "base.html")]
struct BasePageTemplate<'a> {
    props: BasePageProps,
    css_route: &'a str,
}
fn base_page(props: BasePageProps) -> String {
    let template = BasePageTemplate {
        props,
        css_route: &CSS_ROUTE.to_string(),
    };
    template.render().expect("Template render error")
}
