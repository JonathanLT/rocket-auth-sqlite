use auth_app::rocket;

#[rocket::main]
async fn main() {
    let _ = rocket().launch().await;
}