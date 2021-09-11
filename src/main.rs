#[macro_use]
extern crate nickel;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate nickel_app;
extern crate nickel_diesel;

mod models;
mod schema;

use diesel::prelude::*;
use models::*;
use nickel::{HttpRouter, JsonBody, MediaType, Nickel};
use nickel_app::{establish_connection, get_connection, Conn, ServerData};
use schema::posts;
use schema::posts::dsl::*;
use std::sync::Mutex;

fn main() {
    let connection = Mutex::new(Conn(establish_connection()));
    let mut server = Nickel::with_data(ServerData { connection });

    server.get(
        "/",
        middleware! { |req, mut res|
            let connection = get_connection!(&req);
            let results = posts.load::<Post>(connection).expect("Error loading posts");

            res.set(MediaType::Json);
            format!("{}", serde_json::to_string(&results).unwrap())
        },
    );

    server.post(
        "/",
        middleware!( |req|
            let connection = get_connection!(req);
            let data = req.json_as::<NewPost>().unwrap();

            let result = diesel::insert_into(posts::table)
                .values(&data)
                .get_result::<Post>(connection)
                .expect("Error saving new post");

            format!("{}", serde_json::to_string(&result).unwrap())
        ),
    );

    server.listen("127.0.0.1:3000").unwrap();
}
