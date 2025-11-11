use rocket_dyn_templates::{Template, context};

#[get("/")]
pub fn index() -> Template {
    let context = context! {
        title: "Hello, world!",
        message: "Hello, world!"
    };

    Template::render("index", context)
}