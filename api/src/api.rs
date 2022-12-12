use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use tower_cookies::Cookies;

use core::{Mutation as MutationCore, Query as QueryCore};
use entity::message;

use crate::cookies::bump_cookie_visited_count;
use crate::AppState;

#[derive(Deserialize)]
pub struct Params {
    page: Option<u64>,
    messages_per_page: Option<u64>,
}

#[derive(Serialize)]
pub struct MessageList {
    messages: Vec<message::Model>,
    num_pages: u64,
}

pub async fn list_messages(
    state: State<AppState>,
    Query(params): Query<Params>,
    cookies: Cookies,
) -> Result<Json<MessageList>, (StatusCode, &'static str)> {
    let page = params.page.unwrap_or(1);
    let messages_per_page = params.messages_per_page.unwrap_or(5);

    let (messages, num_pages) =
        QueryCore::find_messages_in_page(&state.conn, page, messages_per_page)
            .await
            .expect("Cannot find messages in page");

    bump_cookie_visited_count(cookies);
    Ok(Json(MessageList {
        messages,
        num_pages,
    }))
}

pub async fn create_message(
    state: State<AppState>,
    cookies: Cookies,
    body: Json<message::CreateMessage>,
) -> Result<Json<message::Model>, (StatusCode, &'static str)> {
    let msg = MutationCore::create_message(&state.conn, body.0)
        .await
        .expect("could not insert message");

    let msg = msg.into();

    bump_cookie_visited_count(cookies);
    Ok(Json(msg))
}

pub async fn get_message(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<message::Model>, (StatusCode, &'static str)> {
    let message = QueryCore::find_message_by_id(&state.conn, id)
        .await
        .expect("could not find message")
        .unwrap_or_else(|| panic!("could not find message with id {}", id));

    Ok(Json(message))
}

pub async fn update_message(
    state: State<AppState>,
    Path(id): Path<i32>,
    cookies: Cookies,
    body: Json<message::UpdateMessage>,
) -> Result<Json<message::Model>, (StatusCode, String)> {
    let body = body.0;
    let msg = MutationCore::update_message_by_id(&state.conn, id, body)
        .await
        .expect("could not edit message");
    bump_cookie_visited_count(cookies);
    Ok(Json(msg))
}

#[derive(Serialize)]
pub struct DeleteMessage {
    rows_deleted: u64,
}

pub async fn delete_message(
    state: State<AppState>,
    Path(id): Path<i32>,
    cookies: Cookies,
) -> Result<Json<DeleteMessage>, (StatusCode, &'static str)> {
    let rows_deleted = MutationCore::delete_message(&state.conn, id)
        .await
        .expect("could not delete message")
        .rows_affected;
    bump_cookie_visited_count(cookies);
    Ok(Json(DeleteMessage { rows_deleted }))
}
