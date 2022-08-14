use std::collections::HashMap;

use rocket::form::{Form, FromForm};
use rocket::response::Redirect;
use rocket::*;
use rocket_dyn_templates::Template;

use misato_database::{database::*, models::user_model::*};
use misato_security::password::*;

#[derive(Debug, FromForm)]
pub struct AccountSignup<'v> {
    username: &'v str,
    password: &'v str,
}

#[derive(Debug, FromForm)]
pub struct AccountLogin<'v> {
    username: &'v str,
    password: &'v str,
    remember: &'v str,
}

#[post("/session", data = "<form>")]
pub async fn session<'r>(db: &State<Database>, form: Form<AccountLogin<'_>>) -> Redirect {
    println!(
        "Username = {} | Password = {} | remember = {}",
        form.username, form.password, form.remember
    );
    let user = db
        .usermanager
        .get_user(Some(form.username.to_string()), None);
    if user.is_err() || (user.is_ok() && user.clone().unwrap().is_none()) {
        println!("no user found or error.");
        return Redirect::to("/login");
    }

    let user = user.unwrap().unwrap();
    if !user.password.is_correct_password(form.password.as_bytes()) {
        println!("password incorrect.");
        return Redirect::to("/login");
    }

    Redirect::to("/profile")
}

#[post("/signup", data = "<form>")]
pub async fn signup_request<'r>(db: &State<Database>, form: Form<AccountSignup<'_>>) -> Redirect {
    println!(
        "Username = {} | Password = {}",
        form.username, form.password
    );

    let user = db
        .usermanager
        .get_user(Some(form.username.to_string()), None);
    if user.is_ok() && user.clone().unwrap().is_some() {
        println!("account already exists, please login instead or change your username.");
        return Redirect::to("/signup");
    }

    let user = User::create(
        form.username.to_string(),
        Password::hash_password(form.password.as_bytes()),
        None,
    );

    let result = db.usermanager.create_user(&user);

    if result.is_err() {
        println!("could not create account server error.");
        return Redirect::to("/signup");
    }

    Redirect::to("/profile")
}

#[get("/signup")]
pub async fn signup() -> Template {
    Template::render("signup", &HashMap::<String, String>::new())
}

#[get("/login")]
pub async fn login() -> Template {
    Template::render("login", &HashMap::<String, String>::new())
}

#[get("/profile")]
pub async fn profile() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("profile", &context)
}
