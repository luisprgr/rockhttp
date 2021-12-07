#[macro_use]
extern crate rocket;
use rocket_dyn_templates::Template;

mod controller;

/// # HTTP file server written in Rocket
/// Allow downloading and uploading files to a configured folder using the http protocol

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                controller::index,
                controller::files_controller,
                controller::assets,
                controller::delete_files_controller,
                controller::upload_file,
            ],
        )
        .attach(Template::fairing())
}
