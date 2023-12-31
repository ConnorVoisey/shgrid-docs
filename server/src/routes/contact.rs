use super::AppState;
use crate::{
    models::{contact::Contact, IndexQueryable},
    query_params::{StdQueryParams, StdQueryParamsPreSerialize},
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;
use std::sync::Arc;

pub async fn index_contacts(
    State(state): State<Arc<AppState>>,
    Query(query): Query<StdQueryParamsPreSerialize>,
) -> impl IntoResponse {
    let query_res = StdQueryParams::from(query);
    let query = match query_res {
        Ok(val) => val,
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
    };
    let contacts = Contact::index(state.contacts.clone(), query);
    let val = json!({"count": contacts.len(), "data": contacts});
    Json(val).into_response()
}
