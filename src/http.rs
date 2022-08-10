use reqwest::{Body, IntoUrl, Method};


pub async fn http_request<T, U>(
    method: reqwest::Method,
    uri: U,
    body: T,
) -> Result<reqwest::Response, Box<dyn std::error::Error>>
where
    U: IntoUrl,
    T: Into<Body> + Default,
{
    let client = reqwest::Client::new();
    let response = match method {
        Method::GET => {
            client
                .get(uri)
                .body(body)
                .header("Content-Type", "application/json")
                .send()
                .await?
        }
        Method::POST => todo!(),
        Method::PUT => todo!(),
        Method::DELETE => todo!(),
    };
    Ok(response)
}

// usage
// let response = http_request(http::HttpMethod::GET, "https://jsonplaceholder.typicode.com/todos/1", "{}").unwrap();
