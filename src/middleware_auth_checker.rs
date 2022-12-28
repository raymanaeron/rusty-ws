use tide::{utils::async_trait, Middleware, Next, Request};
pub struct CheckAuthorization;

// This is a middleware function that checks for authorization
#[async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for CheckAuthorization {
    async fn handle(
        &self,
        req: Request<State>,
        next: Next<'_, State>,
    ) -> Result<tide::Response, tide::Error> {
        let authorization_header = req.header("Authorization");
        if let Some(auth_header) = authorization_header {
            println!("Authorization header value: {}", auth_header);
        } else {
            println!("Authorization header not found");
        }

        // Return the result of the next handler
        Ok(next.run(req).await)
    }
}