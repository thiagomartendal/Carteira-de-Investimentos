use axum::http::StatusCode;
use axum::{response::Html, response::Response};
use axum::response::{IntoResponse, Redirect};
use axum_extra::extract::cookie::PrivateCookieJar;

pub enum StateResponse {
    RenderObject {html: String, status: StatusCode},
    Redirect {route: String, status: StatusCode},
    RedirectWithCookie {jar_cookie: PrivateCookieJar, route: String, status: StatusCode}
}

impl IntoResponse for StateResponse {
    fn into_response(self) -> Response {
        match self {
            StateResponse::RenderObject {html, status} => {
                (status, Html(html)).into_response()
            }

            StateResponse::Redirect {route, status} => {
                (status, Redirect::to(route.as_str())).into_response()
            }

            StateResponse::RedirectWithCookie {jar_cookie, route, status} => {
                let mut response = Redirect::to(route.as_str()).into_response();
                *response.status_mut() = status;

                (jar_cookie, response).into_response()
            }
        }
    }
}
