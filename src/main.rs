use tide::Request;

mod middleware_auth_checker;
mod person;
mod query_parameters;

#[async_std::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    // Create a new Tide server
    let mut app = tide::Server::new();

    // If we want to call the middleware function in every route
    // app.with(CheckAuthorization);

    // Only seach route will call the middleware
    app.at("/search")
        .with(middleware_auth_checker::CheckAuthorization)
        .get(get_fn_with_query_param);
    app.at("/submit").post(post_fn_with_json_body);
    app.at("/update").put(put_fn_with_json_body);

    // This function handles HTTP GET with a query parameter
    async fn get_fn_with_query_param(req: Request<()>) -> tide::Result {
        let query_params: query_parameters::QueryParams = req.query().unwrap();
        Ok(format!("Received GET request with query parameter q = {:?}", query_params.q).into())
    }

    // This function handles HTTP POST with a JSON body
    async fn post_fn_with_json_body(mut req: Request<()>) -> tide::Result {
        let body: person::Person = req.body_json().await?;
        Ok(format!(
            "Received POST request with name = {}, age = {}",
            body.name, body.age
        )
        .into())
    }

    // This function handles HTTP PUT with a JSON body
    async fn put_fn_with_json_body(mut req: Request<()>) -> tide::Result {
        let body: person::Person = req.body_json().await?;
        Ok(format!(
            "Received PUT request with name = {}, age = {}",
            body.name, body.age
        )
        .into())
    }

    // Example code -- checks header for a specific key
    async fn check_token(req: Request<()>) {
        let authorization_header = req.header("Authorization");
        if let Some(auth_header) = authorization_header {
            println!("Authorization header value: {}", auth_header);
        } else {
            println!("Authorization header not found");
        }
    }

    // Start the server and listen for incoming requests
    let port = 8080;
    println!("Server is listening on port:{}. Press CTRL+C to exit.", port);
    app.listen(format!("127.0.0.1:{}",port)).await?;

    Ok(())
}