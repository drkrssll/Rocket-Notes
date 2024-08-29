#[macro_use]
extern crate rocket;

use rocket::{
    fs::{relative, FileServer, NamedFile},
    http::Status,
    response::status::Custom,
};

use std::path::{Path, PathBuf};

#[get("/<path..>")]
async fn index(path: PathBuf) -> Result<NamedFile, Custom<String>> {
    let mut path = Path::new(relative!("static")).join(path);
    if path.is_dir() {
        path.push("index.html")
    }

    NamedFile::open(path)
        .await
        .map_err(|_| Custom(Status::NotFound, String::from("404 Not Found")))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", FileServer::from(relative!("static")))
}
