use std::time::Duration;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Body, IntoUrl, Method,
};
use tokio::time::Instant;

pub async fn http_request<T, U>(
    method: reqwest::Method,
    uri: U,
    headers: HeaderMap,
    content_type: HeaderValue,
    body: T,
) -> Result<(reqwest::Response, Duration), Box<dyn std::error::Error>>
where
    U: IntoUrl,
    T: Into<Body> + Default,
{
    let client = reqwest::Client::new();
    let request = match method {
        Method::GET => client.get(uri),
        Method::POST => client.post(uri),
        Method::PUT => client.put(uri),
        Method::DELETE => client.delete(uri),
        Method::PATCH => client.patch(uri),
        Method::HEAD => client.head(uri),
        Method::OPTIONS => client.request(Method::OPTIONS, uri),
        _ => todo!(),
    };
    let request = request
        .body(body)
        .headers(headers)
        .header("content-type", content_type);
    let timer = Instant::now();
    let response = request.send().await?;
    Ok((response, timer.elapsed()))
}

#[cfg(test)]
mod tests {
    use mockito::{self, mock};
    use reqwest::header::{HeaderMap, HeaderValue};
    use tokio::test;

    use super::http_request;

    #[test]
    async fn get_ok() {
        let _mock = mock("GET", "/").create();
        let (resp, _time) = http_request(
            reqwest::Method::GET,
            mockito::server_url(),
            HeaderMap::new(),
            HeaderValue::from_str("").unwrap(),
            "{}",
        )
        .await
        .unwrap();
        assert_eq!(resp.status(), 200)
    }
}
