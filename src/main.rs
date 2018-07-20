extern crate hyper;
extern crate diesel;
extern crate serde;
extern crate serde_json;
extern crate quizduell;

use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;
use diesel::prelude::*;
use quizduell::establish_connection;
use quizduell::models::Question;

fn get_questions(_req: Request<Body>) -> Response<Body> {
    use quizduell::schema::questions::dsl::*;

    let connection = establish_connection();
    let results = questions
        .limit(5)
        .load::<Question>(&connection)
        .expect("Error loading questions");

    Response::new(Body::from(serde_json::to_string(&results).unwrap()))
}

fn main() {
    // This is our socket address...
    let addr = ([127, 0, 0, 1], 3000).into();

    // A `Service` is needed for every connection, so this
    // creates on of our `hello_world` function.
    let get_questions_svc = || {
        // service_fn_ok converts our function into a `Service`
        service_fn_ok(get_questions)
    };

    let server = Server::bind(&addr)
        .serve(get_questions_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    // Run this server for... forever!
    hyper::rt::run(server);
}
