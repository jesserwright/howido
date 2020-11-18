use actix_web::{
    get, http, middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder,
};
use askama::Template;
use dotenv::dotenv;
use lazy_static::lazy_static;
use serde::Deserialize;
use sha1::Sha1;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use v_htmlescape::escape as e;

// Make a hash of the prod css content, and change the file name if the content is changed
// This causes the browser to fetch new css
lazy_static! {
    static ref CSS_ROUTE: String = {
        let css = std::fs::read("./css/prod.css").expect("Failed to read css file");
        let mut hasher = Sha1::new();
        hasher.update(&css);
        let result = hasher.digest().to_string();
        format!("/main.{}.css", result)
    };
    static ref CSS_FILE: &'static str = {
        match env::var("PROD_CSS")
            .expect("Faild to read env variable")
            .as_str()
        {
            "true" => include_str!("../css/prod.css"),
            _ => include_str!("../css/dev.css"),
        }
    };
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

"#,
            ))
            .wrap(middleware::Compress::new(
                http::header::ContentEncoding::Gzip,
            ))
            .data(pool.clone())
            .service(instruction_page)
            .service(instructions_page)
            // .service(upload_test)
            // .service(save_file)
            // .service(all_images)
            .route(INDEX, web::get().to(idx))
            .route(ACCOUNT, web::get().to(account_page))
            .route(&CSS_ROUTE, web::get().to(css))
            .route(CREATE_INSTRUCTION, web::post().to(create_instruction))
            .route(CREATE_INSTRUCTION, web::get().to(instruction_form))
            .route(DELETE_INSTRUCTION, web::post().to(delete_instruction))
            .default_service(web::route().to(not_found))
    })
    .bind("localhost:8080")?
    .run()
    .await
}
const DELETE_INSTRUCTION: &'static str = "/delete-instruction";
const CREATE_INSTRUCTION: &'static str = "/create-instruction";
const ACCOUNT: &'static str = "/account";
const INDEX: &'static str = "/";

