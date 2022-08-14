use std::collections::HashMap;

use rocket::{fs::FileServer, *};
use rocket_dyn_templates::Template;

use misato_database::database::Database;
use misato_utils::settings::Settings;

mod session;

#[get("/")]
async fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("default", &context)
}

#[launch]
fn rocket() -> _ {
    let settings = Settings::init();
    let database = Database::init(&settings);
    let mut routes: Vec<Route> = Vec::new();

    routes.append(&mut routes![index]);

    routes.append(&mut routes![
        session::signup,
        session::login,
        session::session,
        session::signup_request,
        session::profile
    ]);

    rocket::build()
        .mount("/", routes)
        .mount("/", FileServer::from("public/"))
        .manage(settings)
        .manage(database)
        .attach(Template::fairing())
}
