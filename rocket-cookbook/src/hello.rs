

pub mod hello {
    #[get("/greet")]
    pub fn greet() -> &'static str {
        "Hello, world!"
    }

    #[get("/greet/<name>")]
    pub fn greet_custom(name: &rocket::http::RawStr) -> String {
        format!("Hello, {}!", name.as_str())
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use rocket::local::Client;

    #[test]
    fn test_hello_world() {
        // Construct a client to use for dispatching requests.
        let rocket = rocket::ignite().mount("/", routes![hello::greet]);
        let client = Client::new(rocket).expect("valid rocket instance");

        // Dispatch a request to 'GET /' and validate the response.
        let mut response = client.get("/greet").dispatch();
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }
    #[test]
    fn test_custom_greet() {
        // Construct a client to use for dispatching requests.
        let rocket = rocket::ignite().mount("/", routes![hello::greet_custom]);
        let client = Client::new(rocket).expect("valid rocket instance");

        // Dispatch a request to 'GET /' and validate the response.
        let mut response = client.get("/greet/paul").dispatch();
        assert_eq!(response.body_string(), Some("Hello, paul!".into()));
    }
}