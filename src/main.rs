use std::num::ParseIntError;

use rocket::{get, http::Status, request::FromSegments, routes};

struct PathNumberSegments(Vec<i32>);

impl<'r> FromSegments<'r> for PathNumberSegments {
    type Error = ParseIntError;

    fn from_segments(
        segments: rocket::http::uri::Segments<'r, rocket::http::uri::fmt::Path>,
    ) -> Result<Self, Self::Error> {
        Ok(PathNumberSegments(
            segments
                .into_iter()
                .map(|num| num.parse())
                .collect::<Result<Vec<i32>, Self::Error>>()?,
        ))
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/-1/error")]
fn error() -> Status {
    Status::InternalServerError
}

#[get("/1/<numbers..>")]
fn day1(numbers: PathNumberSegments) -> String {
    numbers
        .0
        .into_iter()
        .reduce(|acc, el| acc ^ el)
        .unwrap_or_default()
        .pow(3)
        .to_string()
}
#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index, error, day1]);

    Ok(rocket.into())
}
