use actix_files::NamedFile;
use actix_web::{get, Result, HttpRequest};
use actix_web::error::ErrorNotFound;
use std::path::PathBuf;

#[get("/{filename:.*[.]css}")]
async fn styles(req: HttpRequest) -> Result<NamedFile> {
    let mut full_path: PathBuf = PathBuf::from("./styles");
    let path: PathBuf = req.match_info().query("filename").parse().unwrap();
    full_path.push(path);
    
    if full_path.exists() {
        let file: NamedFile = NamedFile::open(full_path)?;

        return Ok(file.use_last_modified(false));
    }

    Err(ErrorNotFound("file not found"))
}
