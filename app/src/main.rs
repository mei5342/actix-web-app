use actix_web::{post, web, Either};
use serde::Deserialize;

#[derive(Deserialize)]
struct Info {
    name: String,
}

#[post("/")]
async fn index(form: Either<web::Json<Info>, web::Form<Info>>) -> String {
    let name: String = match form {
        Either::Left(json) => json.name.to_owned(),
        Either::Right(form) => form.name.to_owned(),
    };

    format!("Welcome {}!", name)
}