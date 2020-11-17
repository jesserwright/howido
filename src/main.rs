use actix_web::{get, http, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::Deserialize;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use v_htmlescape::escape as e;

// use lazy_static::lazy_static;
// lazy_static! {
// }
// A new build should:
// > make a new route (exact match)
// > make a new request (exact match to the md5 of the css)
// > the hash will always be generated, but invalidation will only happen if it changed.
// use sha1::{Sha1, Digest};
// let mut hasher = Sha1::new();
// main.{name}.css

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_uri = env::var("DATABASE_URL").expect("Failed to parse .env variable for database url");

    let pool = PgPoolOptions::new()
        .connect(&db_uri)
        .await
        .expect("Failed to connect to postgres: {}");

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
            .service(upload_test)
            .service(save_file)
            .service(all_images)
            .route("/", web::get().to(idx))
            .route("/account", web::get().to(account_page))
            .route("/main.css", web::get().to(css))
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

fn idx() -> HttpResponse {
    let body = base_page(String::from("Home"), String::from("Page content"), &[]);
    HttpResponse::Ok().content_type("text/html").body(body)
}
fn not_found() -> HttpResponse {
    HttpResponse::NotFound()
        .content_type("text/html")
        .body("404 Not found")
}
fn css() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css")
        .header(http::header::CACHE_CONTROL, "max-age=3600")
        .body(include_str!("../css/dev.css"))
}
fn account_page() -> HttpResponse {
    let body = base_page(
        "Account".to_string(),
        "<h1>Change your password...</h1>".to_string(),
        &[],
    );
    HttpResponse::Ok().content_type("text/html").body(body)
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
    let body = base_page(
        "New Instruction".to_string(),
        form,
        &[HeaderButton::Close("/user/instructions")],
    );
    body
}

fn instruction_form() -> HttpResponse {
    let body = render_instruction_form(None); // None is no error info
    HttpResponse::Ok().content_type("text/html").body(body)
}

// INSTRUCTION PAGE
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
            let body = base_page(
                row.title,
                "(steps here)".to_string(),
                &[HeaderButton::Delete("/"), HeaderButton::Edit("/")],
            );
            HttpResponse::Ok().content_type("text/html").body(body)
        }
        Err(sqlx::Error::RowNotFound) => {
            let body = base_page(
                "Instruction not found".to_string(),
                "This instruction does not exist or has been deleted".to_string(),
                &[HeaderButton::Close("/user/instructions")],
            );
            // HTTP 410
            HttpResponse::Gone().content_type("text/html").body(body)
        }
        Err(msg) => {
            println!("error: {}", msg);
            let body = base_page(
                "Internal Server Error 500".to_string(),
                "We're sorry for the inconvenience".to_string(),
                &[HeaderButton::Close("/user/instructions")],
            );
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

    let body = base_page(
        "Instructions".to_string(),
        format!("<ul>{}</ul>", page_rows),
        &[HeaderButton::Create(CREATE_INSTRUCTION)],
    );

    HttpResponse::Ok()
        .header(http::header::CACHE_CONTROL, "no-store, must-revalidate")
        .content_type("text/html")
        .body(body)
}

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

    let images: String = steps
        .iter()
        .map(|s| format!(r#"<img src="/img/{}">"#, e(&s.filename)))
        .collect();

    let t = base_page("Images".to_string(), images, &[]);
    HttpResponse::Ok().body(t)
}
// A file server
// #[get("/img/*")]

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
                f.write_str(&button_base(
                    href,
                    "bg-gray-300 hover:bg-gray-400",
                    svg,
                ))
            }
            HeaderButton::Close(href) => {
                let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"> <line x1="18" y1="6" x2="6" y2="18"></line> <line x1="6" y1="6" x2="18" y2="18"></line> </svg> "#;
                f.write_str(&button_base(
                    href,
                    "bg-gray-300 hover:bg-gray-400",
                    svg,
                ))
            }
            HeaderButton::Edit(href) => {
                let svg = r#"<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"> <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"></path> <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"></path></svg>"#;
                f.write_str(&button_base(
                    href,
                    "bg-gray-300 hover:bg-gray-400",
                    svg,
                ))
            }
        }
    }
}

fn base_page(title: String, page_content: String, header_buttons: &[HeaderButton]) -> String {
    r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <meta http-equiv="Content-Type" content="text/html; charset=utf-8">


    <link rel="icon" href="data:image/svg+xml,%3Csvg%20xmlns='http://www.w3.org/2000/svg'%20viewBox='0%200%2016%2016'%3E%3Ctext%20x='0'%20y='14'%3E📖%3C/text%3E%3C/svg%3E" type="image/svg+xml" />
    <link href="data:image/x-icon;base64,8J+Msg==" rel="icon" type="image/x-icon" />
    <link href="/main.css" rel="stylesheet" />

    <title>"#.to_string() + &e(&title).to_string() + &r#" | Spruce</title>

    <style>
        :root {
            --inset-shadow: 0px 0px, rgb(255, 255, 255) 0px -2px inset, 0px 0px, 0px 0px;
        }
        nav a:hover {
            box-shadow: var(--inset-shadow);
        }
        .nav-link-underline {
            /* Avoid transitioning on page load */
            transition: none;
            box-shadow: var(--inset-shadow);
        }
        nav a {
            transition: box-shadow 0.15s ease;
            padding-top: 0.5rem;
            padding-bottom: 0.5rem;
        }
    </style>

    <script defer>
        window.addEventListener("load", () => {
            document.querySelectorAll(`a[href="${window.location.pathname}"]`).forEach((el) => {
            el.className += " nav-link-underline";
            });
        })
    </script> 
    
</head>

<body class="h-screen bg-gray-100 relative">
    <div class="shadow-md px-4 bg-green-700">
    <nav class="flex items-center max-w-screen-md mx-auto text-white">
        <a class="font-bold tracking-wide mr-auto" href="/">How I Do</a>
        <a class="mr-3" href="/user/instructions">Instructions</a>
        <a href="/account">You</a>
    </nav>
    </div>
    <div class="pt-2 px-4">
    <div class="max-w-screen-md mx-auto py-2">
        <div class="flex mb-2">
        <h1 class="text-2xl font-bold text-gray-900 mr-auto">
            "#.to_string() + &title + &r#"
        </h1>
            "#.to_string() + &header_buttons.iter().map(|button| button.to_string() + "\n").collect::<String>() + &r#"
        </div>
            "#.to_string() + &page_content + &r#"
        </div>
    </div>
    <!-- <footer class="absolute bottom-0 text-gray-600 w-full p-4 text-center"> <small>&copy; 2020 SpruceDoc</small> </footer> -->
</body>
</html>
"#
}
