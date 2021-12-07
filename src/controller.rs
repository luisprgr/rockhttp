use std::fs::remove_file;
use std::path::{Path, PathBuf};
use rocket::form::Form;
use rocket::fs::{NamedFile, TempFile};
use rocket::serde::Serialize;
use rocket_dyn_templates::Template;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct FileDescription {
    name: String,
    date: String,
    size: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct FileServerContext {
    files: Vec<FileDescription>,
    sum: f64,
    sum_string: String,
    error: String,
}

#[get("/")]
pub async fn index() -> Template {
    let directory = Path::new("files");
    let mut context = FileServerContext {
        files: Vec::new(),
        sum: 0.,
        sum_string: String::new(),
        error: String::from(""),
    };

    match directory.read_dir() {
        Ok(dir_iter) => {
            for file in dir_iter {
                match file {
                    Ok(file) => match file.metadata() {
                        Ok(metadata) => {
                            if metadata.is_file() {
                                let name_temp = file.file_name().to_string_lossy().into_owned();
                                let date_temp = match metadata.modified() {
                                    Ok(date) => format!("{:?}", date),
                                    Err(_) => String::from("no date found"),
                                };
                                context.sum += metadata.len() as f64;
                                let mut size_processed = (metadata.len() as f64) / 1024.;
                                let mut size_sub = String::from("Kb");
                                if size_processed > 1000. {
                                    size_processed = size_processed / 1024.;
                                    size_sub = String::from("Mb");
                                }
                                if size_processed > 1000. {
                                    size_processed = size_processed / 1024.;
                                    size_sub = String::from("Gb");
                                }
                                let size_temp = format!("{:.2} {}", size_processed, size_sub);
                                context.files.push(FileDescription {
                                    name: name_temp,
                                    date: date_temp,
                                    size: size_temp,
                                });
                            }
                        }
                        Err(e) => context.error = e.to_string(),
                    },
                    Err(e) => context.error = e.to_string(),
                }
            }
        }
        Err(e) => {
            context.error = e.to_string();
        }
    }
    if context.sum > 0. {
        let mut size_processed = context.sum / 1024.;
        let mut size_sub = String::from("Kb");
        if size_processed > 1000. {
            size_processed = size_processed / 1024.;
            size_sub = String::from("Mb");
        }
        if size_processed > 1000. {
            size_processed = size_processed / 1024.;
            size_sub = String::from("Gb");
        }
        context.sum_string = format!("{:.2} {}", size_processed, size_sub);
    }

    Template::render("index", context)
}

#[get("/files/<file_name..>")]
pub async fn files_controller(file_name: PathBuf) -> Option<NamedFile> {
    let directory = Path::new("files").join(file_name);
    if directory.is_dir() {
        return None;
    } else {
        NamedFile::open(directory).await.ok()
    }
}

#[get("/assets/<file_name..>")]
pub async fn assets(file_name: PathBuf) -> Option<NamedFile> {
    let directory = Path::new("assets").join(file_name);
    if directory.is_dir() {
        return None;
    } else {
        NamedFile::open(directory).await.ok()
    }
}

#[get("/delete/<file_name..>")]
pub async fn delete_files_controller(file_name: PathBuf) {
    let directory = Path::new("files").join(file_name);
    match remove_file(directory) {
        Ok(_) => {}
        Err(_) => {}
    }
}

#[derive(FromForm)]
pub struct UploadS<'t> {
    file: TempFile<'t>,
}

#[post("/upload", data = "<form_file>")]
pub async fn upload_file(mut form_file: Form<UploadS<'_>>) {
    if form_file.file.len() > 0 {
        match form_file.file.raw_name() {
            Some(name) => {
                let mut name_secure = String::from(name.as_str().unwrap());
                //checking the insecure raw file name
                if name
                    .dangerous_unsafe_unsanitized_raw()
                    .as_str()
                    .contains(".mkv")
                {
                    //the file name using the secure file name plus the extension
                    name_secure = name_secure + ".mkv";
                } else if name
                    .dangerous_unsafe_unsanitized_raw()
                    .as_str()
                    .contains(".mp4")
                {
                    name_secure = name_secure + ".mp4";
                } else if name
                    .dangerous_unsafe_unsanitized_raw()
                    .as_str()
                    .contains(".jpeg")
                {
                    name_secure = name_secure + ".jpeg";
                } else if name
                    .dangerous_unsafe_unsanitized_raw()
                    .as_str()
                    .contains(".jpg")
                {
                    name_secure = name_secure + ".jpg";
                } else if name
                    .dangerous_unsafe_unsanitized_raw()
                    .as_str()
                    .contains(".webp")
                {
                    name_secure = name_secure + ".webp";
                } else if name
                    .dangerous_unsafe_unsanitized_raw()
                    .as_str()
                    .contains(".webm")
                {
                    name_secure = name_secure + ".webm";
                } else if name.dangerous_unsafe_unsanitized_raw().as_str().contains(".gif")
                {
                    name_secure = name_secure + ".gif";
                } else if name.dangerous_unsafe_unsanitized_raw().as_str().contains(".zip")
                {
                    name_secure = name_secure + ".zip";
                } else if name.dangerous_unsafe_unsanitized_raw().as_str().contains(".rar")
                {
                    name_secure = name_secure + ".rar";
                } else if name.dangerous_unsafe_unsanitized_raw().as_str().contains(".7z")
                {
                    name_secure = name_secure + ".7z";
                }

                let directory = Path::new("files").join(name_secure);
                match form_file.file.persist_to(directory).await {
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
            None => {}
        }
    }
}