// Static Routes
fn idx() -> HttpResponse {
    let body = base_page(BasePageProps {
        title: String::from("Home"),
        page_content: String::from("Page content"),
        header_buttons: &[],
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
        header_buttons: &[],
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
fn account_page() -> HttpResponse {
    let body = base_page(BasePageProps {
        title: "Account".to_string(),
        page_content: "<h1>Change your password...</h1>".to_string(),
        header_buttons: &[],
    });
    HttpResponse::Ok().content_type("text/html").body(body)
}
fn instruction_form() -> HttpResponse {
    let body = render_instruction_form(None); // None is no error info
    HttpResponse::Ok().content_type("text/html").body(body)
}

#[get("/instruction/{id}")]
async fn instruction_page(
    db_pool: web::Data<PgPool>,
    web::Path(id): web::Path<i32>,
) -> impl Responder {
    let db_result: Result<InstructionDbRow, sqlx::Error> =
        sqlx::query_as("SELECT id, title FROM instruction WHERE id = $1")
            .bind(id)
            .fetch_one(&**db_pool)
            .await;

    // "handle database error"
    match db_result {
        Ok(row) => {
            let body = base_page(BasePageProps {
                title: row.title,
                page_content: "(steps here)".to_string(),
                header_buttons: &[HeaderButton::Delete("/"), HeaderButton::Edit("/")],
            });
            HttpResponse::Ok().content_type("text/html").body(body)
        }
        Err(sqlx::Error::RowNotFound) => {
            let body = base_page(BasePageProps {
                title: "Instruction not found".to_string(),
                page_content: "This instruction does not exist or has been deleted".to_string(),
                header_buttons: &[HeaderButton::Close("/user/instructions")],
            });
            // HTTP 410
            HttpResponse::Gone().content_type("text/html").body(body)
        }
        Err(_) => {
            // Log errror some how?
            let body = base_page(BasePageProps {
                title: "Internal Server Error 500".to_string(),
                page_content: "We're sorry for the inconvenience".to_string(),
                header_buttons: &[HeaderButton::Close("/user/instructions")],
            });
            // HTTP 500
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
        let body = render_instruction_form(Some(ErrorInfo {
            input: title.to_string(),
            msg,
        }));
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

#[get("/user/instructions")]
async fn instructions_page(db_pool: web::Data<PgPool>) -> impl Responder {
    // TODO: sort by newest at top?
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
        header_buttons: &[HeaderButton::Create(CREATE_INSTRUCTION)],
    });

    HttpResponse::Ok()
        .header(http::header::CACHE_CONTROL, "no-store, must-revalidate")
        .content_type("text/html")
        .body(body)
}

fn error_field(error_info: ErrorInfo) -> String {
    format!(
        r#"
<ul class="text-sm text-red-500">
  <li class="font-medium text-lg overflow-x-auto whitespace-pre">"{}"</li>
  <li>{}</li>
</ul>
"#,
        e(&error_info.input),
        error_info.msg,
    )
}

struct ErrorInfo {
    msg: String,
    input: String,
}

fn render_instruction_form(error_info: Option<ErrorInfo>) -> String {
    let err_fragment = match error_info {
        Some(info) => error_field(info),
        None => "".to_string(),
    };
    let form = format!(
        r#"
<form autocomplete="off" class="flex flex-col" action="{}" method="POST">
    <input autofocus type="text" name="title" placeholder="Title..." class="
        w-full
        focus:border-green-500 focus:outline-none
        py-2 px-3 my-1
        text-lg text-gray-700 leading-tight
        rounded border
        appearance-none
    "/>
    {}
    <button type="submit" class="
        transition ease-in-out duration-150
        bg-green-600 hover:bg-green-700
        shadow-sm hover:shadow-md rounded
        self-streach sm:self-end  
        mt-2 py-1 px-2
        text-white text-lg font-medium
    ">
      Create
    </button>
</form>
    "#,
        CREATE_INSTRUCTION, err_fragment
    );
    let body = base_page(BasePageProps {
        title: "New Instruction".to_string(),
        page_content: form,
        header_buttons: &[HeaderButton::Close("/user/instructions")],
    });
    body
}

fn render_instruction_row(id: i32, title: &str) -> String {
    format!(
        r#"
<a href="/instruction/{id}">
<li class="shadow-sm hover:shadow-md transition-shadow ease-in-out duration-150 bg-white flex justify-between items-center border rounded-md py-2 px-3 mb-2 ">
    <div class="block md:flex flex-grow justify-between items-center">
        <h2 class=" text-md font-bold mb-2 md:mb-0">{title}</h2>
        <span class="whitespace-no-wrap md:mx-4 text-sm">Private&nbsp; 🔒</span>
        <!-- <span class="whitespace-no-wrap md:mx-4 ">Public 🌎</span> -->
    </div>
    <div>
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"> <line x1="5" y1="12" x2="19" y2="12"></line> <polyline points="12 5 19 12 12 19"></polyline></svg>
    </div>
</li>
</a> 
    "#,
        id = id,
        title = e(&title)
    )
}

fn button_base(href: &str, styles: &str, svg_icon: &str) -> String {
    format!(
        r#"<a href="{}" class="transition ease-in-out duration-150 hover:shadow-sm self-start p-2 ml-2 rounded-full tracking-wide {}">{}</a>"#,
        href, styles, svg_icon
    )
}
// The string is the link uri
enum HeaderButton<'a> {
    Create(&'a str),
    Close(&'a str),
    Delete(&'a str),
    Edit(&'a str),
}
impl std::fmt::Display for HeaderButton<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            HeaderButton::Create(href) => {
                let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"> <line x1="12" y1="5" x2="12" y2="19"></line> <line x1="5" y1="12" x2="19" y2="12"></line> </svg> "#;
                f.write_str(&button_base(
                    href,
                    "bg-green-600 hover:bg-green-700 text-white",
                    svg,
                ))
            }
            HeaderButton::Delete(href) => {
                let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"> <polyline points="3 6 5 6 21 6"></polyline> <path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"> </path> <line x1="10" y1="11" x2="10" y2="17"></line> <line x1="14" y1="11" x2="14" y2="17"></line> </svg> "#;
                f.write_str(&button_base(href, "bg-gray-300 hover:bg-gray-400", svg))
            }
            HeaderButton::Close(href) => {
                let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"> <line x1="18" y1="6" x2="6" y2="18"></line> <line x1="6" y1="6" x2="18" y2="18"></line> </svg> "#;
                f.write_str(&button_base(href, "bg-gray-300 hover:bg-gray-400", svg))
            }
            HeaderButton::Edit(href) => {
                let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"> <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path> <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path></svg>"#;
                f.write_str(&button_base(href, "bg-gray-300 hover:bg-gray-400", svg))
            }
        }
    }
}

struct BasePageProps<'a> {
    title: String,                          // Template WILL escape
    page_content: String,                   // Template will NOT escape
    header_buttons: &'a [HeaderButton<'a>], // Template will NOT escape
}

#[derive(Template)]
#[template(path = "base.html")]
struct BasePageTemplate<'a> {
    props: BasePageProps<'a>,
    account_route: &'a str,
    css_route: &'a str,
}

fn base_page(props: BasePageProps) -> String {
    let template = BasePageTemplate {
        props,
        account_route: ACCOUNT,
        css_route: &CSS_ROUTE.to_string(),
    };
    template.render().expect("Template render error")
}
