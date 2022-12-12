use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use tower_cookies::{Cookie, Cookies};

const COOKIE_NAME: &str = "visited";

pub fn bump_cookie_visited_count(cookies: Cookies) {
    let visited = cookies
        .get(COOKIE_NAME)
        .and_then(|c| c.value().parse().ok())
        .unwrap_or(0);
    cookies.add(Cookie::new(COOKIE_NAME, (visited + 1).to_string()));
}

const MIDDLEWARE_COOKIE: &str = "middleware";

pub fn update_middleware_cookie(cookies: Cookies) {
    let value = cookies
        .get(MIDDLEWARE_COOKIE)
        .and_then(|c| c.value().parse().ok())
        .unwrap_or(0);
    cookies.add(Cookie::new(MIDDLEWARE_COOKIE, (value + 10).to_string()));
}

pub async fn handle_cookies(
    cookies: Cookies,
    req: Request<Body>,
    next: Next<Body>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    update_middleware_cookie(cookies);
    let res = next.run(req).await;
    Ok(res)
}
