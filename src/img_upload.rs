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

// TODO: A file server
// #[get("/img/*")]