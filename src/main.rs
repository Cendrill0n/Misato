use std::collections::HashMap;

use rocket::{*, fs::FileServer};
use rocket_dyn_templates::Template;


#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("default", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", FileServer::from("public/"))
        .attach(Template::fairing())
}
