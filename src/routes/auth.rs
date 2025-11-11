use rocket::form::Form;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};

use crate::db;

// Auth guard using a private cookie.
pub struct AuthUser(pub String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthUser {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let jar = req.cookies();
        match jar.get_private("user_id") {
            Some(cookie) => Outcome::Success(AuthUser(cookie.value().to_string())),
            None => Outcome::Forward(Status::Unauthorized),
        }
    }
}

#[derive(FromForm)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

#[derive(FromForm)]
pub struct RegisterData {
    pub username: String,
    pub password: String,
}

// POST /login: validate credentials.
#[post("/login", data = "<form_data>")]
pub fn login_submit(jar: &CookieJar<'_>, form_data: Form<LoginData>) -> Redirect {
    let data = form_data.into_inner();
    match db::validate_user(&data.username, &data.password) {
        Ok(true) => {
            jar.add_private(Cookie::new("user_id", data.username));
            Redirect::to(uri!(crate::routes::dashboard::dashboard))
        },
        _ => Redirect::to(uri!(crate::routes::index::index)),
    }
}

// GET /register
#[get("/register")]
pub fn register_page() -> Template {
    Template::render("register", context! { title: "Register" })
}

// POST /register: create new user.
#[post("/register", data = "<form_data>")]
pub fn register_submit(form_data: Form<RegisterData>) -> Redirect {
    let data = form_data.into_inner();
    match db::create_user(&data.username, &data.password) {
        Ok(_) => Redirect::to(uri!(crate::routes::index::index)),
        Err(_) => Redirect::to(uri!(crate::routes::auth::register_page)),
    }
}

// GET /logout
#[get("/logout")]
pub fn logout(jar: &CookieJar<'_>) -> Redirect {
    jar.remove_private(Cookie::from("user_id"));
    Redirect::to(uri!(crate::routes::index::index))
}