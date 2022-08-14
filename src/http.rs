use reqwest::{
    header::{HeaderMap, HeaderValue},
    Body, IntoUrl, Method,
};

pub async fn http_request<T, U>(
    method: reqwest::Method,
    uri: U,
    headers: HeaderMap,
    content_type: HeaderValue,
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
                .headers(headers)
                .header("content-type", content_type)
                .send()
                .await?
        }
        Method::POST => todo!(),
        Method::PUT => todo!(),
        Method::DELETE => todo!(),
        Method::PATCH => todo!(),
        Method::OPTIONS => todo!(),
        Method::HEAD => todo!(),
        _ => todo!(),
    };
    Ok(response)
}

// usage
// let response = http_request(reqwest::Method::GET,
//     "https://jsonplaceholder.typicode.com/todos/1",
//     HeaderMap::try_from(&empty).unwrap(), reqwest::header::HeaderValue::from_str("application/json").unwrap(),
//     "{}").await?;
// println!("{}", response.text().await?);= http_request(http::HttpMethod::GET, "https://jsonplaceholder.typicode.com/todos/1", "{}").unwrap();
