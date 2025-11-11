use rocket::response::Redirect;
use rocket_dyn_templates::{Template, context};

use crate::routes::auth::AuthUser;

#[get("/dashboard")]
pub fn dashboard(user: AuthUser) -> Template {
    Template::render("dashboard", context! {
        title: "Dashboard",
        message: "Welcome to your dashboard",
        user: user.0
    })
}

#[get("/dashboard", rank = 2)]
pub fn dashboard_unauthenticated() -> Redirect {
    Redirect::to(uri!(crate::routes::index::index))
}