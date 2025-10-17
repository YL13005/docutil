#[macro_use] extern crate rocket;

use rocket::form::Form;
use rocket::fs::{FileServer, TempFile};
use rocket_dyn_templates::{Template, context};
use std::path::PathBuf;

static PASSWORD: &str = "monpassword"; // change ça !

#[derive(FromForm)]
struct UploadForm<'r> {
    password: &'r str,
    file: TempFile<'r>,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        error: Option::<String>::None,
        path: Option::<String>::None
    })
}

#[post("/upload", data = "<form>")]
async fn upload(mut form: Form<UploadForm<'_>>) -> Template {
    if form.password != PASSWORD {
        return Template::render("index", context! {
            error: Some("Mot de passe incorrect"),
            path: Option::<String>::None
        });
    }

    let save_path: PathBuf = PathBuf::from("uploads/uploaded_image.png");
    std::fs::create_dir_all("uploads").unwrap();

    if let Err(e) = form.file.persist_to(&save_path).await {
        return Template::render("index", context! {
            error: Some(format!("Erreur upload: {}", e)),
            path: Option::<String>::None
        });
    }

    Template::render("index", context! {
        error: Option::<String>::None,
        path: Some("/uploads/uploaded_image.png")
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, upload])
        .mount("/uploads", FileServer::from("uploads"))
        .attach(Template::fairing())
}