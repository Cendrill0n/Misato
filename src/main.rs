use std::collections::HashMap;

use rocket::{fs::FileServer, *};
use rocket_dyn_templates::Template;

use yui_database::database::Database;
use yui_utils::settings::Settings;

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("default", &context)
}

#[launch]
fn rocket() -> _ {
    let settings = Settings::init();
    let database = Database::init(&settings);
    rocket::build()
        .mount("/", routes![index])
        .mount("/", FileServer::from("public/"))
        .manage(settings)
        .manage(database)
        .attach(Template::fairing())
}
