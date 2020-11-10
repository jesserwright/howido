use actix_web::{get, http, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;

use validator::{Validate, ValidationError};

// TODO: add this, and use it. It'll automatically set content type for HTML responses.
// askama_actix

#[get("/main.css")]
async fn css() -> impl Responder {
    // TODO: cache bust with etag (or something) on rebuild.
    // Very important for software update!!!! Or else user will get broken stylesheet!
    HttpResponse::Ok()
        .content_type("text/css")
        .header(http::header::CACHE_CONTROL, "public, max-age=3600")
        .body(include_str!("../css/dev.css"))
}

#[derive(Deserialize, Validate)]
struct InstructionCreateFormData {
    #[validate(
        length(max = 80, message = "Too long. Max 80 characters"),
        length(min = 1, message = "Too short. Min 1 character"),
        custom(function = "leading_whitespace", message = "Leading whitespace"),
        custom(function = "trailing_whitespace", message = "Trailing whitespace")
    )]
    title: String,
    #[validate(
        length(max = 80, message = "Too long. Max 80 characters"),
        length(min = 1, message = "Too short. Min 1 character"),
        custom(function = "leading_whitespace", message = "Leading whitespace"),
        custom(function = "trailing_whitespace", message = "Trailing whitespace")
    )]
    titleb: String,
}

fn trailing_whitespace(s: &str) -> Result<(), ValidationError> {
    if s.trim_end().len() != s.len() {
        return Err(ValidationError::new("trailing_whitespace"));
    }
    Ok(())
}
fn leading_whitespace(s: &str) -> Result<(), ValidationError> {
    if s.trim_start().len() != s.len() {
        return Err(ValidationError::new("leading_whitespace"));
    }
    Ok(())
}

#[get("/error")]
async fn error_route() -> impl Responder {
    return "Errors";
}

// The problem now, is that the modal does not pop-up again!

#[post("/create-instruction")]
async fn create_instruction(
    app_data: web::Data<AppState>,
    form: web::Form<InstructionCreateFormData>,
) -> impl Responder {
    let mut data = app_data.data.lock().unwrap();

    if let Err(err) = form.validate() {
        let mut renderable_errors = HashMap::new();
        for e in err.errors() {
            let (field, val_error) = e;
            println!("\nError on {}. ", field);

            let mut field_error_messages = vec![];
            if let validator::ValidationErrorsKind::Field(field_errs) = val_error {
                for field_err in field_errs {
                    if let Some(msg) = &field_err.message {
                        println!("{}", msg);
                        let m: &str = &msg;
                        field_error_messages.push(m);
                    }
                }
            }
            renderable_errors.insert(field.clone(), field_error_messages);
        }
        let resp = render_instructions(data.clone(), Some(renderable_errors));
        return HttpResponse::UnprocessableEntity()
            // DO NOT store previous form errors
            .header(http::header::CACHE_CONTROL, "no-store")
            .content_type("text/html")
            .body(resp);
    }

    // In case of error, redirect to the modal page, but rendered
    // with an error in the modal body next to the field.
    // This page will only render like this ONCE. If relaoded, it will
    // show the usual modal w/o the error.
    // User must modify input before doing anything else.
    // User should not be able to navigate back to a bad form input?
    // Don't cache these requests?

    // In case of success, redirect to /instruction/id (which MUST EXIST
    // after code in here has ran (db call))

    // TODO: replace this with a database call.
    data.push(Instruction {
        id: 1,
        title: form.title.to_string(),
    });

    HttpResponse::Found() // this is the 3xx status code - redirect
        .content_type("text/html")
        .header(http::header::LOCATION, "/user/instructions")
        .body("Instruction succesfully created. Redirecting to instructions page.")
}

#[derive(Debug, Clone)]
struct Instruction {
    id: i32,
    title: String,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.title)
    }
}

#[derive(Template)]
#[template(path = "instructions.html")]
struct InstructionsTemplate<'a> {
    user: String,
    instructions: Vec<Instruction>,
    errors: Option<HashMap<&'a str, Vec<&'a str>>>,
}

use std::sync::Mutex;

#[get("/user/instructions")]
async fn instructions_page(app_data: web::Data<AppState>) -> impl Responder {
    let data = app_data.data.lock().unwrap();
    let resp = render_instructions(data.clone(), None);

    HttpResponse::Ok().content_type("text/html").body(resp)
}

// Render function that is available for both post(after post, re-render) and get requests
fn render_instructions(
    instructions: Vec<Instruction>,
    errors: Option<HashMap<&str, Vec<&str>>>,
) -> String {
    // errs2.insert("title", vec!["Trailing white space"]);
    // errs2.insert("titleb", vec!["Too long. Max 1 character."]);

    // TODO [in template]: map the errors to fields, conditionally.

    let template = InstructionsTemplate {
        user: "Jesse".to_string(),
        instructions,
        errors,
    };
    let resp = format!(" {}", template.render().unwrap());
    resp
}

#[derive(Debug)]
struct AppState {
    data: Mutex<Vec<Instruction>>,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexPage<'a> {
    name: &'a str,
}

#[get("/")]
async fn index() -> impl Responder {
    let template = IndexPage { name: "Jesse" };
    // WHY is there a space in the format string?
    let resp = template.render().unwrap();
    // Why is an unwrap needed here if the template is known at compile time? Or is it?
    // What could go wrong?

    HttpResponse::Ok().content_type("text/html").body(resp)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let data = AppState {
        data: Mutex::new(vec![
            Instruction {
                id: 1,
                title: "My way of making eggs 🥚, and other crazy things that are fun.".to_string(),
            },
            Instruction {
                id: 2,
                title: "Chicken noodle soup".to_string(),
            },
            Instruction {
                id: 3,
                title: "Chicken chores".to_string(),
            },
        ]),
    };

    let app_data = web::Data::new(data);
    // What is a closure?
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::new("%{User-Agent}i"))
            .wrap(middleware::Compress::new(
                http::header::ContentEncoding::Gzip,
            ))
            .app_data(app_data.clone()) // the clone takes a refrence for each (thread?)
            .service(index)
            .service(instructions_page)
            .service(create_instruction)
            .service(error_route)
            .service(css)
    })
    .bind("localhost:8080")?
    .run()
    .await
}
