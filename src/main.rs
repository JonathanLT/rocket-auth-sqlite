#[macro_use] extern crate rocket;

use rocket_dyn_templates::Template;

mod db;
mod routes;

#[launch]
fn rocket() -> _ {
    db::init_db().expect("Failed to initialize database");

    rocket::build()
        .mount("/", routes![
            routes::index::index,
            routes::dashboard::dashboard,
            routes::dashboard::dashboard_unauthenticated,
            routes::auth::login_submit,
            routes::auth::logout,
            routes::auth::register_page,
            routes::auth::register_submit
        ])
        .attach(Template::fairing())
}