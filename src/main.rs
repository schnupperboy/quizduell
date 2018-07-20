extern crate hyper;
extern crate diesel;
extern crate serde;
extern crate serde_json;
extern crate quizduell;
extern crate futures;

use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::rt::Future;
use hyper::service::service_fn;
use diesel::prelude::*;
use quizduell::establish_connection;
use quizduell::models::Question;
use futures::future;

fn get_questions() -> String {
    use quizduell::schema::questions::dsl::*;

    let connection = establish_connection();
    let results = questions
        .limit(5)
        .load::<Question>(&connection)
        .expect("Error loading questions");

    serde_json::to_string(&results).unwrap()
}


// Just a simple type alias
type BoxFut = Box<Future<Item=Response<Body>, Error=hyper::Error> + Send>;

fn quizduell(req: Request<Body>) -> BoxFut {
    let mut response = Response::new(Body::empty());

    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            *response.body_mut() = Body::from("Try GETting data from /questions");
        },
        (&Method::GET, "/questions") => {
            *response.body_mut() = Body::from(get_questions());
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        },
    };

    Box::new(future::ok(response))
}


fn main() {
    // This is our socket address...
    let addr = ([127, 0, 0, 1], 3000).into();

    // A `Service` is needed for every connection, so this
    // creates on of our `hello_world` function.
    let get_questions_svc = || {
        // service_fn_ok converts our function into a `Service`
        service_fn(quizduell)
    };

    let server = Server::bind(&addr)
        .serve(get_questions_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    // Run this server for... forever!
    hyper::rt::run(server);
}
